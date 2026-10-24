<div align="center">

<img src="docs/logo.svg" width="200" alt="VoiceFlow"/>

# VoiceFlow

### Speak. Type. Anywhere. 🎙️⚡

**বাংলায় বলুন — যেকোনো অ্যাপে লিখুন।**

Real-time voice typing for Windows. Press a hotkey, speak in Bangla or English,
and watch the text appear live — then land instantly in whatever app you're using.

[![Latest Release](https://img.shields.io/github/v/release/voiceflowdev/voiceflow?style=for-the-badge&label=RELEASE&color=1D6E53)](https://github.com/voiceflowdev/voiceflow/releases/latest)
[![Downloads](https://img.shields.io/github/downloads/voiceflowdev/voiceflow/total?style=for-the-badge&label=DOWNLOADS&color=257D5E)](https://github.com/voiceflowdev/voiceflow/releases)
[![Platform](https://img.shields.io/badge/PLATFORM-WINDOWS%2010%20%2F%2011-0078D6?style=for-the-badge&logo=windows11&logoColor=white)](https://github.com/voiceflowdev/voiceflow/releases/latest)
[![Built with Tauri](https://img.shields.io/badge/BUILT%20WITH-TAURI%202%20·%20RUST-FFC131?style=for-the-badge&logo=tauri&logoColor=black)](https://tauri.app)
[![AI Engine](https://img.shields.io/badge/AI%20ENGINE-GEMINI%20LIVE-8E75B2?style=for-the-badge&logo=googlegemini&logoColor=white)](https://ai.google.dev/gemini-api/docs)

---

</div>

<p align="center">
  <img src="docs/screenshot-pill.png" width="640" alt="VoiceFlow floating pill streaming live Bangla speech with a blinking caret"/>
</p>

<p align="center"><i>The collapsed pill — real-time live caption, waveform and encoding badge, all in one minimalist bar.</i></p>

<p align="center">
  <img src="docs/screenshot-panel.png" width="420" alt="VoiceFlow expanded control panel with listening banner, transcript editor and settings"/>
</p>

<p align="center"><i>Expand it into a full studio: live transcript editor, history, settings and quick actions.</i></p>

---

## ✨ Features

### 🎙️ Real-time Voice Engine
- **Google Gemini Live API** streaming (`gemini-3.5-transcribe-live`) over WebSocket — interim text appears **as you speak**, not after
- **Zero-config fallback** to the built-in Web Speech API when no API key is set — try it out of the box
- Language-aware **auto punctuation** (`।` for Bangla, `.` for English)
- **11 recognition languages** + auto-detect — Bangla (Bangladesh) by default

### 💊 Live Caption Pill *(new in v1.0.4)*
- While dictating, the floating pill **widens automatically** and streams your speech as live text with a blinking emerald caret
- Each confirmed phrase flashes a green **✓** right before it's typed into your app
- Mic button, encoding badge and audio waveform all stay in **one minimal bar** — no extra panels floating around

### ⌨️ Type Into Any App
- Press <kbd>Alt</kbd>+<kbd>V</kbd> anywhere and speak — Unicode keystrokes are injected **directly into the focused app**: Word, Google Docs, Chrome, WhatsApp Web, VS Code, Slack, Photoshop and more
- Live **target window badge** shows exactly where your text will land
- Every phrase is saved to a 50-item **history** — re-type or copy anything later

### 🇧🇩 Built for Bangla
- One-click **Unicode ⇄ ANSI (Bijoy / SutonnyMJ)** switching — type Bangla into legacy apps like Photoshop & Illustrator
- **Hands-free voice commands**: just say `“বিজয় মোড”` / `“ANSI mode”` or `“অভ্র মোড”` / `“Unicode mode”`
- Switch encoding from the pill badge, settings, or the system tray

### 🪶 Light & Native
- **Rust + Tauri v2** with native WebView2 — ~25–35 MB RAM, <0.5% idle CPU (no Electron bloat)
- Frameless glassmorphism UI with **dark theme**, draggable & resizable anywhere on screen
- **Tray-first**: no taskbar button, no Alt-Tab clutter — summon or hide the pill anytime with <kbd>Ctrl</kbd>+<kbd>Alt</kbd>+<kbd>V</kbd>, and it starts silently in the tray on boot
- System tray controls, start-on-boot, and **auto-update** straight from GitHub Releases

---

## ⬇️ Download & Install

1. Grab the latest installer from the [**Releases**](https://github.com/voiceflowdev/voiceflow/releases/latest) page:
   - **`VoiceFlow_x.x.x_x64-setup.exe`** — NSIS installer *(recommended)*
   - `VoiceFlow_x.x.x_x64_en-US.msi` — MSI alternative
2. Run it — that's it. Windows 10/11 with WebView2 (pre-installed on modern systems).

> VoiceFlow auto-updates itself. When a new version ships, the app downloads and installs it on next launch — no manual downloads needed.

---

## 🚀 Quick Start

1. **Launch VoiceFlow** — a small pill appears on your screen
2. **Add your Gemini API key** (free): grab one at [Google AI Studio](https://aistudio.google.com/apikey), then open the pill → **More** tab → paste the key
   *(No key? It still works via the offline Web Speech fallback.)*
3. Click into any text field in any app
4. Press <kbd>Alt</kbd>+<kbd>V</kbd> and **speak** — watch the pill show your words live, then type into the app ✨

---

## ⌨️ Shortcuts

| Shortcut | Action |
| :--- | :--- |
| <kbd>Alt</kbd>+<kbd>V</kbd> | Toggle voice typing **globally** (works inside any app) — remappable in settings |
| <kbd>Ctrl</kbd>+<kbd>Alt</kbd>+<kbd>V</kbd> | Show / hide the floating pill anytime — the app lives quietly in the system tray |
| <kbd>Ctrl</kbd>+<kbd>Enter</kbd> | Type the editor's text into the active app |
| <kbd>Esc</kbd> | Collapse the panel back to the floating pill |

---

## 🌍 Recognition Languages

| | | | |
| :--- | :--- | :--- | :--- |
| 🇧🇩 Bangla (Bangladesh) | 🇮🇳 Bangla (India) | 🇺🇸 English (US) | 🇮🇳 English (India) |
| 🇮🇳 Hindi | 🇳🇵 Nepali | 🇮🇳 Assamese | 🇮🇳 Odia |
| 🇪🇬 Arabic (Egypt) | 🌎 Spanish | 🇯🇵 Japanese | 🔍 Auto-detect |

---

## 🔤 Bangla Encoding Modes

| Mode | Best for | Apps |
| :--- | :--- | :--- |
| **Unicode** ✅ default | Modern web & everyday typing | Facebook, Messenger, Word, Google Docs, browsers |
| **ANSI (Bijoy)** | Legacy ANSI Bangla fonts | Photoshop, Illustrator, old DTP software (SutonnyMJ) |

Switch anywhere: the **UNI/ANSI badge** on the pill, the Settings dropdown, the system-tray menu — or just **say it**: `“বিজয় মোড”` or `“ইউনিকোড মোড”`.

---

## 🛠️ Tech Stack

| Layer | Technology |
| :--- | :--- |
| Native backend | **Rust** + **Tauri v2** (system tray, global shortcuts, updater) |
| Text injection | Windows `SendInput` — batched Unicode keystroke injection |
| Frontend | Vanilla JS + Tailwind CSS, single-file UI with glassmorphism design |
| Speech engine | Google **Gemini Live API** (WebSocket, 16 kHz PCM streaming) + Web Speech API fallback |
| Bangla encoding | Built-in Unicode ⇄ Bijoy ANSI converter |
| CI/CD | GitHub Actions — signed installers + `latest.json` updater artifacts |

---

## 📁 Project Structure

```
voiceflow/
├── src/                        # Frontend
│   ├── index.html              # Collapsed pill + expanded panel UI
│   ├── geminiLive.js           # Gemini Live WebSocket engine + Web Speech fallback
│   ├── bijoyConverter.js       # Unicode ⇄ Bijoy (ANSI) converter
│   └── styles.css
├── src-tauri/                  # Native Rust backend
│   ├── tauri.conf.json         # Frameless, transparent, always-on-top window
│   └── src/
│       ├── lib.rs              # SendInput injector, window tracker, tray, updater
│       └── main.rs             # Entry point
├── .github/workflows/          # CI build + release pipelines
└── docs/                       # Logo & screenshots
```

---

## 🔨 Build from Source

**Prerequisites:** [Rust](https://rustup.rs/) (stable) · [Node.js](https://nodejs.org/) 18+ · Windows 10/11

```bash
# Install dependencies
npm install

# Run in development mode
npm run tauri dev

# Build a release installer
npm run tauri build
```

---

## 🗺️ Roadmap

- [ ] More voice commands (new line, punctuation control, delete-last-phrase)
- [ ] Custom dictionary & name recognition boost
- [ ] On-device offline recognition option
- [ ] Explore Linux & macOS support

---

## 🤝 Contributing

Bug reports, feature ideas and pull requests are welcome at the [issue tracker](https://github.com/voiceflowdev/voiceflow/issues).

---

<div align="center">

**If VoiceFlow saves you typing time, a ⭐ star is the best compliment!**

Made with ❤️ in Bangladesh 🇧🇩

</div>
