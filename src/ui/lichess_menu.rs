use crate::app::App;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{BarChart, Block, BorderType, Borders, Paragraph},
    Frame,
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
            Constraint::Percentage(60), // Menu
            Constraint::Percentage(40), // Stats
        ])
        .split(main_chunks[1]);

    // Split stats area into stats text and graph
    let stats_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(15),   // Stats text
            Constraint::Length(8), // Graph
        ])
        .split(content_chunks[1]);

    // Menu options
    let menu_items = [
        ("Seek Game", "Find a random opponent"),
        ("Puzzle", "Play a puzzle"),
        ("My Ongoing Games", "View and join your current games"),
        ("Join by Code", "Enter a game code to join"),
        ("Disconnect", "Remove Lichess token and logout"),
    ];

    let mut menu_lines = vec![Line::from("")];

    for (idx, (option, description)) in menu_items.iter().enumerate() {
        let is_selected = app.menu_cursor == idx as u8;
        let is_disconnect = idx == 4; // Disconnect is the 5th option (index 4)

        let style = if is_selected {
            if is_disconnect {
                // Selected disconnect option - red background with white text
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
            // Unselected disconnect option - red text
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
    frame.render_widget(menu, content_chunks[0]);

    // User stats panel
    let mut stats_lines = vec![Line::from("")];

    if let Some(profile) = &app.lichess_user_profile {
        // Username and title
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

        // Online status
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

        // Location
        if let Some(profile_info) = &profile.profile {
            if let Some(country) = &profile_info.country {
                stats_lines.push(Line::from(vec![Span::styled(
                    "Country:",
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD),
                )]));
                stats_lines.push(Line::from(vec![
                    Span::raw("  "),
                    Span::styled(country, Style::default().fg(Color::White)),
                ]));
                stats_lines.push(Line::from(""));
            }
        }

        // Ratings with progress
        if let Some(perfs) = &profile.perfs {
            stats_lines.push(Line::from(vec![Span::styled(
                "Ratings:",
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            )]));

            if let Some(blitz) = &perfs.blitz {
                let mut rating_text = format!("{}", blitz.rating);
                if let Some(prog) = blitz.prog {
                    if prog > 0 {
                        rating_text = format!("{} (+{})", rating_text, prog);
                    } else if prog < 0 {
                        rating_text = format!("{} ({})", rating_text, prog);
                    }
                }
                stats_lines.push(Line::from(vec![
                    Span::raw("  Blitz: "),
                    Span::styled(rating_text, Style::default().fg(Color::Yellow)),
                ]));
            }
            if let Some(rapid) = &perfs.rapid {
                let mut rating_text = format!("{}", rapid.rating);
                if let Some(prog) = rapid.prog {
                    if prog > 0 {
                        rating_text = format!("{} (+{})", rating_text, prog);
                    } else if prog < 0 {
                        rating_text = format!("{} ({})", rating_text, prog);
                    }
                }
                stats_lines.push(Line::from(vec![
                    Span::raw("  Rapid: "),
                    Span::styled(rating_text, Style::default().fg(Color::Yellow)),
                ]));
            }
            if let Some(classical) = &perfs.classical {
                let mut rating_text = format!("{}", classical.rating);
                if let Some(prog) = classical.prog {
                    if prog > 0 {
                        rating_text = format!("{} (+{})", rating_text, prog);
                    } else if prog < 0 {
                        rating_text = format!("{} ({})", rating_text, prog);
                    }
                }
                stats_lines.push(Line::from(vec![
                    Span::raw("  Classical: "),
                    Span::styled(rating_text, Style::default().fg(Color::Yellow)),
                ]));
            }
            if let Some(bullet) = &perfs.bullet {
                let mut rating_text = format!("{}", bullet.rating);
                if let Some(prog) = bullet.prog {
                    if prog > 0 {
                        rating_text = format!("{} (+{})", rating_text, prog);
                    } else if prog < 0 {
                        rating_text = format!("{} ({})", rating_text, prog);
                    }
                }
                stats_lines.push(Line::from(vec![
                    Span::raw("  Bullet: "),
                    Span::styled(rating_text, Style::default().fg(Color::Yellow)),
                ]));
            }
            if let Some(puzzle) = &perfs.puzzle {
                let mut rating_text = format!("{}", puzzle.rating);
                if let Some(prog) = puzzle.prog {
                    if prog > 0 {
                        rating_text = format!("{} (+{})", rating_text, prog);
                    } else if prog < 0 {
                        rating_text = format!("{} ({})", rating_text, prog);
                    }
                }
                stats_lines.push(Line::from(vec![
                    Span::raw("  Puzzle: "),
                    Span::styled(rating_text, Style::default().fg(Color::Yellow)),
                ]));
            }
        }

        // Game statistics
        if let Some(counts) = &profile.count {
            stats_lines.push(Line::from(""));
            stats_lines.push(Line::from(vec![Span::styled(
                "Games:",
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            )]));

            if let Some(all) = counts.all {
                stats_lines.push(Line::from(vec![
                    Span::raw("  Total: "),
                    Span::styled(format!("{}", all), Style::default().fg(Color::White)),
                ]));
            }
            if let Some(win) = counts.win {
                stats_lines.push(Line::from(vec![
                    Span::raw("  Wins: "),
                    Span::styled(format!("{}", win), Style::default().fg(Color::Green)),
                ]));
            }
            if let Some(loss) = counts.loss {
                stats_lines.push(Line::from(vec![
                    Span::raw("  Losses: "),
                    Span::styled(format!("{}", loss), Style::default().fg(Color::Red)),
                ]));
            }
            if let Some(draw) = counts.draw {
                stats_lines.push(Line::from(vec![
                    Span::raw("  Draws: "),
                    Span::styled(format!("{}", draw), Style::default().fg(Color::Yellow)),
                ]));
            }
            if let Some(playing) = counts.playing {
                if playing > 0 {
                    stats_lines.push(Line::from(vec![
                        Span::raw("  Playing: "),
                        Span::styled(format!("{}", playing), Style::default().fg(Color::Magenta)),
                    ]));
                }
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
    frame.render_widget(stats, stats_chunks[0]);

    // Statistics graph - show win/loss/draw ratio or rating distribution
    if let Some(profile) = &app.lichess_user_profile {
        // Try to show win/loss/draw ratio first, fallback to rating distribution
        if let Some(counts) = &profile.count {
            if let (Some(win), Some(loss), Some(draw)) = (counts.win, counts.loss, counts.draw) {
                let total = win + loss + draw;
                if total > 0 {
                    // Create a bar chart showing win/loss/draw using tuple format
                    let data = vec![
                        ("Win", win as u64),
                        ("Draw", draw as u64),
                        ("Loss", loss as u64),
                    ];

                    let bar_chart = BarChart::default()
                        .block(
                            Block::default()
                                .borders(Borders::ALL)
                                .border_type(BorderType::Rounded)
                                .title(format!("Game Results (W:{}, D:{}, L:{})", win, draw, loss)),
                        )
                        .data(&data)
                        .bar_width(6)
                        .bar_gap(2)
                        .bar_style(Style::default().fg(Color::Cyan))
                        .max(total as u64);

                    frame.render_widget(bar_chart, stats_chunks[1]);
                } else {
                    // Fallback to rating distribution
                    render_rating_graph(frame, profile, stats_chunks[1]);
                }
            } else {
                // Fallback to rating distribution
                render_rating_graph(frame, profile, stats_chunks[1]);
            }
        } else {
            // Fallback to rating distribution
            render_rating_graph(frame, profile, stats_chunks[1]);
        }
    } else {
        let loading_graph = Paragraph::new("Loading...")
            .style(Style::default().fg(Color::Gray))
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .title("Statistics Graph"),
            )
            .alignment(Alignment::Center);
        frame.render_widget(loading_graph, stats_chunks[1]);
    }

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

fn render_rating_graph(
    frame: &mut Frame,
    profile: &crate::lichess::UserProfile,
    area: ratatui::layout::Rect,
) {
    if let Some(perfs) = &profile.perfs {
        let mut data = Vec::new();
        let mut max_rating = 1000u64;

        // Collect all available ratings with labels
        if let Some(puzzle) = &perfs.puzzle {
            data.push(("Puzzle", puzzle.rating as u64));
            max_rating = max_rating.max(puzzle.rating as u64);
        }
        if let Some(bullet) = &perfs.bullet {
            data.push(("Bullet", bullet.rating as u64));
            max_rating = max_rating.max(bullet.rating as u64);
        }
        if let Some(blitz) = &perfs.blitz {
            data.push(("Blitz", blitz.rating as u64));
            max_rating = max_rating.max(blitz.rating as u64);
        }
        if let Some(rapid) = &perfs.rapid {
            data.push(("Rapid", rapid.rating as u64));
            max_rating = max_rating.max(rapid.rating as u64);
        }
        if let Some(classical) = &perfs.classical {
            data.push(("Classical", classical.rating as u64));
            max_rating = max_rating.max(classical.rating as u64);
        }

        if !data.is_empty() {
            // Round up max_rating to nearest 100 for better visualization
            max_rating = max_rating.div_ceil(100) * 100;

            let bar_chart = BarChart::default()
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .border_type(BorderType::Rounded)
                        .title("Ratings"),
                )
                .data(&data)
                .bar_width(6)
                .bar_gap(1)
                .bar_style(Style::default().fg(Color::Cyan))
                .max(max_rating);

            frame.render_widget(bar_chart, area);
        } else {
            let empty_graph = Paragraph::new("No rating data available")
                .style(Style::default().fg(Color::Gray))
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .border_type(BorderType::Rounded)
                        .title("Ratings"),
                )
                .alignment(Alignment::Center);
            frame.render_widget(empty_graph, area);
        }
    } else {
        let empty_graph = Paragraph::new("No rating data available")
            .style(Style::default().fg(Color::Gray))
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .title("Ratings"),
            )
            .alignment(Alignment::Center);
        frame.render_widget(empty_graph, area);
    }
}
