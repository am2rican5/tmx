use std::time::{SystemTime, UNIX_EPOCH};

use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::{Color, Modifier, Style};
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
            let content = render_pane_with_header(app);
            let fallback = Paragraph::new(content);
            frame.render_widget(fallback, inner);
        }
        return;
    }

    let content = match app.focused {
        Panel::Sessions => render_session_details(app),
        Panel::Windows => render_window_details(app),
        _ => render_pane_with_header(app),
    };

    let widget = Paragraph::new(content).block(block);
    frame.render_widget(widget, area);
}

// --- Formatting helpers ---

fn label_value(label: &str, value: String) -> Line<'static> {
    Line::from(vec![
        Span::styled(label.to_string(), Style::default().fg(Color::DarkGray)),
        Span::styled(value, Style::default().fg(Color::White)),
    ])
}

fn separator_line() -> Line<'static> {
    Line::from(Span::styled(
        "────────────────────────────────────────",
        Style::default().fg(Color::DarkGray),
    ))
}

fn section_header(text: String) -> Line<'static> {
    Line::from(Span::styled(
        text,
        Style::default()
            .fg(Color::Cyan)
            .add_modifier(Modifier::BOLD),
    ))
}

fn format_relative_time(unix_ts: u64) -> String {
    if unix_ts == 0 {
        return "unknown".to_string();
    }
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    if unix_ts > now {
        return "just now".to_string();
    }
    let diff = now - unix_ts;
    if diff < 60 {
        return format!("{}s ago", diff);
    }
    if diff < 3600 {
        return format!("{}m ago", diff / 60);
    }
    if diff < 86400 {
        return format!("{}h ago", diff / 3600);
    }
    if diff < 604800 {
        return format!("{}d ago", diff / 86400);
    }
    // Older than 7 days: show absolute date
    // Simple date formatting without chrono
    let days_since_epoch = unix_ts / 86400;
    let (year, month, day) = days_to_ymd(days_since_epoch);
    let month_name = match month {
        1 => "Jan", 2 => "Feb", 3 => "Mar", 4 => "Apr",
        5 => "May", 6 => "Jun", 7 => "Jul", 8 => "Aug",
        9 => "Sep", 10 => "Oct", 11 => "Nov", 12 => "Dec",
        _ => "???",
    };
    let secs_in_day = unix_ts % 86400;
    let hour = secs_in_day / 3600;
    let minute = (secs_in_day % 3600) / 60;
    format!("{} {} {} {:02}:{:02}", month_name, day, year, hour, minute)
}

fn days_to_ymd(days: u64) -> (u64, u64, u64) {
    // Algorithm from http://howardhinnant.github.io/date_algorithms.html
    let z = days + 719468;
    let era = z / 146097;
    let doe = z - era * 146097;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let y = if m <= 2 { y + 1 } else { y };
    (y, m, d)
}

fn shorten_path(path: &str) -> String {
    if let Some(home) = std::env::var("HOME").ok() {
        if path.starts_with(&home) {
            return format!("~{}", &path[home.len()..]);
        }
    }
    path.to_string()
}

// --- Render functions ---

fn render_session_details(app: &App) -> Vec<Line<'static>> {
    match app.selected_session() {
        Some(session) => {
            let (indicator, indicator_color) = if session.attached {
                ("●", Color::Green)
            } else {
                ("○", Color::DarkGray)
            };
            let attached_text = if session.attached { "attached" } else { "detached" };

            let mut lines = vec![
                section_header(format!("  {}", session.name)),
                Line::from(""),
                label_value("  ID        ", session.id.clone()),
                label_value("  Windows   ", session.windows.to_string()),
                Line::from(vec![
                    Span::styled("  Status    ".to_string(), Style::default().fg(Color::DarkGray)),
                    Span::styled(format!("{} ", indicator), Style::default().fg(indicator_color)),
                    Span::styled(attached_text.to_string(), Style::default().fg(Color::White)),
                ]),
                label_value("  Created   ", format_relative_time(session.created)),
                label_value("  Activity  ", format_relative_time(session.last_activity)),
            ];

            if !app.windows.is_empty() {
                lines.push(Line::from(""));
                lines.push(separator_line());
                lines.push(section_header("  Windows".to_string()));
                lines.push(Line::from(""));
                for w in &app.windows {
                    let active = if w.active { " *" } else { "" };
                    lines.push(Line::from(vec![
                        Span::styled(
                            format!("  {}:", w.index),
                            Style::default().fg(Color::DarkGray),
                        ),
                        Span::styled(
                            format!(" {}{}", w.name, active),
                            Style::default().fg(Color::White),
                        ),
                    ]));
                }
            }

            lines
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

            let flags_display = if window.flags.trim().is_empty() || window.flags.trim() == "-" {
                "none".to_string()
            } else {
                window.flags.trim().to_string()
            };

            // Parse layout type from the layout string (e.g., "ab12,80x24,0,0,0" → extract dimensions)
            let layout_short = simplify_layout(&window.layout);

            let mut lines = vec![
                section_header(format!("  {}:{}", session_name, window.name)),
                Line::from(""),
                label_value("  Index     ", window.index.to_string()),
                label_value("  ID        ", window.id.clone()),
                label_value("  Panes     ", window.panes.to_string()),
                label_value("  Active    ", if window.active { "yes" } else { "no" }.to_string()),
                label_value("  Layout    ", layout_short),
                label_value("  Flags     ", flags_display),
            ];

            if !app.panes.is_empty() {
                lines.push(Line::from(""));
                lines.push(separator_line());
                lines.push(section_header("  Panes".to_string()));
                lines.push(Line::from(""));
                for p in &app.panes {
                    let active = if p.active { "*" } else { " " };
                    let short_cwd = shorten_path(&p.cwd);
                    lines.push(Line::from(vec![
                        Span::styled(
                            format!("  {}{}", active, p.index),
                            Style::default().fg(Color::DarkGray),
                        ),
                        Span::styled(
                            format!("  {}", p.command),
                            Style::default().fg(Color::White),
                        ),
                        Span::styled(
                            format!("  {}", short_cwd),
                            Style::default().fg(Color::DarkGray),
                        ),
                    ]));
                }
            }

            lines
        }
        None => vec![Line::from(Span::styled(
            "(no window selected)",
            Style::default().fg(Color::DarkGray),
        ))],
    }
}

fn render_pane_info_header(app: &App) -> Vec<Line<'static>> {
    match app.selected_pane() {
        Some(pane) => {
            let short_cwd = shorten_path(&pane.cwd);
            vec![
                Line::from(vec![
                    Span::styled(
                        format!(" {} ", pane.command),
                        Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
                    ),
                    Span::styled(" │ ", Style::default().fg(Color::DarkGray)),
                    Span::styled(short_cwd, Style::default().fg(Color::White)),
                    Span::styled(" │ ", Style::default().fg(Color::DarkGray)),
                    Span::styled(
                        format!("{}x{}", pane.width, pane.height),
                        Style::default().fg(Color::White),
                    ),
                    Span::styled(" │ ", Style::default().fg(Color::DarkGray)),
                    Span::styled(
                        format!("PID {}", pane.pid),
                        Style::default().fg(Color::DarkGray),
                    ),
                ]),
                separator_line(),
            ]
        }
        None => Vec::new(),
    }
}

fn render_pane_with_header(app: &App) -> Vec<Line<'static>> {
    let mut lines = render_pane_info_header(app);

    if app.pane_capture.is_empty() {
        if app.selected_pane().is_none() {
            lines.push(Line::from(Span::styled(
                "(no pane selected)",
                Style::default().fg(Color::DarkGray),
            )));
        } else {
            lines.push(Line::from(Span::styled(
                "(empty)",
                Style::default().fg(Color::DarkGray),
            )));
        }
    } else {
        for l in app.pane_capture.lines() {
            lines.push(Line::from(l.to_string()));
        }
    }

    lines
}

fn simplify_layout(layout: &str) -> String {
    // tmux layout strings look like "ab12,80x24,0,0,0" or more complex nested forms
    // Extract the dimensions portion if present
    if let Some((_checksum, rest)) = layout.split_once(',') {
        if let Some((dims, _)) = rest.split_once(',') {
            return dims.to_string();
        }
    }
    layout.to_string()
}
