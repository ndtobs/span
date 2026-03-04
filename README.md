# Span

A modern, fast, open-source SSH terminal for network engineers.

Built to replace SecureCRT with a lightweight, keyboard-driven workflow — jump host proxying, tabbed sessions, device inventory, and Lua scripting.

## Tech Stack

- **Backend:** Rust (Tauri)
- **Frontend:** Svelte + TypeScript
- **Terminal:** xterm.js
- **SSH:** russh (pure Rust)
- **Storage:** SQLite (rusqlite)
- **Scripting:** Lua (mlua)
- **License:** MIT

## Features (MVP)

- [x] SSH connections (password + key auth)
- [ ] Tabbed sessions with reorder/rename
- [ ] Jump host / SSH proxy chains
- [ ] Device inventory (tree view, folders, search)
- [ ] Quick connect bar
- [ ] Session logging
- [ ] Credential vault (OS keyring + encrypted SQLite)
- [ ] SecureCRT session import
- [ ] Scrollback search
- [ ] Keyboard-first navigation
- [ ] Color schemes + font config

## Future

- Split panes
- Multi-send (command to N sessions)
- SFTP sidebar
- Serial/console support
- Lua scripting engine
- Netbox inventory integration
- WASM plugin system

## Development

### Prerequisites

- [Rust](https://rustup.rs/) (stable)
- [Node.js](https://nodejs.org/) >= 20
- System deps for Tauri (see platform-specific setup below)

### macOS Setup

```bash
# Install Homebrew (if needed)
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"

# Install Rust
brew install rustup
rustup-init

# Install Node.js
brew install node

# Xcode Command Line Tools (required by Tauri for macOS builds)
xcode-select --install

# Clone and run
git clone https://github.com/ndtobs/span.git
cd span
npm install
npm run tauri dev
```

### Linux Setup (Ubuntu/Debian)

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Tauri system dependencies
sudo apt install libwebkit2gtk-4.1-dev libappindicator3-dev librsvg2-dev patchelf

# Clone and run
git clone https://github.com/ndtobs/span.git
cd span
npm install
npm run tauri dev
```

### Build for Production

```bash
npm run tauri build
```

> **Note:** First build takes several minutes to compile Rust dependencies. Subsequent builds are fast with hot reload for both frontend and backend.

### Project Structure

```
span/
├── src/                    # Svelte frontend
│   ├── lib/
│   │   ├── components/     # UI components
│   │   ├── stores/         # Svelte stores (state)
│   │   └── types/          # TypeScript types
│   ├── App.svelte          # Root component
│   └── main.ts             # Entry point
├── src-tauri/              # Rust backend
│   ├── src/
│   │   ├── main.rs         # Tauri entry
│   │   ├── commands/       # IPC command handlers
│   │   ├── ssh/            # SSH connection manager
│   │   ├── inventory/      # Device inventory (SQLite)
│   │   ├── credentials/    # Credential vault
│   │   └── scripting/      # Lua engine
│   ├── migrations/         # SQLite migrations
│   └── Cargo.toml
├── static/                 # Static assets
└── package.json
```
