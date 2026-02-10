use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::List;

use crate::app::{App, Panel};
use super::panel_block;

pub fn draw_sessions(frame: &mut Frame, app: &mut App, area: Rect) {
    let focused = app.focused == Panel::Sessions;
    let block = panel_block("[1] Sessions", focused);

    if app.sessions.is_empty() {
        let list = List::new(vec![Line::from(Span::styled(
            "(no sessions)",
            Style::default().fg(Color::DarkGray),
        ))])
        .block(block);
        frame.render_widget(list, area);
        return;
    }

    let items: Vec<Line> = app
        .sessions
        .iter()
        .map(|s| {
            let attached = if s.attached { " *" } else { "" };
            Line::from(format!("{} [{}w]{}", s.name, s.windows, attached))
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

    frame.render_stateful_widget(list, area, &mut app.session_state);
}
