# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Build & Run

```bash
cargo build          # compile
cargo run            # run (requires tmux server running)
cargo build --release  # optimized build
```

No tests or linter configured yet. Validate changes by running `cargo build` and checking for warnings.

## Architecture

tmmx is a terminal UI for managing tmux sessions, windows, and panes. Built with ratatui (0.30) + crossterm (0.29), inspired by lazygit's panel layout.

### Core Loop

`main.rs` sets up the terminal (raw mode, alternate screen, panic hook) and runs the event loop:
```
terminal.draw() → events.next() → app.handle_key() / app.tick() → repeat
```

On quit, terminal is restored. On "suspend" (attach from outside tmux), the app exits and spawns `tmux attach-session`.

### Module Responsibilities

- **`app.rs`** — Central `App` struct owns all state. Handles all keyboard input via `handle_key()` which dispatches to mode-specific handlers (Normal/TextInput/Confirm/Help), then to panel-specific handlers. Contains `refresh_tmux_state()` which re-queries tmux and preserves selections.

- **`tmux.rs`** — Pure tmux CLI wrapper. All functions shell out via `std::process::Command` with `-F` format strings. Two categories: queries (`list_sessions`, `list_windows`, `list_panes`, `capture_pane`) and mutations (`new_session`, `kill_session`, `rename_session`, etc.). Never holds state.

- **`event.rs`** — `EventReader` wraps crossterm event polling with a 250ms tick rate. Returns `AppEvent::{Key, Tick, Resize}`.

- **`model/`** — Plain data structs: `TmuxSession`, `TmuxWindow`, `TmuxPane`. Parsed from tmux format string output using `|||` as field separator.

- **`ui/`** — Rendering only, no state mutation (except `ListState` for scroll position). `ui/mod.rs` handles layout (3-column wide, 2-column narrow at <100 cols). Each panel file (`sessions.rs`, `windows.rs`, `panes.rs`, `preview.rs`) renders a `List` or `Paragraph` widget. `prompt.rs` renders both confirm and text-input modal overlays. `help.rs` renders the keybinding help overlay.

### State & Input Flow

- `App.focused: Panel` determines which panel receives keybindings
- `App.mode: InputMode` determines input routing: Normal → panel keys, TextInput → modal input, Confirm → y/n, Help → dismiss only
- Selection cascading: changing session selection refreshes windows → panes → preview
- `PendingAction` enum carries the action context through TextInput/Confirm modals back to execution

### tmux Environment Detection

`is_inside_tmux()` checks `$TMUX` env var. Inside tmux: uses `switch-client`. Outside tmux: app exits and spawns `tmux attach-session` (the "suspend" path via `app.should_suspend`).
