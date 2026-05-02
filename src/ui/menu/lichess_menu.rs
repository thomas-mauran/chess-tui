//! Renders the user profile, rating sparkline, and menu options for the Lichess landing page.

use super::rating_chart::render_rating_history_chart;
use crate::app::App;
use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph},
};

pub fn render_lichess_menu(frame: &mut Frame, app: &App) {
    let area = frame.area();

    // Create main layout: title, content (menu + stats), footer
    let main_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Title
            Constraint::Min(10),   // Content (menu + stats)
            Constraint::Length(3), // Footer
        ])
        .split(area);

    // Title
    let title = Paragraph::new("Lichess Menu")
        .style(
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
        );
    frame.render_widget(title, main_chunks[0]);

    // Split content area into menu (left) and stats (right)
    let content_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(40), // Menu
            Constraint::Percentage(60), // Stats
        ])
        .split(main_chunks[1]);

    // Split stats area into stats text and graph
    let stats_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(2),     // Stats text
            Constraint::Length(24), // Graph
        ])
        .split(content_chunks[1]);

    render_menu_panel(frame, app, content_chunks[0]);

    render_user_stats_panel(frame, app, stats_chunks[0]);

    render_chart_panel(frame, app, stats_chunks[1]);

    // Footer with controls
    let footer = Paragraph::new(vec![Line::from(vec![
        Span::styled("↑/↓", Style::default().fg(Color::Cyan)),
        Span::raw(" Navigate  "),
        Span::styled("Enter", Style::default().fg(Color::Cyan)),
        Span::raw(" Select  "),
        Span::styled("Esc", Style::default().fg(Color::Cyan)),
        Span::raw(" Back to Home"),
    ])])
    .alignment(Alignment::Center)
    .block(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded),
    );
    frame.render_widget(footer, main_chunks[2]);
}

fn render_menu_panel(frame: &mut Frame, app: &App, area: Rect) {
    let menu_items = [
        ("Seek Game", "Find a random opponent"),
        ("Puzzle", "Play a puzzle"),
        ("My Ongoing Games", "View and join your current games"),
        ("Join by Code", "Enter a game code to join"),
        ("Disconnect", "Remove Lichess token and logout"),
    ];

    let mut menu_lines = vec![Line::from("")];

    for (idx, (option, description)) in menu_items.iter().enumerate() {
        let is_selected = app.ui_state.menu_cursor == idx as u8;
        let is_disconnect = idx == 4;

        let style = if is_selected {
            if is_disconnect {
                Style::default()
                    .fg(Color::White)
                    .bg(Color::Red)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default()
                    .fg(Color::Black)
                    .bg(Color::White)
                    .add_modifier(Modifier::BOLD)
            }
        } else if is_disconnect {
            Style::default().fg(Color::Red)
        } else {
            Style::default().fg(Color::White)
        };

        let prefix = if is_selected { "► " } else { "  " };
        menu_lines.push(Line::from(vec![
            Span::styled(prefix, style),
            Span::styled(*option, style),
        ]));
        menu_lines.push(Line::from(vec![
            Span::raw("    "),
            Span::styled(*description, Style::default().fg(Color::Gray)),
        ]));
        menu_lines.push(Line::from(""));
    }

    let menu = Paragraph::new(menu_lines)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .title("Select an option"),
        )
        .alignment(Alignment::Left);
    frame.render_widget(menu, area);
}

fn rating_line(label: &str, rating: u16, prog: Option<i16>) -> Line<'static> {
    let text = match prog {
        Some(p) if p > 0 => format!("{} (+{})", rating, p),
        Some(p) if p < 0 => format!("{} ({})", rating, p),
        _ => format!("{}", rating),
    };
    Line::from(vec![
        Span::raw(format!("  {}: ", label)),
        Span::styled(text, Style::default().fg(Color::Yellow)),
    ])
}

fn render_user_stats_panel(frame: &mut Frame, app: &App, area: Rect) {
    let mut stats_lines = vec![Line::from("")];

    if let Some(profile) = &app.lichess_state.user_profile {
        let username_display = if let Some(title) = &profile.title {
            format!("{} {}", title, profile.username)
        } else {
            profile.username.clone()
        };
        stats_lines.push(Line::from(vec![Span::styled(
            "Username:",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )]));
        stats_lines.push(Line::from(vec![
            Span::raw("  "),
            Span::styled(username_display, Style::default().fg(Color::White)),
        ]));

        if let Some(online) = profile.online {
            stats_lines.push(Line::from(vec![
                Span::raw("  "),
                Span::styled(
                    if online { "● Online" } else { "○ Offline" },
                    Style::default().fg(if online { Color::Green } else { Color::Gray }),
                ),
            ]));
        }
        stats_lines.push(Line::from(""));

        if let Some(profile_info) = &profile.profile
            && let Some(country) = &profile_info.country
        {
            stats_lines.push(Line::from(vec![Span::styled(
                "Country:",
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            )]));
            stats_lines.push(Line::from(vec![
                Span::raw("  "),
                Span::styled(country.clone(), Style::default().fg(Color::White)),
            ]));
            stats_lines.push(Line::from(""));
        }

        if let Some(perfs) = &profile.perfs {
            stats_lines.push(Line::from(vec![Span::styled(
                "Ratings:",
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            )]));
            if let Some(p) = &perfs.blitz {
                stats_lines.push(rating_line("Blitz", p.rating, p.prog));
            }
            if let Some(p) = &perfs.rapid {
                stats_lines.push(rating_line("Rapid", p.rating, p.prog));
            }
            if let Some(p) = &perfs.classical {
                stats_lines.push(rating_line("Classical", p.rating, p.prog));
            }
            if let Some(p) = &perfs.bullet {
                stats_lines.push(rating_line("Bullet", p.rating, p.prog));
            }
            if let Some(p) = &perfs.puzzle {
                stats_lines.push(rating_line("Puzzle", p.rating, p.prog));
            }
        }

        if let Some(counts) = &profile.count {
            stats_lines.push(Line::from(""));
            stats_lines.push(Line::from(vec![Span::styled(
                "Games:",
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            )]));
            if let Some(v) = counts.all {
                stats_lines.push(Line::from(vec![
                    Span::raw("  Total: "),
                    Span::styled(format!("{}", v), Style::default().fg(Color::White)),
                ]));
            }
            if let Some(v) = counts.win {
                stats_lines.push(Line::from(vec![
                    Span::raw("  Wins: "),
                    Span::styled(format!("{}", v), Style::default().fg(Color::Green)),
                ]));
            }
            if let Some(v) = counts.loss {
                stats_lines.push(Line::from(vec![
                    Span::raw("  Losses: "),
                    Span::styled(format!("{}", v), Style::default().fg(Color::Red)),
                ]));
            }
            if let Some(v) = counts.draw {
                stats_lines.push(Line::from(vec![
                    Span::raw("  Draws: "),
                    Span::styled(format!("{}", v), Style::default().fg(Color::Yellow)),
                ]));
            }
            if let Some(v) = counts.playing
                && v > 0
            {
                stats_lines.push(Line::from(vec![
                    Span::raw("  Playing: "),
                    Span::styled(format!("{}", v), Style::default().fg(Color::Magenta)),
                ]));
            }
        }
    } else {
        stats_lines.push(Line::from(vec![Span::styled(
            "Loading...",
            Style::default().fg(Color::Gray),
        )]));
    }

    let stats = Paragraph::new(stats_lines)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .title("User Stats"),
        )
        .alignment(Alignment::Left);
    frame.render_widget(stats, area);
}

fn render_chart_panel(frame: &mut Frame, app: &App, area: Rect) {
    if let Some(history) = &app.lichess_state.rating_history {
        render_rating_history_chart(frame, history, area);
    } else {
        let msg = if app.lichess_state.user_profile.is_some() {
            "Loading rating history..."
        } else {
            "Loading..."
        };
        let loading_graph = Paragraph::new(msg)
            .style(Style::default().fg(Color::Gray))
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .title("Rating History"),
            )
            .alignment(Alignment::Center);
        frame.render_widget(loading_graph, area);
    }
}
