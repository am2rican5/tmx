use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::List;

use crate::app::{App, Panel};
use super::panel_block;

pub fn draw_windows(frame: &mut Frame, app: &mut App, area: Rect) {
    let focused = app.focused == Panel::Windows;
    let block = panel_block("[2] Windows", focused);

    if app.windows.is_empty() {
        let msg = if app.sessions.is_empty() {
            "(no session selected)"
        } else {
            "(no windows)"
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
        .windows
        .iter()
        .map(|w| {
            let active = if w.active { " *" } else { "" };
            Line::from(format!("{}:{}{}", w.index, w.name, active))
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

    frame.render_stateful_widget(list, area, &mut app.window_state);
}
