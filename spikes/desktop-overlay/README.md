# desktop-overlay-spike (Issue #18)

**Disposable** technical spike. Not the production Cargo workspace, renderer, tray, or process architecture.

Compares two documented Win32 top-level window models for Solpaper desktop widgets:

| Approach | Model |
|----------|--------|
| **A** | Independent transparent top-level HWND per sample widget |
| **B** | One monitor-sized transparent surface HWND containing multiple sample widgets |

WorkerW/Progman parenting is **not** used and must never be the sole architecture.

## Requirements

- Windows 11 x64
- Rust stable (`cargo` on PATH)

## Run

From this directory:

```powershell
# Approach A — per-widget windows
cargo run --release -- --approach a

# Approach B — monitor surface
cargo run --release -- --approach b
```

### Controls

Global hotkeys use **Ctrl+Alt** so the spike does not steal bare keys from other apps
(`WS_EX_NOACTIVATE` windows also never take keyboard focus).

| Hotkey | Action |
|--------|--------|
| **Ctrl+Alt+F2** | Toggle Edit Mode / Normal Mode |
| **Ctrl+Alt+Plus** / **Ctrl+Alt+Minus** | Increase / decrease opacity |
| **Ctrl+Alt+S** | Save layout to disk |
| **Ctrl+Alt+Esc** | Exit |

In **Edit Mode**:

- Drag a card by its title bar region
- Resize via the bottom-right 16×16 grip
- Approach A: whole window receives input
- Approach B: only widget regions receive input; empty surface area is click-through (`HTTRANSPARENT`)

In **Normal Mode** both approaches make widgets non-interactive so desktop icons and other apps remain usable.

## Layout persistence

Layouts write to:

`%LOCALAPPDATA%\solpaper-overlay-spike\layout-a.json`  
`%LOCALAPPDATA%\solpaper-overlay-spike\layout-b.json`

Restored automatically on next launch of the same approach.

## Checks

```powershell
cargo fmt --all -- --check
cargo check
cargo test
cargo clippy --all-targets --all-features -- -D warnings
```

## Research output

Findings live in [`docs/research/overlay-feasibility.md`](../../docs/research/overlay-feasibility.md).

## Non-goals

- Google OAuth, wallpaper APIs, SQLite, tray polish, TUI, installer, production themes
- Establishing production crate boundaries or IPC
