# tmmx

A terminal UI for managing tmux sessions, windows, and panes. Inspired by [lazygit](https://github.com/jesseduffield/lazygit)'s panel-based interface.

Navigate your tmux server with vim-style keybindings, create/rename/kill sessions and windows, split panes, preview pane contents, and launch sessions from templates — all without leaving your terminal.

## Requirements

- [Rust](https://www.rust-lang.org/tools/install) (1.85+ / edition 2024)
- [tmux](https://github.com/tmux/tmux) running on the system

## Install

```bash
git clone https://github.com/am2rican5/tmmx.git
cd tmmx
cargo build --release
# Binary is at target/release/tmmx
```

Optionally copy it somewhere on your `$PATH`:

```bash
cp target/release/tmmx ~/.local/bin/
```

## Usage

Start tmux (if not already running), then:

```bash
tmmx
```

**Inside tmux:** switching sessions/windows uses `switch-client` — you stay in tmux.

**Outside tmux:** selecting a session exits tmmx and attaches to it via `tmux attach-session`.

## Layout

Wide terminals (>=100 cols) show a 3-column layout:

```
┌─Sessions──┬─Panes─────┬─Preview──────────────┐
│ project   │ %0 bash   │ $ cargo build        │
│ dotfiles  │ %1 vim    │   Compiling tmmx ... │
├─Windows───┤           │                      │
│ 0: code   │           │                      │
│ 1: logs   │           │                      │
└───────────┴───────────┴──────────────────────┘
 q:quit  ?:help  1-4:panels  n:new  d:delete
```

Narrow terminals switch to a 2-column layout, toggling between Panes and Preview based on focus.

The Panes panel includes a **layout minimap** — a scaled-down visual representation of the pane arrangement in the selected window.

## Session Templates

Save any session's layout as a reusable template, then launch new sessions from it.

Templates are stored as TOML files in `~/.config/tmx/templates/` and capture each window's name, working directory, and pane splits.

| Action | How |
|--------|-----|
| Save current session as template | Focus Sessions panel, press `S`, enter a name |
| Browse & launch templates | Focus Sessions panel, press `t` to open the picker |
| Launch a template | Select in picker, press `Enter`, enter a session name |
| Delete a template | Select in picker, press `d` to confirm deletion |

Example template (`~/.config/tmx/templates/dev.toml`):

```toml
[template]
name = "dev"
description = "Development workspace"

[[windows]]
name = "code"
cwd = "/home/user/project"

[[windows.panes]]
cwd = "/home/user/project"
split = "full"

[[windows.panes]]
cwd = "/home/user/project"
split = "horizontal"
```

## Keybindings

### Global

| Key | Action |
|-----|--------|
| `q` | Quit |
| `?` | Toggle help overlay |
| `R` | Force refresh |
| `1`-`4` | Jump to panel |
| `Tab` / `Shift+Tab` | Next / previous panel |

### Sessions

| Key | Action |
|-----|--------|
| `j/k` or `Up/Down` | Navigate |
| `h/l` or `Left/Right` | Switch panel |
| `n` | New session |
| `r` | Rename session |
| `d` | Kill session (confirm) |
| `Enter` | Switch to session |
| `S` | Save session as template |
| `t` | Open template picker |

### Windows

| Key | Action |
|-----|--------|
| `j/k` or `Up/Down` | Navigate |
| `h/l` or `Left/Right` | Switch panel |
| `n` | New window |
| `r` | Rename window |
| `d` | Kill window (confirm) |
| `Enter` | Switch to window |

### Panes

| Key | Action |
|-----|--------|
| `j/k` or `Up/Down` | Navigate |
| `h/l` or `Left/Right` | Switch panel |
| `n` | Split vertical |
| `N` | Split horizontal |
| `d` | Kill pane (confirm) |
| `z` | Toggle zoom |
| `w` | Break pane to new window |
| `Enter` | Switch to pane |

## Built With

- [ratatui](https://github.com/ratatui/ratatui) — Terminal UI framework
- [crossterm](https://github.com/crossterm-rs/crossterm) — Terminal manipulation
- [color-eyre](https://github.com/eyre-rs/color-eyre) — Error reporting

## License

MIT
