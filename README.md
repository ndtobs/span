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
- System deps for Tauri: see [Tauri Prerequisites](https://v2.tauri.app/start/prerequisites/)

### Setup

```bash
# Install frontend dependencies
npm install

# Run in development mode
npm run tauri dev

# Build for production
npm run tauri build
```

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
