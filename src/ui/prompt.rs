use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Clear, Paragraph};

use crate::app::App;

pub fn draw_confirm(frame: &mut Frame, app: &App) {
    let area = centered_rect(50, 20, frame.area());

    frame.render_widget(Clear, area);

    let block = Block::default()
        .borders(Borders::ALL)
        .title(" Confirm ")
        .border_style(Style::default().fg(Color::Yellow));

    let inner = block.inner(area);
    frame.render_widget(block, area);

    let text = Paragraph::new(app.confirm_message.as_str())
        .style(Style::default().fg(Color::White));
    frame.render_widget(text, inner);
}

pub fn draw_text_input(frame: &mut Frame, app: &App) {
    let area = centered_rect(50, 20, frame.area());

    frame.render_widget(Clear, area);

    let block = Block::default()
        .borders(Borders::ALL)
        .title(format!(" {} ", app.input_prompt.trim_end_matches(": ")))
        .border_style(Style::default().fg(Color::Cyan));

    let inner = block.inner(area);
    frame.render_widget(block, area);

    let input_line = Line::from(vec![
        Span::raw(&app.input_buffer),
        Span::styled("_", Style::default().fg(Color::DarkGray)),
    ]);

    let hint = Line::from(Span::styled(
        "Enter: confirm  Esc: cancel",
        Style::default().fg(Color::DarkGray),
    ));

    let text = Paragraph::new(vec![input_line, Line::from(""), hint])
        .style(Style::default().fg(Color::White));
    frame.render_widget(text, inner);
}

fn centered_rect(percent_x: u16, percent_y: u16, area: Rect) -> Rect {
    let vertical = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(area);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(vertical[1])[1]
}
