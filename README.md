# 🕌 Jadwal Sholat

A lightweight macOS menu bar app for Islamic prayer times (jadwal sholat), built with [Tauri v2](https://tauri.app) + Alpine.js.

![macOS](https://img.shields.io/badge/macOS-Ventura%2B-black?logo=apple)
![Rust](https://img.shields.io/badge/Rust-2021-orange?logo=rust)
![Tauri](https://img.shields.io/badge/Tauri-v2-blue?logo=tauri)
![License](https://img.shields.io/badge/license-MIT-green)

## Features

- Lives in your **menu bar** — no dock icon
- Shows today's 5 prayer times (Subuh, Dzuhur, Ashar, Maghrib, Isya) with the next prayer highlighted
- **Monthly schedule** opens in a separate window
- Province & city selector powered by the [equran.id](https://equran.id) API
- Settings and API cache persist locally via `tauri-plugin-store`
- Native macOS **notifications** when each prayer time arrives
- Transparent popup with rounded corners and blur effect

## Screenshots

> Coming soon

## Prerequisites

| Tool | Version |
|------|---------|
| macOS | Ventura (13) or later |
| Xcode Command Line Tools | Latest |
| Rust | 1.85+ (edition 2021) |
| Tauri CLI | v2 |

## Setup

### 1. Install Xcode Command Line Tools

```bash
xcode-select --install
```

### 2. Install Rust

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source "$HOME/.cargo/env"
rustup update stable
```

### 3. Install Tauri CLI

```bash
cargo install tauri-cli --version "^2" --locked
```

### 4. Run in development mode

```bash
git clone https://github.com/PajarKharisma/pray-schedule.git
cd pray-schedule
cargo tauri dev
```

A crescent moon icon will appear in your macOS menu bar. Click it to open the popup.

### 5. Build for production

```bash
cargo tauri build
```

The `.app` bundle will be in `src-tauri/target/release/bundle/macos/`.

## First-time use

1. Click the tray icon — the popup opens
2. Click **⚙** (settings) in the top-right corner
3. Select your **Province** and **City**
4. Click **Simpan** (Save)
5. Prayer times will load automatically

## Tech Stack

| Layer | Technology |
|-------|-----------|
| Framework | [Tauri v2](https://tauri.app) |
| Backend | Rust (`reqwest`, `chrono`, `serde`) |
| Frontend | HTML + CSS + [Alpine.js v3](https://alpinejs.dev) (CDN, no bundler) |
| Storage | `tauri-plugin-store` |
| Notifications | `tauri-plugin-notification` |
| Prayer time API | [equran.id](https://equran.id/api/v2/shalat) |

## Project Structure

```
pray-schedule/
├── src/                  # Frontend (HTML, CSS, JS)
│   ├── index.html        # Main popup UI
│   ├── month.html        # Monthly schedule window
│   ├── app.js            # Alpine.js app (main)
│   ├── month.js          # Alpine.js app (monthly)
│   └── style.css         # Shared styles
└── src-tauri/
    ├── src/
    │   ├── lib.rs        # App setup, tray icon, window management
    │   ├── commands.rs   # Tauri invoke handlers
    │   ├── api.rs        # HTTP calls to equran.id
    │   ├── timer.rs      # Background prayer notification timer
    │   └── models.rs     # Data structs
    ├── icons/            # App & tray icons
    ├── capabilities/     # Tauri permission config
    └── tauri.conf.json   # Tauri configuration
```

## License

MIT © [PajarKharisma](https://github.com/PajarKharisma)
