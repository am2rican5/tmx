use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::List;

use crate::app::{App, Panel};
use super::panel_block;

pub fn draw_panes(frame: &mut Frame, app: &mut App, area: Rect) {
    let focused = app.focused == Panel::Panes;
    let block = panel_block("[3] Panes", focused);

    if app.panes.is_empty() {
        let msg = if app.windows.is_empty() {
            "(no window selected)"
        } else {
            "(no panes)"
        };
        let list = List::new(vec![Line::from(Span::styled(
            msg,
            Style::default().fg(Color::DarkGray),
        ))])
        .block(block);
        frame.render_widget(list, area);
        return;
    }

    let items: Vec<Line> = app
        .panes
        .iter()
        .map(|p| {
            let active = if p.active { "*" } else { " " };
            Line::from(format!(
                "{}{} {} ({}x{})",
                active, p.index, p.command, p.width, p.height
            ))
        })
        .collect();

    let list = List::new(items)
        .block(block)
        .highlight_style(
            Style::default()
                .fg(Color::Black)
                .bg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol("> ");

    frame.render_stateful_widget(list, area, &mut app.pane_state);
}
