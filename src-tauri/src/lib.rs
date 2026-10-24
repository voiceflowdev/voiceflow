use std::sync::atomic::{AtomicIsize, Ordering};
use std::sync::RwLock;
use std::thread;
use std::time::Duration;
use tauri::{AppHandle, Emitter, Manager};
use tauri_plugin_global_shortcut::GlobalShortcutExt;
use tauri_plugin_updater::UpdaterExt;

static LAST_TARGET_HWND: AtomicIsize = AtomicIsize::new(0);
static VOICEFLOW_HWND: AtomicIsize = AtomicIsize::new(0);
static LAST_TARGET_TITLE: RwLock<String> = RwLock::new(String::new());

#[cfg(target_os = "windows")]
use windows_sys::Win32::Foundation::HWND;
#[cfg(target_os = "windows")]
use windows_sys::Win32::UI::Input::KeyboardAndMouse::{
    SendInput, INPUT, INPUT_0, INPUT_KEYBOARD, KEYBDINPUT, KEYEVENTF_KEYUP, KEYEVENTF_UNICODE,
    VK_BACK,
};
#[cfg(target_os = "windows")]
use windows_sys::Win32::UI::WindowsAndMessaging::{
    GetForegroundWindow, GetWindowTextW, SetForegroundWindow,
};

#[cfg(target_os = "windows")]
fn get_window_title(hwnd: HWND) -> String {
    let mut buffer = [0u16; 512];
    let len = unsafe { GetWindowTextW(hwnd, buffer.as_mut_ptr(), buffer.len() as i32) };
    if len > 0 {
        String::from_utf16_lossy(&buffer[..len as usize])
    } else {
        String::new()
    }
}

/// Start a lightweight background tracker for the user's active window
fn start_window_tracker(app: AppHandle) {
    thread::spawn(move || {
        loop {
            thread::sleep(Duration::from_millis(50));
            #[cfg(target_os = "windows")]
            {
                let fg = unsafe { GetForegroundWindow() };
                let my_hwnd = VOICEFLOW_HWND.load(Ordering::Relaxed) as HWND;
                if fg != 0 as _ && fg != my_hwnd {
                    let prev = LAST_TARGET_HWND.swap(fg as isize, Ordering::Relaxed);
                    if prev != fg as isize {
                        let title = get_window_title(fg);
                        if let Ok(mut lock) = LAST_TARGET_TITLE.write() {
                            *lock = title.clone();
                        }
                        let _ = app.emit("target-window-changed", title);
                    }
                }
            }
        }
    });
}

/// Types text directly into the active/target application via Windows SendInput with Unicode
#[tauri::command]
fn type_text(text: String, restore_target: bool) -> Result<(), String> {
    if text.is_empty() {
        return Ok(());
    }

    #[cfg(target_os = "windows")]
    {
        let target_hwnd = LAST_TARGET_HWND.load(Ordering::Relaxed) as HWND;
        let my_hwnd = VOICEFLOW_HWND.load(Ordering::Relaxed) as HWND;
        let fg = unsafe { GetForegroundWindow() };

        // If VoiceFlow has focus or restore is requested, switch to target app
        if (restore_target || fg == my_hwnd) && target_hwnd != 0 as _ && target_hwnd != my_hwnd {
            unsafe {
                SetForegroundWindow(target_hwnd);
            }
            thread::sleep(Duration::from_millis(10));
        }

        // Batch all unicode keyboard inputs together into a single atomic SendInput call!
        // This delivers instant, zero-delay real-time typing across all Windows apps.
        let mut inputs: Vec<INPUT> = Vec::with_capacity(text.encode_utf16().count() * 2);

        for ch in text.encode_utf16() {
            inputs.push(INPUT {
                r#type: INPUT_KEYBOARD,
                Anonymous: INPUT_0 {
                    ki: KEYBDINPUT {
                        wVk: 0,
                        wScan: ch,
                        dwFlags: KEYEVENTF_UNICODE,
                        time: 0,
                        dwExtraInfo: 0,
                    },
                },
            });
            inputs.push(INPUT {
                r#type: INPUT_KEYBOARD,
                Anonymous: INPUT_0 {
                    ki: KEYBDINPUT {
                        wVk: 0,
                        wScan: ch,
                        dwFlags: KEYEVENTF_UNICODE | KEYEVENTF_KEYUP,
                        time: 0,
                        dwExtraInfo: 0,
                    },
                },
            });
        }

        if !inputs.is_empty() {
            unsafe {
                SendInput(
                    inputs.len() as u32,
                    inputs.as_mut_ptr(),
                    std::mem::size_of::<INPUT>() as i32,
                );
            }
        }
        Ok(())
    }

    #[cfg(not(target_os = "windows"))]
    {
        Ok(())
    }
}

/// Alias for type_text to inject into active window
#[tauri::command]
fn inject_text(text: String) -> Result<(), String> {
    type_text(text, true)
}

/// Native window dragging command that works 100% reliably on Windows
#[tauri::command]
fn start_dragging(window: tauri::WebviewWindow) -> Result<(), String> {
    window.start_dragging().map_err(|e| e.to_string())
}

/// Deletes previous characters by simulating backspace keys
#[tauri::command]
fn backspace(count: usize) -> Result<(), String> {
    #[cfg(target_os = "windows")]
    {
        for _ in 0..count {
            let mut inputs = [
                INPUT {
                    r#type: INPUT_KEYBOARD,
                    Anonymous: INPUT_0 {
                        ki: KEYBDINPUT {
                            wVk: VK_BACK,
                            wScan: 0,
                            dwFlags: 0,
                            time: 0,
                            dwExtraInfo: 0,
                        },
                    },
                },
                INPUT {
                    r#type: INPUT_KEYBOARD,
                    Anonymous: INPUT_0 {
                        ki: KEYBDINPUT {
                            wVk: VK_BACK,
                            wScan: 0,
                            dwFlags: KEYEVENTF_KEYUP,
                            time: 0,
                            dwExtraInfo: 0,
                        },
                    },
                },
            ];

            unsafe {
                SendInput(
                    inputs.len() as u32,
                    inputs.as_mut_ptr(),
                    std::mem::size_of::<INPUT>() as i32,
                );
            }
            thread::sleep(Duration::from_millis(5));
        }
        Ok(())
    }

    #[cfg(not(target_os = "windows"))]
    {
        Ok(())
    }
}

/// Returns the title of the current active target application
#[tauri::command]
fn get_target_window_title() -> Result<String, String> {
    if let Ok(lock) = LAST_TARGET_TITLE.read() {
        Ok(lock.clone())
    } else {
        Ok(String::new())
    }
}

/// Dynamically switch window between Collapsed pill and Expanded mode
/// Supports both { mode: "collapsed" | "expanded" } and { expanded: true | false }
#[tauri::command]
fn set_window_mode(
    app: AppHandle,
    expanded: Option<bool>,
    mode: Option<String>,
) -> Result<(), String> {
    let is_expanded = expanded.unwrap_or_else(|| mode.as_deref() == Some("expanded"));

    if let Some(window) = app.get_webview_window("main") {
        let _ = window.set_resizable(true);
        if is_expanded {
            let _ = window.set_min_size(Some(tauri::LogicalSize::new(280.0, 420.0)));
            let _ = window.set_max_size(None::<tauri::LogicalSize<f64>>);
            let _ = window.set_size(tauri::LogicalSize::new(380.0, 710.0));
            let _ = window.set_always_on_top(true);
        } else {
            let _ = window.set_min_size(Some(tauri::LogicalSize::new(200.0, 52.0)));
            let _ = window.set_max_size(None::<tauri::LogicalSize<f64>>);
            let _ = window.set_size(tauri::LogicalSize::new(320.0, 70.0));
            let _ = window.set_always_on_top(true);
        }
    }
    Ok(())
}

/// Resize the window directly by delta (for custom resize grips)
#[tauri::command]
fn resize_window_by(app: AppHandle, delta_x: f64, delta_y: f64) -> Result<(), String> {
    if let Some(window) = app.get_webview_window("main") {
        if let Ok(size) = window.inner_size() {
            let scale = window.scale_factor().unwrap_or(1.0);
            let new_w = (size.width as f64 + delta_x * scale).max(200.0);
            let new_h = (size.height as f64 + delta_y * scale).max(52.0);
            let _ = window.set_size(tauri::PhysicalSize::new(new_w as u32, new_h as u32));
        }
    }
    Ok(())
}

// Saved size of the collapsed pill so it can be restored after dictation
static PILL_RESTORE_W: AtomicIsize = AtomicIsize::new(-1);
static PILL_RESTORE_H: AtomicIsize = AtomicIsize::new(-1);

/// Widen the collapsed pill while dictating so live transcription text fits
/// inside it. The previous size is remembered and only pill-sized windows
/// (short height) are touched — the expanded panel is never affected.
#[tauri::command]
fn begin_pill_dictation(app: AppHandle) -> Result<(), String> {
    if let Some(window) = app.get_webview_window("main") {
        if let Ok(size) = window.inner_size() {
            let scale = window.scale_factor().unwrap_or(1.0);
            let height_logical = size.height as f64 / scale;
            if height_logical > 160.0 {
                return Ok(()); // expanded panel — leave it alone
            }
            let target_w = (520.0 * scale) as u32;
            if size.width < target_w {
                PILL_RESTORE_W.store(size.width as isize, Ordering::Relaxed);
                PILL_RESTORE_H.store(size.height as isize, Ordering::Relaxed);
                let _ = window.set_size(tauri::PhysicalSize::new(target_w, size.height));
            } else {
                // pill is already wide enough — nothing was changed, so make
                // sure no stale restore size survives for end_pill_dictation
                PILL_RESTORE_W.store(-1, Ordering::Relaxed);
                PILL_RESTORE_H.store(-1, Ordering::Relaxed);
            }
        }
    }
    Ok(())
}

/// Restore the collapsed pill to its pre-dictation size
#[tauri::command]
fn end_pill_dictation(app: AppHandle) -> Result<(), String> {
    let w = PILL_RESTORE_W.swap(-1, Ordering::Relaxed);
    let h = PILL_RESTORE_H.swap(-1, Ordering::Relaxed);
    if w <= 0 || h <= 0 {
        return Ok(());
    }
    if let Some(window) = app.get_webview_window("main") {
        if let Ok(size) = window.inner_size() {
            let scale = window.scale_factor().unwrap_or(1.0);
            if (size.height as f64 / scale) <= 160.0 {
                // only restore while the window is still a collapsed pill
                let _ = window.set_size(tauri::PhysicalSize::new(w as u32, h as u32));
            }
        }
    }
    Ok(())
}


/// Toggle voice listening state
#[tauri::command]
fn toggle_listening(app: AppHandle, _config: Option<serde_json::Value>) -> Result<bool, String> {
    static IS_LISTENING: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
    let prev = IS_LISTENING.fetch_xor(true, Ordering::Relaxed);
    let current = !prev;
    let _ = app.emit("listening-state", current);
    Ok(current)
}

/// Configure start on system boot via Windows Registry Run key
#[tauri::command]
fn set_autostart(enabled: bool) -> Result<(), String> {
    #[cfg(target_os = "windows")]
    {
        if let Ok(exe) = std::env::current_exe() {
            let exe_str = exe.to_string_lossy().to_string();
            if enabled {
                let _ = std::process::Command::new("reg")
                    .args([
                        "add",
                        "HKCU\\Software\\Microsoft\\Windows\\CurrentVersion\\Run",
                        "/v",
                        "VoiceFlow",
                        "/t",
                        "REG_SZ",
                        "/d",
                        &format!("\"{}\" --tray", exe_str),
                        "/f",
                    ])
                    .output();
            } else {
                let _ = std::process::Command::new("reg")
                    .args([
                        "delete",
                        "HKCU\\Software\\Microsoft\\Windows\\CurrentVersion\\Run",
                        "/v",
                        "VoiceFlow",
                        "/f",
                    ])
                    .output();
            }
        }
    }
    Ok(())
}

/// Show or hide the floating pill window (used by Ctrl+Alt+V hotkey and tray)
fn toggle_pill_visibility(app: &AppHandle) {
    if let Some(window) = app.get_webview_window("main") {
        if window.is_visible().unwrap_or(false) {
            let _ = window.hide();
        } else {
            let _ = window.show();
            let _ = window.set_focus();
        }
    }
}

/// Change the registered global shortcut
#[tauri::command]
fn register_shortcut(app: AppHandle, shortcut_str: String) -> Result<(), String> {
    if let Ok(shortcut) = shortcut_str.parse::<tauri_plugin_global_shortcut::Shortcut>() {
        let _ = app.global_shortcut().unregister_all();
        app.global_shortcut()
            .register(shortcut)
            .map_err(|e| e.to_string())?;
        // Keep the pill show/hide hotkey alive alongside the mic hotkey
        if let Ok(vis) = "Control+Alt+KeyV".parse::<tauri_plugin_global_shortcut::Shortcut>() {
            let _ = app.global_shortcut().register(vis);
        }
        Ok(())
    } else {
        Err("Invalid shortcut string. Try e.g. 'Alt+KeyV' or 'Control+Shift+Space'".to_string())
    }
}

/// Minimize the window
#[tauri::command]
fn minimize_window(app: AppHandle) -> Result<(), String> {
    if let Some(window) = app.get_webview_window("main") {
        let _ = window.minimize();
    }
    Ok(())
}

static ENCODING_MODE: RwLock<String> = RwLock::new(String::new());

/// Set the Bengali typing encoding mode ("unicode" or "ansi")
#[tauri::command]
fn set_encoding_mode(app: AppHandle, mode: String) -> Result<(), String> {
    let mode_val = if mode.to_lowercase() == "ansi" {
        "ansi".to_string()
    } else {
        "unicode".to_string()
    };
    if let Ok(mut lock) = ENCODING_MODE.write() {
        *lock = mode_val.clone();
    }
    let _ = app.emit("encoding-mode-changed", mode_val);
    Ok(())
}

/// Get the current encoding mode
#[tauri::command]
fn get_encoding_mode() -> Result<String, String> {
    if let Ok(lock) = ENCODING_MODE.read() {
        if lock.is_empty() {
            Ok("unicode".to_string())
        } else {
            Ok(lock.clone())
        }
    } else {
        Ok("unicode".to_string())
    }
}

/// Close the application
#[tauri::command]
fn close_app(app: AppHandle) -> Result<(), String> {
    app.exit(0);
    Ok(())
}

/// Check GitHub releases for a newer VoiceFlow version
#[tauri::command]
async fn check_update(app: AppHandle) -> Result<Option<UpdateInfo>, String> {
    let updater = app.updater().map_err(|e| e.to_string())?;
    match updater.check().await.map_err(|e| e.to_string())? {
        Some(update) => Ok(Some(UpdateInfo {
            current_version: update.current_version.clone(),
            version: update.version.clone(),
            notes: update.body.clone().unwrap_or_default(),
        })),
        None => Ok(None),
    }
}

#[derive(serde::Serialize)]
struct UpdateInfo {
    current_version: String,
    version: String,
    notes: String,
}

/// Download + silently install the latest update, then restart the app.
/// Emits "update-download-progress" events ({ downloaded, total }) to the UI.
#[tauri::command]
async fn download_and_install_update(app: AppHandle) -> Result<(), String> {
    // Re-check (in case the user clicked a while after checking)
    let updater = app.updater().map_err(|e| e.to_string())?;
    let update = updater
        .check()
        .await
        .map_err(|e| e.to_string())?
        .ok_or_else(|| "No update available".to_string())?;

    let emitter = app.clone();
    let finished = app.clone();
    let emitter_failed = app.clone();

    update
        .download_and_install(
            move |chunk, total| {
                static DOWNLOADED: AtomicIsize = AtomicIsize::new(0);
                let done = DOWNLOADED.fetch_add(chunk as isize, Ordering::Relaxed) as u64
                    + chunk as u64;
                let _ = emitter.emit(
                    "update-download-progress",
                    serde_json::json!({ "downloaded": done, "total": total }),
                );
            },
            move || {
                let _ = finished.emit("update-downloaded", ());
            },
        )
        .await
        .map_err(|e| {
            let _ = emitter_failed.emit("update-failed", e.to_string());
            e.to_string()
        })?;

    // Restarts the app and applies the pending Windows installer
    app.restart();
}

pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(
            tauri_plugin_global_shortcut::Builder::new()
                .with_handler(|app, shortcut, event| {
                    if event.state() != tauri_plugin_global_shortcut::ShortcutState::Pressed {
                        return;
                    }
                    // Ctrl+Alt+V: show / hide the pill on screen anytime
                    if let Ok(vis) =
                        "Control+Alt+KeyV".parse::<tauri_plugin_global_shortcut::Shortcut>()
                    {
                        if *shortcut == vis {
                            toggle_pill_visibility(app);
                            return;
                        }
                    }
                    // Alt+V (or the user's remapped key): toggle voice typing
                    let _ = app.emit("toggle-listening", ());
                })
                .build(),
        )
        .setup(|app| {
            if let Some(window) = app.get_webview_window("main") {
                #[cfg(target_os = "windows")]
                {
                    if let Ok(hwnd) = window.hwnd() {
                        VOICEFLOW_HWND.store(hwnd.0 as isize, Ordering::Relaxed);
                    }
                }
                let _ = window.set_always_on_top(true);
                // Tray-first app: no taskbar button, no Alt-Tab clutter
                let _ = window.set_skip_taskbar(true);
                // Auto-start on boot launches silently in the tray
                if std::env::args().any(|arg| arg == "--tray") {
                    let _ = window.hide();
                }
            }

            // Global hotkeys: Alt+V = mic toggle, Ctrl+Alt+V = show/hide pill
            if let Ok(shortcut) = "Alt+KeyV".parse::<tauri_plugin_global_shortcut::Shortcut>() {
                let _ = app.global_shortcut().register(shortcut);
            }
            if let Ok(shortcut) =
                "Control+Alt+KeyV".parse::<tauri_plugin_global_shortcut::Shortcut>()
            {
                let _ = app.global_shortcut().register(shortcut);
            }

            // System Tray with Unicode / ANSI mode switching and quick controls
            use tauri::menu::{Menu, MenuItem, PredefinedMenuItem};
            use tauri::tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent};

            let show_i = MenuItem::with_id(app, "show_window", "Show / Hide Pill (Ctrl+Alt+V)", true, None::<&str>)?;
            let toggle_i = MenuItem::with_id(app, "toggle_mic", "Toggle Mic (Alt+V)", true, None::<&str>)?;
            let sep1 = PredefinedMenuItem::separator(app)?;
            let mode_uni = MenuItem::with_id(app, "mode_unicode", "✔ Unicode Mode (Modern/Web)", true, None::<&str>)?;
            let mode_ansi = MenuItem::with_id(app, "mode_ansi", "  ANSI Mode (Bijoy/SutonnyMJ)", true, None::<&str>)?;
            let sep2 = PredefinedMenuItem::separator(app)?;
            let quit_i = MenuItem::with_id(app, "quit", "Quit VoiceFlow", true, None::<&str>)?;

            let menu = Menu::with_items(
                app,
                &[
                    &show_i,
                    &toggle_i,
                    &sep1,
                    &mode_uni,
                    &mode_ansi,
                    &sep2,
                    &quit_i,
                ],
            )?;

            let mut tray_builder = TrayIconBuilder::with_id("voiceflow-tray")
                .menu(&menu)
                .show_menu_on_left_click(false)
                .tooltip("VoiceFlow - Voice Typing Anywhere")
                .on_menu_event(|app, event| match event.id().as_ref() {
                    "mode_unicode" => {
                        let _ = set_encoding_mode(app.clone(), "unicode".to_string());
                    }
                    "mode_ansi" => {
                        let _ = set_encoding_mode(app.clone(), "ansi".to_string());
                    }
                    "toggle_mic" => {
                        let _ = app.emit("toggle-listening", ());
                    }
                    "show_window" => {
                        toggle_pill_visibility(app);
                    }
                    "quit" => {
                        app.exit(0);
                    }
                    _ => {}
                })
                .on_tray_icon_event(|tray, event| {
                    if let TrayIconEvent::Click {
                        button: MouseButton::Left,
                        button_state: MouseButtonState::Up,
                        ..
                    } = event
                    {
                        let app = tray.app_handle();
                        toggle_pill_visibility(app);
                    }
                });

            if let Some(icon) = app.default_window_icon() {
                tray_builder = tray_builder.icon(icon.clone());
            }

            let _tray = tray_builder.build(app)?;

            start_window_tracker(app.handle().clone());
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            type_text,
            inject_text,
            start_dragging,
            backspace,
            get_target_window_title,
            set_window_mode,
            resize_window_by,
            begin_pill_dictation,
            end_pill_dictation,
            toggle_listening,
            set_autostart,
            register_shortcut,
            minimize_window,
            close_app,
            set_encoding_mode,
            get_encoding_mode,
            check_update,
            download_and_install_update
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
