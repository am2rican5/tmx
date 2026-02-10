use ratatui::Frame;
use ratatui::buffer::Buffer;
use ratatui::layout::{Position, Rect};
use ratatui::style::{Color, Style};

use crate::app::App;
use crate::model::TmuxPane;

const MIN_CELL_WIDTH: u16 = 3;
const MIN_CELL_HEIGHT: u16 = 2;

struct MappedPane {
    x: u16,
    y: u16,
    w: u16,
    h: u16,
    label: String,
    selected: bool,
}

/// Draw the pane layout minimap. Returns `false` if the area is too small.
pub fn draw_layout_minimap(frame: &mut Frame, app: &App, area: Rect) -> bool {
    if app.panes.is_empty() {
        let buf = frame.buffer_mut();
        let msg = "(no layout)";
        let x = area.x + area.width.saturating_sub(msg.len() as u16) / 2;
        let y = area.y + area.height / 2;
        if y < area.y + area.height && x < area.x + area.width {
            buf.set_string(x, y, msg, Style::default().fg(Color::DarkGray));
        }
        return true;
    }

    let selected_index = app.pane_state.selected();
    let mapped = map_panes(&app.panes, area, selected_index);

    // Check if area is too small
    if mapped.is_none() {
        return false;
    }

    let mapped = mapped.unwrap();
    let buf = frame.buffer_mut();

    for mp in &mapped {
        draw_pane_cell(buf, mp, area);
    }

    true
}

fn map_panes(panes: &[TmuxPane], area: Rect, selected_index: Option<usize>) -> Option<Vec<MappedPane>> {
    // Derive window total dimensions
    let win_w = panes.iter().map(|p| p.left + p.width).max().unwrap_or(1);
    let win_h = panes.iter().map(|p| p.top + p.height).max().unwrap_or(1);

    if win_w == 0 || win_h == 0 {
        return Some(Vec::new());
    }

    // Check if we have enough space for all panes at minimum size
    let needed_w = (panes.len() as u16) * MIN_CELL_WIDTH;
    let needed_h = MIN_CELL_HEIGHT;
    if area.width < needed_w.min(MIN_CELL_WIDTH) || area.height < needed_h {
        return None;
    }

    let mut mapped = Vec::with_capacity(panes.len());

    for (i, pane) in panes.iter().enumerate() {
        let x = area.x + ((pane.left as u32 * area.width as u32) / win_w as u32) as u16;
        let y = area.y + ((pane.top as u32 * area.height as u32) / win_h as u32) as u16;

        let right = area.x + (((pane.left + pane.width) as u32 * area.width as u32) / win_w as u32) as u16;
        let bottom = area.y + (((pane.top + pane.height) as u32 * area.height as u32) / win_h as u32) as u16;

        let w = right.saturating_sub(x).max(MIN_CELL_WIDTH);
        let h = bottom.saturating_sub(y).max(MIN_CELL_HEIGHT);

        // Clamp to area bounds
        let w = w.min(area.x + area.width - x);
        let h = h.min(area.y + area.height - y);

        let prefix = if pane.active { "*" } else { "" };
        let label = format!("{}{} {}", prefix, pane.index, pane.command);

        mapped.push(MappedPane {
            x,
            y,
            w,
            h,
            label,
            selected: selected_index == Some(i),
        });
    }

    Some(mapped)
}

fn set_cell(buf: &mut Buffer, x: u16, y: u16, symbol: &str, style: Style) {
    if let Some(cell) = buf.cell_mut(Position::new(x, y)) {
        cell.set_symbol(symbol).set_style(style);
    }
}

fn draw_pane_cell(buf: &mut Buffer, mp: &MappedPane, clip: Rect) {
    if mp.w < 2 || mp.h < 1 {
        return;
    }

    let border_style = if mp.selected {
        Style::default().fg(Color::Cyan)
    } else {
        Style::default().fg(Color::DarkGray)
    };

    let x1 = mp.x;
    let y1 = mp.y;
    let x2 = mp.x + mp.w - 1;
    let y2 = mp.y + mp.h - 1;

    let clip_right = clip.x + clip.width;
    let clip_bottom = clip.y + clip.height;

    // Top border
    if y1 < clip_bottom {
        if x1 < clip_right {
            set_cell(buf, x1, y1, "┌", border_style);
        }
        for x in (x1 + 1)..x2.min(clip_right) {
            set_cell(buf, x, y1, "─", border_style);
        }
        if x2 < clip_right {
            set_cell(buf, x2, y1, "┐", border_style);
        }
    }

    // Bottom border
    if y2 < clip_bottom && mp.h >= 2 {
        if x1 < clip_right {
            set_cell(buf, x1, y2, "└", border_style);
        }
        for x in (x1 + 1)..x2.min(clip_right) {
            set_cell(buf, x, y2, "─", border_style);
        }
        if x2 < clip_right {
            set_cell(buf, x2, y2, "┘", border_style);
        }
    }

    // Side borders
    for y in (y1 + 1)..y2.min(clip_bottom) {
        if x1 < clip_right {
            set_cell(buf, x1, y, "│", border_style);
        }
        if x2 < clip_right {
            set_cell(buf, x2, y, "│", border_style);
        }
    }

    // Label (centered in the interior)
    if mp.h >= 2 && mp.w >= 3 {
        let interior_w = (mp.w - 2) as usize;
        let label_y = y1 + 1; // first interior row
        if label_y < clip_bottom {
            let display: String = if mp.label.len() > interior_w {
                mp.label.chars().take(interior_w).collect()
            } else {
                mp.label.clone()
            };
            let pad = (interior_w.saturating_sub(display.len())) / 2;
            let label_x = x1 + 1 + pad as u16;

            let label_style = if mp.selected {
                Style::default().fg(Color::Cyan)
            } else {
                Style::default().fg(Color::White)
            };

            for (i, ch) in display.chars().enumerate() {
                let cx = label_x + i as u16;
                if cx < clip_right && cx < x2 {
                    if let Some(cell) = buf.cell_mut(Position::new(cx, label_y)) {
                        cell.set_char(ch).set_style(label_style);
                    }
                }
            }
        }
    }
}
