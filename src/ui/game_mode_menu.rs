use crate::app::App;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph, Wrap},
    Frame,
};
use shakmaty::Color as ShakmatyColor;

use crate::constants::TIME_CONTROL_OPTIONS;

/// Renders the time control selection UI (buttons and custom time field if needed)
/// Returns the updated chunk index after rendering
fn render_time_control_ui(
    frame: &mut Frame,
    app: &App,
    time_control_area: Rect,
    form_chunks: &[Rect],
    chunk_idx: &mut usize,
    is_active: bool,
    time_control_cursor: u8,
    grey_color: Color,
) {
    let time_control_label_area = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(1), Constraint::Length(1)])
        .split(time_control_area);

    let is_time_control_focused = is_active && app.game_mode_form_cursor == time_control_cursor;
    let time_control_label_style = if !is_active {
        Style::default().fg(grey_color)
    } else if is_time_control_focused {
        Style::default()
            .fg(Color::Cyan)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default()
            .fg(Color::Cyan)
            .add_modifier(Modifier::BOLD)
    };
    let time_control_label = Paragraph::new("Time Control:")
        .style(time_control_label_style)
        .alignment(Alignment::Left);
    frame.render_widget(time_control_label, time_control_label_area[0]);

    let current_index = app
        .clock_form_cursor
        .min(crate::constants::TIME_CONTROL_CUSTOM_INDEX) as usize;

    // Calculate button widths - need to fit all options horizontally
    let button_widths: Vec<u16> = TIME_CONTROL_OPTIONS
        .iter()
        .map(|opt| opt.len() as u16 + 2) // +2 for padding
        .collect();

    // Create constraints for each button
    let mut constraints = Vec::new();
    for width in &button_widths {
        constraints.push(Constraint::Length(*width));
    }

    let button_areas = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(constraints)
        .horizontal_margin(0)
        .split(time_control_label_area[1]);

    for (idx, option) in TIME_CONTROL_OPTIONS.iter().enumerate() {
        let is_selected = idx == current_index;
        let option_style = if !is_active {
            Style::default().fg(grey_color)
        } else if is_selected {
            Style::default()
                .fg(Color::Black)
                .bg(Color::White)
                .add_modifier(Modifier::BOLD)
        } else if is_time_control_focused && idx == current_index {
            Style::default()
                .fg(Color::Black)
                .bg(Color::Cyan)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::White)
        };
        let option_text = Paragraph::new(*option)
            .alignment(Alignment::Center)
            .style(option_style);
        frame.render_widget(option_text, button_areas[idx]);
    }

    // Show custom time adjustment field if Custom is selected
    if app.clock_form_cursor == crate::constants::TIME_CONTROL_CUSTOM_INDEX && is_active {
        *chunk_idx += 1;
        if *chunk_idx < form_chunks.len() {
            let custom_time_area = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Length(1), Constraint::Length(1)])
                .split(form_chunks[*chunk_idx]);

            let custom_time_cursor = time_control_cursor + 1;
            let is_custom_time_focused = app.game_mode_form_cursor == custom_time_cursor;
            let custom_time_label_style = if is_custom_time_focused {
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::White)
            };
            let custom_time_label = Paragraph::new("Custom Time (minutes):")
                .style(custom_time_label_style)
                .alignment(Alignment::Left);
            frame.render_widget(custom_time_label, custom_time_area[0]);

            let custom_time_value_area = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([
                    Constraint::Length(4),  // - button
                    Constraint::Length(10), // Time value
                    Constraint::Length(4),  // + button
                ])
                .split(custom_time_area[1]);

            // Decrease button
            let decrease_style = if is_custom_time_focused {
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
            frame.render_widget(decrease_text, custom_time_value_area[0]);

            // Custom time value display
            let custom_time_value_style = if is_custom_time_focused {
                Style::default()
                    .fg(Color::Black)
                    .bg(Color::Cyan)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::White)
            };
            let custom_time_display = format!("  {}", app.custom_time_minutes);
            let custom_time_value_text = Paragraph::new(custom_time_display)
                .alignment(Alignment::Center)
                .style(custom_time_value_style);
            frame.render_widget(custom_time_value_text, custom_time_value_area[1]);

            // Increase button
            let increase_style = if is_custom_time_focused {
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
            frame.render_widget(increase_text, custom_time_value_area[2]);
        }
    }
}

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
                "Time Control:",
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            )]));
            let current_time_control = app.get_time_control_name();
            let time_seconds = app.get_time_control_seconds();
            if let Some(seconds) = time_seconds {
                if seconds < 60 {
                    // Show seconds for UltraBullet
                    info_lines.push(Line::from(vec![Span::styled(
                        format!("  {} ({} sec)", current_time_control, seconds),
                        Style::default()
                            .fg(Color::White)
                            .add_modifier(Modifier::BOLD),
                    )]));
                } else {
                    // Show minutes for others
                    let minutes = seconds / 60;
                    info_lines.push(Line::from(vec![Span::styled(
                        format!("  {} ({} min)", current_time_control, minutes),
                        Style::default()
                            .fg(Color::White)
                            .add_modifier(Modifier::BOLD),
                    )]));
                }
            } else {
                info_lines.push(Line::from(vec![Span::styled(
                    format!("  {}", current_time_control),
                    Style::default()
                        .fg(Color::White)
                        .add_modifier(Modifier::BOLD),
                )]));
            }
            info_lines.push(Line::from(""));
            let description = app.get_time_control_description();
            // Add description as a single line - let Paragraph widget handle wrapping
            info_lines.push(Line::from(format!("  {}", description)));
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
            0 => {
                // Show custom time field if Custom is selected
                if app.clock_form_cursor == crate::constants::TIME_CONTROL_CUSTOM_INDEX {
                    vec![
                        Constraint::Length(3), // Time control selection
                        Constraint::Length(3), // Custom time adjustment
                        Constraint::Min(1),
                    ]
                } else {
                    vec![
                        Constraint::Length(3), // Time control selection
                        Constraint::Min(1),
                    ]
                }
            }
            1 => vec![
                Constraint::Length(3), // Mode (host/join)
                Constraint::Length(3), // Color (if hosting)
                Constraint::Min(1),
            ],
            2 => {
                // Show custom time field if Custom is selected
                if app.clock_form_cursor == crate::constants::TIME_CONTROL_CUSTOM_INDEX {
                    vec![
                        Constraint::Length(3), // Time control selection
                        Constraint::Length(3), // Custom time adjustment
                        Constraint::Length(3), // Color
                        Constraint::Length(3), // Bot depth
                        Constraint::Min(1),
                    ]
                } else {
                    vec![
                        Constraint::Length(3), // Time control selection
                        Constraint::Length(3), // Color
                        Constraint::Length(3), // Bot depth
                        Constraint::Min(1),
                    ]
                }
            }
            _ => vec![Constraint::Min(3)],
        })
        .split(inner_form_area);

    let mut chunk_idx = 0;

    match game_mode {
        0 => {
            // Local game: time control selection
            render_time_control_ui(
                frame,
                app,
                form_chunks[chunk_idx],
                &form_chunks,
                &mut chunk_idx,
                is_active,
                0, // time_control_cursor
                grey_color,
            );
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
            // Bot: time control selection
            render_time_control_ui(
                frame,
                app,
                form_chunks[chunk_idx],
                &form_chunks,
                &mut chunk_idx,
                is_active,
                0,
                grey_color,
            );
            chunk_idx += 1;

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
            let color_cursor =
                if app.clock_form_cursor == crate::constants::TIME_CONTROL_CUSTOM_INDEX {
                    2
                } else {
                    1
                };
            let is_focused = is_active && app.game_mode_form_cursor == color_cursor;
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
            let depth_cursor =
                if app.clock_form_cursor == crate::constants::TIME_CONTROL_CUSTOM_INDEX {
                    3
                } else {
                    2
                };
            let is_depth_field_focused = is_active && app.game_mode_form_cursor == depth_cursor;
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
