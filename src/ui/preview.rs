use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::Paragraph;

use crate::app::{App, Panel};
use super::panel_block;
use super::layout_minimap;

pub fn draw_preview(frame: &mut Frame, app: &App, area: Rect) {
    let focused = app.focused == Panel::Preview;
    let block = panel_block("[4] Preview", focused);

    // When Panes panel is focused, try rendering the layout minimap
    if app.focused == Panel::Panes {
        // Render the block first, then draw minimap in the inner area
        let inner = block.inner(area);
        frame.render_widget(block, area);
        if !layout_minimap::draw_layout_minimap(frame, app, inner) {
            // Fallback: minimap couldn't fit, render pane capture over the inner area
            let content = render_pane_capture(app);
            let fallback = Paragraph::new(content);
            frame.render_widget(fallback, inner);
        }
        return;
    }

    let content = match app.focused {
        Panel::Sessions => render_session_details(app),
        Panel::Windows => render_window_details(app),
        _ => render_pane_capture(app),
    };

    let widget = Paragraph::new(content).block(block);
    frame.render_widget(widget, area);
}

fn render_session_details(app: &App) -> Vec<Line<'static>> {
    match app.selected_session() {
        Some(session) => {
            vec![
                Line::from(Span::styled(
                    format!("Session: {}", session.name),
                    Style::default().fg(Color::Cyan),
                )),
                Line::from(""),
                Line::from(format!("ID:       {}", session.id)),
                Line::from(format!("Windows:  {}", session.windows)),
                Line::from(format!(
                    "Attached: {}",
                    if session.attached { "yes" } else { "no" }
                )),
                Line::from(format!("Created:  {}", session.created)),
            ]
        }
        None => vec![Line::from(Span::styled(
            "(no session selected)",
            Style::default().fg(Color::DarkGray),
        ))],
    }
}

fn render_window_details(app: &App) -> Vec<Line<'static>> {
    match app.selected_window() {
        Some(window) => {
            let session_name = app
                .selected_session()
                .map(|s| s.name)
                .unwrap_or_default();
            vec![
                Line::from(Span::styled(
                    format!("Window: {}:{}", session_name, window.name),
                    Style::default().fg(Color::Cyan),
                )),
                Line::from(""),
                Line::from(format!("Index:  {}", window.index)),
                Line::from(format!("ID:     {}", window.id)),
                Line::from(format!("Panes:  {}", window.panes)),
                Line::from(format!(
                    "Active: {}",
                    if window.active { "yes" } else { "no" }
                )),
            ]
        }
        None => vec![Line::from(Span::styled(
            "(no window selected)",
            Style::default().fg(Color::DarkGray),
        ))],
    }
}

fn render_pane_capture(app: &App) -> Vec<Line<'static>> {
    if app.pane_capture.is_empty() {
        if app.selected_pane().is_none() {
            return vec![Line::from(Span::styled(
                "(no pane selected)",
                Style::default().fg(Color::DarkGray),
            ))];
        }
        return vec![Line::from(Span::styled(
            "(empty)",
            Style::default().fg(Color::DarkGray),
        ))];
    }

    app.pane_capture
        .lines()
        .map(|l| Line::from(l.to_string()))
        .collect()
}
