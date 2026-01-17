use crate::app::App;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph, Wrap},
    Frame,
};
use shakmaty::Color as ShakmatyColor;

pub fn render_game_mode_menu(frame: &mut Frame, app: &App) {
    let area = frame.area();

    // Get selected game mode (use menu_cursor)
    let game_mode = app.menu_cursor;

    // Create main layout: title, content (menu+form | details), footer
    let main_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Title
            Constraint::Min(10),   // Content area
            Constraint::Length(3), // Footer
        ])
        .split(area);

    // Split content area into left (menu+form) and right (details panel)
    let content_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(60), // Left: menu + form
            Constraint::Percentage(40), // Right: details panel
        ])
        .split(main_chunks[1]);

    // Split left side into menu and form
    let left_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(12), // Menu
            Constraint::Min(10),    // Form
        ])
        .split(content_chunks[0]);

    // Title
    let title = Paragraph::new("Game setup")
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

    // Menu options (always visible)
    let menu_items = [
        (
            "Local Game",
            "Practice mode - Play locally on a single computer",
        ),
        ("Multiplayer", "Play with friends over network"),
        ("Play Bot", "Challenge a chess engine"),
    ];

    let mut menu_lines = vec![Line::from("")];

    for (idx, (option, description)) in menu_items.iter().enumerate() {
        let is_selected = app.menu_cursor == idx as u8;

        let style = if is_selected {
            Style::default()
                .fg(Color::Black)
                .bg(Color::White)
                .add_modifier(Modifier::BOLD)
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
                .title("Select a game mode"),
        )
        .alignment(Alignment::Left);
    frame.render_widget(menu, left_chunks[0]);

    // Render the form based on selected game mode (greyed if not active)
    render_game_mode_form(frame, app, left_chunks[1], game_mode);

    // Render details panel on the right
    render_details_panel(frame, app, content_chunks[1], game_mode);

    // Footer with controls
    let footer_text = if app.game_mode_form_active {
        vec![Line::from(vec![
            Span::styled("↑/↓", Style::default().fg(Color::Cyan)),
            Span::raw(" Navigate form  "),
            Span::styled("←/→", Style::default().fg(Color::Cyan)),
            Span::raw(" Change value  "),
            Span::styled("Enter", Style::default().fg(Color::Cyan)),
            Span::raw(" Start game  "),
            Span::styled("Esc", Style::default().fg(Color::Cyan)),
            Span::raw(" Back"),
        ])]
    } else {
        vec![Line::from(vec![
            Span::styled("↑/↓", Style::default().fg(Color::Cyan)),
            Span::raw(" Navigate menu  "),
            Span::styled("Enter", Style::default().fg(Color::Cyan)),
            Span::raw(" Activate form  "),
            Span::styled("Esc", Style::default().fg(Color::Cyan)),
            Span::raw(" Back to Home"),
        ])]
    };
    let footer = Paragraph::new(footer_text)
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
        );
    frame.render_widget(footer, main_chunks[2]);
}

fn render_details_panel(frame: &mut Frame, app: &App, area: Rect, game_mode: u8) {
    let panel_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .title("Game Mode Info");
    let inner_area = panel_block.inner(area);
    frame.render_widget(panel_block, area);

    let mut info_lines = vec![Line::from("")];

    match game_mode {
        0 => {
            // Local Game details
            info_lines.push(Line::from(vec![Span::styled(
                "Local Game",
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            )]));
            info_lines.push(Line::from(""));
            info_lines.push(Line::from("Perfect for practicing chess alone or"));
            info_lines.push(Line::from("playing with your friend on a single computer."));
            info_lines.push(Line::from(""));
            info_lines.push(Line::from(vec![Span::styled(
                "Features:",
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            )]));
            info_lines.push(Line::from("  • Play both sides"));
            info_lines.push(Line::from("  • Practice openings"));
            info_lines.push(Line::from("  • Play offline."));
            info_lines.push(Line::from(""));
            info_lines.push(Line::from(vec![Span::styled(
                "Controls:",
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            )]));
            info_lines.push(Line::from("  Arrow keys: Move cursor"));
            info_lines.push(Line::from("  Enter: Select piece/move"));
            info_lines.push(Line::from("  R: Restart game"));
            info_lines.push(Line::from("  B: Back to menu"));
        }
        1 => {
            // Multiplayer details
            info_lines.push(Line::from(vec![Span::styled(
                "Multiplayer",
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            )]));
            info_lines.push(Line::from(""));
            info_lines.push(Line::from("Play chess with friends on different devices."));
            info_lines.push(Line::from(""));
            info_lines.push(Line::from(vec![Span::styled(
                "Hosting:",
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            )]));
            info_lines.push(Line::from("  • Choose your color"));
            info_lines.push(Line::from("  • Share your IP"));
            info_lines.push(Line::from("  • Wait for opponent"));
            info_lines.push(Line::from(""));
            info_lines.push(Line::from(vec![Span::styled(
                "Joining:",
                Style::default()
                    .fg(Color::Magenta)
                    .add_modifier(Modifier::BOLD),
            )]));
            info_lines.push(Line::from("  • Enter host IP"));
            info_lines.push(Line::from("  • Connect instantly"));
            info_lines.push(Line::from("  • Color assigned"));
            info_lines.push(Line::from(""));
            info_lines.push(Line::from(vec![Span::styled(
                "Port:",
                Style::default().fg(Color::Cyan),
            )]));
            info_lines.push(Line::from("  2308 (default)"));
            info_lines.push(Line::from(""));
            info_lines.push(Line::from(
                "For setting up multiplayer on different networks, see the documentation:",
            ));
            info_lines.push(Line::from(
                "  https://thomas-mauran.github.io/chess-tui/docs/Multiplayer/Online%20multiplayer",
            ));
        }
        2 => {
            // Bot details
            info_lines.push(Line::from(vec![Span::styled(
                "Play Bot",
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            )]));
            info_lines.push(Line::from(""));
            info_lines.push(Line::from("Challenge a chess engine"));
            info_lines.push(Line::from("and improve your skills."));
            info_lines.push(Line::from(""));
            info_lines.push(Line::from(vec![Span::styled(
                "Features:",
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            )]));
            info_lines.push(Line::from("  • Choose your color"));
            info_lines.push(Line::from("  • Adjustable difficulty"));
            info_lines.push(Line::from("  • UCI engine support"));
            info_lines.push(Line::from("  • Play offline."));
            info_lines.push(Line::from(""));
            if app.chess_engine_path.is_some() {
                info_lines.push(Line::from(vec![Span::styled(
                    "Engine:",
                    Style::default()
                        .fg(Color::Green)
                        .add_modifier(Modifier::BOLD),
                )]));
                if let Some(ref path) = app.chess_engine_path {
                    let display_path = if path.len() > 25 {
                        format!("...{}", &path[path.len() - 22..])
                    } else {
                        path.clone()
                    };
                    info_lines.push(Line::from(vec![Span::raw(format!("  {}", display_path))]));
                }
            } else {
                info_lines.push(Line::from(vec![Span::styled(
                    "Engine:",
                    Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
                )]));
                info_lines.push(Line::from(vec![Span::styled(
                    "  Not configured",
                    Style::default().fg(Color::Red),
                )]));
            }
            info_lines.push(Line::from(""));
            info_lines.push(Line::from(vec![Span::styled(
                "Bot Depth:",
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            )]));
            info_lines.push(Line::from(format!("  {}", app.bot_depth)));
            info_lines.push(Line::from(""));
            info_lines.push(Line::from("  Controls how many moves"));
            info_lines.push(Line::from("  ahead the bot thinks."));
            info_lines.push(Line::from("  Higher = stronger but slower."));
            info_lines.push(Line::from(""));
            info_lines.push(Line::from(
                "  https://thomas-mauran.github.io/chess-tui/docs/intro",
            ));
        }
        _ => {}
    }

    let details = Paragraph::new(info_lines)
        .block(Block::default())
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: true });
    frame.render_widget(details, inner_area);
}

fn render_game_mode_form(frame: &mut Frame, app: &App, area: Rect, game_mode: u8) {
    // Check if form should be greyed out
    let is_active = app.game_mode_form_active;
    let grey_color = if is_active {
        Color::White
    } else {
        Color::DarkGray
    };

    // Form content area
    let form_area = area;
    let form_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .title(if is_active {
            "Configuration"
        } else {
            "Configuration (Press Enter to activate)"
        });
    let inner_form_area = form_block.inner(form_area);
    frame.render_widget(form_block, form_area);

    // Split form area into sections - all fields visible, adapting based on selections
    let form_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(match game_mode {
            0 => vec![
                Constraint::Min(1), // Local game has no fields (just start)
            ],
            1 => vec![
                Constraint::Length(3), // Mode (host/join)
                Constraint::Length(3), // Color (if hosting)
                Constraint::Min(1),
            ],
            2 => vec![
                Constraint::Length(3), // Color
                Constraint::Length(3), // Bot depth
                Constraint::Min(1),
            ],
            _ => vec![Constraint::Min(3)],
        })
        .split(inner_form_area);

    let mut chunk_idx = 0;

    match game_mode {
        0 => {
            // Local game: no fields, just start immediately
        }
        1 => {
            // Multiplayer: host/join buttons
            let mode_area = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Length(1), Constraint::Length(1)])
                .split(form_chunks[chunk_idx]);

            let mode_label = Paragraph::new("Mode:")
                .style(
                    Style::default()
                        .fg(if is_active { Color::Cyan } else { grey_color })
                        .add_modifier(if is_active {
                            Modifier::BOLD
                        } else {
                            Modifier::empty()
                        }),
                )
                .alignment(Alignment::Left);
            frame.render_widget(mode_label, mode_area[0]);

            let button_area = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([
                    Constraint::Length(8),
                    Constraint::Length(2),
                    Constraint::Length(8),
                ])
                .split(mode_area[1]);

            // Host button
            let is_focused = is_active && app.game_mode_form_cursor == 0;
            let host_selected = app.hosting == Some(true);
            let host_focused = is_focused && app.hosting.is_none();
            let host_style = if !is_active {
                Style::default().fg(grey_color)
            } else if host_selected {
                Style::default()
                    .fg(Color::Black)
                    .bg(Color::White)
                    .add_modifier(Modifier::BOLD)
            } else if host_focused || (is_focused && app.hosting.is_none()) {
                // Show focus when cursor is on this field
                Style::default()
                    .fg(Color::Black)
                    .bg(Color::Cyan)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::White)
            };
            let host_text = Paragraph::new("HOST")
                .alignment(Alignment::Center)
                .style(host_style);
            frame.render_widget(host_text, button_area[0]);

            // Join button
            let is_focused_join = is_active && app.game_mode_form_cursor == 0;
            let join_selected = app.hosting == Some(false);
            let join_focused = is_focused_join && app.hosting.is_none();
            let join_style = if !is_active {
                Style::default().fg(grey_color)
            } else if join_selected {
                Style::default()
                    .fg(Color::Black)
                    .bg(Color::White)
                    .add_modifier(Modifier::BOLD)
            } else if join_focused {
                // Don't show focus on Join when Host is focused by default
                Style::default().fg(Color::White)
            } else {
                Style::default().fg(Color::White)
            };
            let join_text = Paragraph::new("JOIN")
                .alignment(Alignment::Center)
                .style(join_style);
            frame.render_widget(join_text, button_area[2]);
            chunk_idx += 1;

            // Color selection (always visible, but grayed out if not hosting)
            let color_area = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Length(1), Constraint::Length(1)])
                .split(form_chunks[chunk_idx]);

            let color_label_style = if !is_active || app.hosting != Some(true) {
                Style::default().fg(grey_color)
            } else {
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD)
            };
            let color_label = Paragraph::new("Color:")
                .style(color_label_style)
                .alignment(Alignment::Left);
            frame.render_widget(color_label, color_area[0]);

            let color_button_area = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([
                    Constraint::Length(8),
                    Constraint::Length(2),
                    Constraint::Length(8),
                ])
                .split(color_area[1]);

            // White button
            let is_focused = is_active && app.game_mode_form_cursor == 1;
            let white_selected = app.selected_color == Some(ShakmatyColor::White);
            let white_focused =
                is_focused && app.selected_color.is_none() && app.hosting == Some(true);
            let white_style = if !is_active || !app.hosting.unwrap_or(false) {
                Style::default().fg(grey_color)
            } else if white_selected {
                Style::default()
                    .fg(Color::Black)
                    .bg(Color::White)
                    .add_modifier(Modifier::BOLD)
            } else if white_focused || (is_focused && app.selected_color.is_none()) {
                // Show focus when cursor is on this field
                Style::default()
                    .fg(Color::Black)
                    .bg(Color::Cyan)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::White)
            };
            let white_text = Paragraph::new("WHITE")
                .alignment(Alignment::Center)
                .style(white_style);
            frame.render_widget(white_text, color_button_area[0]);

            // Black button
            let black_selected = app.selected_color == Some(ShakmatyColor::Black);
            let black_style = if !is_active || !app.hosting.unwrap_or(false) {
                Style::default().fg(grey_color)
            } else if black_selected {
                Style::default()
                    .fg(Color::Black)
                    .bg(Color::White)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::White)
            };
            let black_text = Paragraph::new("BLACK")
                .alignment(Alignment::Center)
                .style(black_style);
            frame.render_widget(black_text, color_button_area[2]);
        }
        2 => {
            // Bot: color buttons
            let color_area = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Length(1), Constraint::Length(1)])
                .split(form_chunks[chunk_idx]);

            let color_label = Paragraph::new("Color:")
                .style(
                    Style::default()
                        .fg(if is_active { Color::Cyan } else { grey_color })
                        .add_modifier(if is_active {
                            Modifier::BOLD
                        } else {
                            Modifier::empty()
                        }),
                )
                .alignment(Alignment::Left);
            frame.render_widget(color_label, color_area[0]);

            let color_button_area = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([
                    Constraint::Length(8),
                    Constraint::Length(2),
                    Constraint::Length(8),
                ])
                .split(color_area[1]);
            chunk_idx += 1;

            // White button
            let is_focused = is_active && app.game_mode_form_cursor == 0;
            let white_selected = app.selected_color == Some(ShakmatyColor::White);
            let white_focused = is_focused && app.selected_color.is_none();
            let white_style = if !is_active {
                Style::default().fg(grey_color)
            } else if white_selected {
                Style::default()
                    .fg(Color::Black)
                    .bg(Color::White)
                    .add_modifier(Modifier::BOLD)
            } else if white_focused || (is_focused && app.selected_color.is_none()) {
                // Show focus when cursor is on this field
                Style::default()
                    .fg(Color::Black)
                    .bg(Color::Cyan)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::White)
            };
            let white_text = Paragraph::new("WHITE")
                .alignment(Alignment::Center)
                .style(white_style);
            frame.render_widget(white_text, color_button_area[0]);

            // Black button
            let black_selected = app.selected_color == Some(ShakmatyColor::Black);
            let black_style = if !is_active {
                Style::default().fg(grey_color)
            } else if black_selected {
                Style::default()
                    .fg(Color::Black)
                    .bg(Color::White)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::White)
            };
            let black_text = Paragraph::new("BLACK")
                .alignment(Alignment::Center)
                .style(black_style);
            frame.render_widget(black_text, color_button_area[2]);

            // Bot depth field
            let is_depth_field_focused = is_active && app.game_mode_form_cursor == 1;
            let depth_area = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Length(1), Constraint::Length(1)])
                .split(form_chunks[chunk_idx]);

            let depth_label_style = if !is_active {
                Style::default().fg(grey_color)
            } else {
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD)
            };
            let depth_label = Paragraph::new("Bot Depth:")
                .style(depth_label_style)
                .alignment(Alignment::Left);
            frame.render_widget(depth_label, depth_area[0]);

            let depth_value_area = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([
                    Constraint::Length(4), // - button
                    Constraint::Length(6), // Depth value (wider for 2-digit numbers)
                    Constraint::Length(4), // + button
                ])
                .split(depth_area[1]);

            // Decrease button
            let decrease_style = if !is_active {
                Style::default().fg(grey_color)
            } else if is_depth_field_focused {
                Style::default()
                    .fg(Color::Black)
                    .bg(Color::Cyan)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::White)
            };
            let decrease_text = Paragraph::new("  -")
                .alignment(Alignment::Center)
                .style(decrease_style);
            frame.render_widget(decrease_text, depth_value_area[0]);

            // Depth value display (centered between + and -)
            let depth_value_style = if !is_active {
                Style::default().fg(grey_color)
            } else if is_depth_field_focused {
                Style::default()
                    .fg(Color::Black)
                    .bg(Color::Cyan)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::White)
            };
            let depth_value_text = Paragraph::new(format!("  {}", app.bot_depth))
                .alignment(Alignment::Center)
                .style(depth_value_style);
            frame.render_widget(depth_value_text, depth_value_area[1]);

            // Increase button
            let increase_style = if !is_active {
                Style::default().fg(grey_color)
            } else if is_depth_field_focused {
                Style::default()
                    .fg(Color::Black)
                    .bg(Color::Cyan)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::White)
            };
            let increase_text = Paragraph::new("  +")
                .alignment(Alignment::Center)
                .style(increase_style);
            frame.render_widget(increase_text, depth_value_area[2]);
        }
        _ => {}
    }
}
