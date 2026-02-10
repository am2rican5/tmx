# tmx

A terminal UI for managing tmux sessions, windows, and panes. Inspired by [lazygit](https://github.com/jesseduffield/lazygit)'s panel-based interface.

Navigate your tmux server with vim-style keybindings, create/rename/kill sessions and windows, split panes, and preview pane contents — all without leaving your terminal.

## Requirements

- [Rust](https://www.rust-lang.org/tools/install) (1.85+ / edition 2024)
- [tmux](https://github.com/tmux/tmux) running on the system

## Install

```bash
git clone https://github.com/am2rican5/tmx.git
cd tmx
cargo build --release
# Binary is at target/release/tmx
```

Optionally copy it somewhere on your `$PATH`:

```bash
cp target/release/tmx ~/.local/bin/
```

## Usage

Start tmux (if not already running), then:

```bash
tmx
```

**Inside tmux:** switching sessions/windows uses `switch-client` — you stay in tmux.

**Outside tmux:** selecting a session exits tmx and attaches to it via `tmux attach-session`.

## Layout

Wide terminals (>=100 cols) show a 3-column layout:

```
┌─Sessions──┬─Panes─────┬─Preview──────────────┐
│ project   │ %0 bash   │ $ cargo build        │
│ dotfiles  │ %1 vim    │   Compiling tmx ...  │
├─Windows───┤           │                      │
│ 0: code   │           │                      │
│ 1: logs   │           │                      │
└───────────┴───────────┴──────────────────────┘
 q:quit  ?:help  1-4:panels  n:new  d:delete
```

Narrow terminals switch to a 2-column layout, toggling between Panes and Preview based on focus.

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
| `Enter` | Switch to pane |

## Built With

- [ratatui](https://github.com/ratatui/ratatui) — Terminal UI framework
- [crossterm](https://github.com/crossterm-rs/crossterm) — Terminal manipulation
- [color-eyre](https://github.com/eyre-rs/color-eyre) — Error reporting

## License

MIT
