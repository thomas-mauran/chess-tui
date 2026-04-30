//! Renders the two-phase game-mode screen: mode list then the per-mode configuration form.

use crate::app::App;
use crate::handlers::game_mode_menu::AvailableGameMode;
use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph, Wrap},
};
use shakmaty::Color as ShakmatyColor;

use crate::constants::TIME_CONTROL_OPTIONS;
/// The three color options shown in the bot and multiplayer configuration forms.
pub enum ColorSelection {
    White,
    Black,
    Random,
}

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
) {
    let grey_color = if is_active {
        Color::White
    } else {
        Color::DarkGray
    };
    let time_control_label_area = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(1), Constraint::Length(1)])
        .split(time_control_area);

    let is_time_control_focused =
        is_active && app.game_mode_state.form_cursor == time_control_cursor;
    let time_control_label_style = if !is_active {
        Style::default().fg(grey_color)
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
        .game_mode_state
        .clock_cursor
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
    if app.game_mode_state.clock_cursor == crate::constants::TIME_CONTROL_CUSTOM_INDEX && is_active
    {
        *chunk_idx += 1;
        if *chunk_idx < form_chunks.len() {
            let custom_time_area = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Length(1), Constraint::Length(1)])
                .split(form_chunks[*chunk_idx]);

            let custom_time_cursor = time_control_cursor + 1;
            let is_custom_time_focused = app.game_mode_state.form_cursor == custom_time_cursor;
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
            let custom_time_display = format!("  {}", app.game_mode_state.custom_time_minutes);
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

fn render_color_selection_ui(
    frame: &mut Frame,
    app: &App,
    color_area: Rect,
    is_active: bool,
    is_enabled: bool,
    grey_color: Color,
    color_cursor: u8,
) {
    let color_area = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(1), Constraint::Length(1)])
        .split(color_area);

    let color_label_style = if !is_enabled {
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
            Constraint::Length(2),
            Constraint::Length(8),
        ])
        .split(color_area[1]);

    let is_focused = is_active && app.game_mode_state.form_cursor == color_cursor;
    let white_selected = app.game_mode_state.selected_color == Some(ShakmatyColor::White)
        && !app.game_mode_state.is_random_color;
    let white_focused = is_enabled
        && is_focused
        && app.game_mode_state.selected_color.is_none()
        && !app.game_mode_state.is_random_color;
    let white_style = if !is_enabled {
        Style::default().fg(grey_color)
    } else if white_selected {
        Style::default()
            .fg(Color::Black)
            .bg(Color::White)
            .add_modifier(Modifier::BOLD)
    } else if white_focused {
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

    let black_selected = app.game_mode_state.selected_color == Some(ShakmatyColor::Black)
        && !app.game_mode_state.is_random_color;
    let black_style = if !is_enabled {
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

    let random_selected = app.game_mode_state.is_random_color;
    let random_style = if !is_enabled {
        Style::default().fg(grey_color)
    } else if random_selected {
        Style::default()
            .fg(Color::Black)
            .bg(Color::White)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::White)
    };
    let random_text = Paragraph::new("RANDOM")
        .alignment(Alignment::Center)
        .style(random_style);
    frame.render_widget(random_text, color_button_area[4]);
}

pub fn render_game_mode_menu(frame: &mut Frame, app: &App) {
    let area = frame.area();

    let game_mode = match app.ui_state.menu_cursor {
        0 => AvailableGameMode::Local,
        1 => AvailableGameMode::Multiplayer,
        2 => AvailableGameMode::Bot,
        _ => AvailableGameMode::PGNLoader,
    };

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
            Constraint::Length(15), // Menu (4 items × 3 rows + borders)
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
        ("Load PGN", "Open a .pgn file and step through the game"),
    ];

    let mut menu_lines = vec![Line::from("")];

    for (idx, (option, description)) in menu_items.iter().enumerate() {
        let is_selected = app.ui_state.menu_cursor == idx as u8;

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
    let footer_text = if app.game_mode_state.form_active {
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

fn build_local_detail_lines(app: &App) -> Vec<Line<'static>> {
    let mut lines = vec![
        Line::from(vec![Span::styled(
            "Local Game",
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from(""),
        Line::from("Perfect for practicing chess alone or"),
        Line::from("playing with your friend on a single computer."),
        Line::from(""),
        Line::from(vec![Span::styled(
            "Time Control:",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )]),
    ];

    let current_time_control = app.game_mode_state.get_time_control_name();
    let time_label = if let Some(seconds) = app.game_mode_state.get_time_control_seconds() {
        if seconds < 60 {
            format!("  {} ({} sec)", current_time_control, seconds)
        } else {
            format!("  {} ({} min)", current_time_control, seconds / 60)
        }
    } else {
        format!("  {}", current_time_control)
    };
    lines.push(Line::from(vec![Span::styled(
        time_label,
        Style::default()
            .fg(Color::White)
            .add_modifier(Modifier::BOLD),
    )]));
    lines.push(Line::from(""));
    lines.push(Line::from(format!(
        "  {}",
        app.game_mode_state.get_time_control_description()
    )));
    lines.push(Line::from(""));
    lines.extend([
        Line::from(vec![Span::styled(
            "Features:",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from("  • Play both sides"),
        Line::from("  • Practice openings"),
        Line::from("  • Play offline."),
        Line::from(""),
        Line::from(vec![Span::styled(
            "Controls:",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from("  Arrow keys: Move cursor"),
        Line::from("  Enter: Select piece/move"),
        Line::from("  R: Restart game"),
        Line::from("  B: Back to menu"),
    ]);
    lines
}

fn build_multiplayer_detail_lines() -> Vec<Line<'static>> {
    vec![
        Line::from(vec![Span::styled(
            "Multiplayer",
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from(""),
        Line::from("Play chess with friends on different devices."),
        Line::from(""),
        Line::from(vec![Span::styled(
            "Hosting:",
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from("  • Choose your color"),
        Line::from("  • Share your IP"),
        Line::from("  • Wait for opponent"),
        Line::from(""),
        Line::from(vec![Span::styled(
            "Joining:",
            Style::default()
                .fg(Color::Magenta)
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from("  • Enter host IP"),
        Line::from("  • Connect instantly"),
        Line::from("  • Color assigned"),
        Line::from(""),
        Line::from(vec![Span::styled(
            "Port:",
            Style::default().fg(Color::Cyan),
        )]),
        Line::from("  2308 (default)"),
        Line::from(""),
        Line::from("For setting up multiplayer on different networks, see the documentation:"),
        Line::from(
            "  https://thomas-mauran.github.io/chess-tui/docs/Multiplayer/Online%20multiplayer",
        ),
    ]
}

fn build_bot_detail_lines(app: &App) -> Vec<Line<'static>> {
    let mut lines = vec![
        Line::from(vec![Span::styled(
            "Play Bot",
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from(""),
        Line::from("Challenge a chess engine"),
        Line::from("and improve your skills."),
        Line::from(""),
        Line::from(vec![Span::styled(
            "Features:",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from("  • Choose your color"),
        Line::from("  • Adjustable difficulty"),
        Line::from("  • UCI engine support"),
        Line::from("  • Play offline."),
        Line::from(""),
    ];

    if let Some(ref path) = app.bot_state.chess_engine_path {
        let display_path = if path.len() > 25 {
            format!("...{}", &path[path.len() - 22..])
        } else {
            path.clone()
        };
        lines.push(Line::from(vec![Span::styled(
            "Engine:",
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD),
        )]));
        lines.push(Line::from(vec![Span::raw(format!("  {}", display_path))]));
    } else {
        lines.push(Line::from(vec![Span::styled(
            "Engine:",
            Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
        )]));
        lines.push(Line::from(vec![Span::styled(
            "  Not configured",
            Style::default().fg(Color::Red),
        )]));
    }

    let difficulty_display = app
        .bot_state
        .bot_difficulty
        .and_then(|i| {
            ((i as usize) < crate::constants::BOT_DIFFICULTY_NAMES.len())
                .then_some(crate::constants::BOT_DIFFICULTY_NAMES[i as usize])
        })
        .unwrap_or("Off (full strength)");
    lines.extend([
        Line::from(""),
        Line::from(vec![Span::styled(
            "Bot Depth:",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from(format!("  {}", app.bot_state.bot_depth)),
        Line::from(""),
        Line::from(vec![Span::styled(
            "Difficulty:",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from(format!("  {}", difficulty_display)),
        Line::from(""),
        Line::from("  Controls how many moves"),
        Line::from("  ahead the bot thinks."),
        Line::from("  Higher = stronger but slower."),
        Line::from(""),
        Line::from("  https://thomas-mauran.github.io/chess-tui/docs/intro"),
    ]);
    lines
}

fn build_pgn_detail_lines() -> Vec<Line<'static>> {
    vec![
        Line::from(vec![Span::styled(
            "Load PGN",
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from(""),
        Line::from("Open a PGN file and step through the"),
        Line::from("game move by move, with auto-play and"),
        Line::from("multi-game navigation."),
        Line::from(""),
        Line::from(vec![Span::styled(
            "Usage:",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from("  Press Enter, then paste the"),
        Line::from("  absolute path to a .pgn file."),
        Line::from(""),
        Line::from(vec![Span::styled(
            "Controls in viewer:",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from("  ← / P: Previous move"),
        Line::from("  → / N: Next move"),
        Line::from("  Space: Toggle auto-play"),
        Line::from("  g / G: Jump to start / end"),
        Line::from("  Tab:   Cycle games (multi-game PGN)"),
    ]
}

fn render_details_panel(frame: &mut Frame, app: &App, area: Rect, game_mode: AvailableGameMode) {
    let panel_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .title("Game Mode Info");
    let inner_area = panel_block.inner(area);
    frame.render_widget(panel_block, area);

    let mut info_lines = vec![Line::from("")];
    info_lines.extend(match game_mode {
        AvailableGameMode::Local => build_local_detail_lines(app),
        AvailableGameMode::Multiplayer => build_multiplayer_detail_lines(),
        AvailableGameMode::Bot => build_bot_detail_lines(app),
        AvailableGameMode::PGNLoader => build_pgn_detail_lines(),
    });

    let details = Paragraph::new(info_lines)
        .block(Block::default())
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: true });
    frame.render_widget(details, inner_area);
}

fn render_spinner(
    frame: &mut Frame,
    label: &str,
    value: &str,
    area: Rect,
    is_active: bool,
    is_focused: bool,
    value_width: u16,
) {
    let grey_color = if is_active {
        Color::White
    } else {
        Color::DarkGray
    };
    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(1), Constraint::Length(1)])
        .split(area);

    let label_style = if !is_active {
        Style::default().fg(grey_color)
    } else {
        Style::default()
            .fg(Color::Cyan)
            .add_modifier(Modifier::BOLD)
    };
    frame.render_widget(
        Paragraph::new(format!("{}:", label))
            .style(label_style)
            .alignment(Alignment::Left),
        rows[0],
    );

    let ctrl_style = if !is_active {
        Style::default().fg(grey_color)
    } else if is_focused {
        Style::default()
            .fg(Color::Black)
            .bg(Color::Cyan)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::White)
    };

    let cols = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(4),
            Constraint::Length(value_width),
            Constraint::Length(4),
        ])
        .split(rows[1]);

    frame.render_widget(
        Paragraph::new("  -")
            .alignment(Alignment::Center)
            .style(ctrl_style),
        cols[0],
    );
    frame.render_widget(
        Paragraph::new(format!("  {}", value))
            .alignment(Alignment::Center)
            .style(ctrl_style),
        cols[1],
    );
    frame.render_widget(
        Paragraph::new("  +")
            .alignment(Alignment::Center)
            .style(ctrl_style),
        cols[2],
    );
}

fn render_multiplayer_form(
    frame: &mut Frame,
    app: &App,
    form_chunks: &[Rect],
    chunk_idx: &mut usize,
    is_active: bool,
    grey_color: Color,
) {
    let mode_area = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(1), Constraint::Length(1)])
        .split(form_chunks[*chunk_idx]);

    let mode_label_style = Style::default()
        .fg(if is_active { Color::Cyan } else { grey_color })
        .add_modifier(if is_active {
            Modifier::BOLD
        } else {
            Modifier::empty()
        });
    frame.render_widget(
        Paragraph::new("Mode:")
            .style(mode_label_style)
            .alignment(Alignment::Left),
        mode_area[0],
    );

    let button_area = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(8),
            Constraint::Length(2),
            Constraint::Length(8),
        ])
        .split(mode_area[1]);

    let is_focused = is_active && app.game_mode_state.form_cursor == 0;
    let host_selected = app.multiplayer_state.hosting == Some(true);
    let host_focused = is_focused && app.multiplayer_state.hosting.is_none();
    let host_style = if !is_active {
        Style::default().fg(grey_color)
    } else if host_selected {
        Style::default()
            .fg(Color::Black)
            .bg(Color::White)
            .add_modifier(Modifier::BOLD)
    } else if host_focused {
        Style::default()
            .fg(Color::Black)
            .bg(Color::Cyan)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::White)
    };
    frame.render_widget(
        Paragraph::new("HOST")
            .alignment(Alignment::Center)
            .style(host_style),
        button_area[0],
    );

    let join_selected = app.multiplayer_state.hosting == Some(false);
    let join_style = if !is_active {
        Style::default().fg(grey_color)
    } else if join_selected {
        Style::default()
            .fg(Color::Black)
            .bg(Color::White)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::White)
    };
    frame.render_widget(
        Paragraph::new("JOIN")
            .alignment(Alignment::Center)
            .style(join_style),
        button_area[2],
    );
    *chunk_idx += 1;

    render_color_selection_ui(
        frame,
        app,
        form_chunks[*chunk_idx],
        is_active,
        is_active && app.multiplayer_state.hosting == Some(true),
        grey_color,
        1,
    );
}

fn render_bot_form(
    frame: &mut Frame,
    app: &App,
    form_chunks: &[Rect],
    chunk_idx: &mut usize,
    is_active: bool,
    grey_color: Color,
) {
    render_time_control_ui(
        frame,
        app,
        form_chunks[*chunk_idx],
        form_chunks,
        chunk_idx,
        is_active,
        0,
    );
    *chunk_idx += 1;

    let is_custom = app.game_mode_state.clock_cursor == crate::constants::TIME_CONTROL_CUSTOM_INDEX;
    let color_cursor = if is_custom { 2 } else { 1 };
    render_color_selection_ui(
        frame,
        app,
        form_chunks[*chunk_idx],
        is_active,
        is_active,
        grey_color,
        color_cursor,
    );
    *chunk_idx += 1;

    let depth_cursor = if is_custom { 3 } else { 2 };
    render_spinner(
        frame,
        "Bot Depth",
        &format!("{}", app.bot_state.bot_depth),
        form_chunks[*chunk_idx],
        is_active,
        is_active && app.game_mode_state.form_cursor == depth_cursor,
        6,
    );
    *chunk_idx += 1;

    let elo_cursor = if is_custom { 4 } else { 3 };
    let elo_display = app
        .bot_state
        .bot_difficulty
        .and_then(|i| {
            ((i as usize) < crate::constants::BOT_DIFFICULTY_NAMES.len())
                .then_some(crate::constants::BOT_DIFFICULTY_NAMES[i as usize])
        })
        .unwrap_or("Off")
        .to_string();
    render_spinner(
        frame,
        "Difficulty",
        &elo_display,
        form_chunks[*chunk_idx],
        is_active,
        is_active && app.game_mode_state.form_cursor == elo_cursor,
        16,
    );
}

fn render_game_mode_form(frame: &mut Frame, app: &App, area: Rect, game_mode: AvailableGameMode) {
    let is_active = app.game_mode_state.form_active;
    let grey_color = if is_active {
        Color::White
    } else {
        Color::DarkGray
    };

    let form_title = if game_mode == AvailableGameMode::PGNLoader {
        "Press Enter to open a PGN file"
    } else if is_active {
        "Configuration"
    } else {
        "Configuration (Press Enter to activate)"
    };
    let form_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .title(form_title);
    let inner_form_area = form_block.inner(area);
    frame.render_widget(form_block, area);

    let is_custom = app.game_mode_state.clock_cursor == crate::constants::TIME_CONTROL_CUSTOM_INDEX;
    let form_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(match game_mode {
            AvailableGameMode::Local => {
                if is_custom {
                    vec![
                        Constraint::Length(3),
                        Constraint::Length(3),
                        Constraint::Min(1),
                    ]
                } else {
                    vec![Constraint::Length(3), Constraint::Min(1)]
                }
            }
            AvailableGameMode::Multiplayer => vec![
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Min(1),
            ],
            AvailableGameMode::Bot => {
                if is_custom {
                    vec![
                        Constraint::Length(3),
                        Constraint::Length(3),
                        Constraint::Length(3),
                        Constraint::Length(3),
                        Constraint::Length(3),
                        Constraint::Min(1),
                    ]
                } else {
                    vec![
                        Constraint::Length(3),
                        Constraint::Length(3),
                        Constraint::Length(3),
                        Constraint::Length(3),
                        Constraint::Min(1),
                    ]
                }
            }
            AvailableGameMode::PGNLoader => vec![Constraint::Min(3)],
        })
        .split(inner_form_area);

    let mut chunk_idx = 0;
    match game_mode {
        AvailableGameMode::Local => render_time_control_ui(
            frame,
            app,
            form_chunks[chunk_idx],
            &form_chunks,
            &mut chunk_idx,
            is_active,
            0,
        ),
        AvailableGameMode::Multiplayer => render_multiplayer_form(
            frame,
            app,
            &form_chunks,
            &mut chunk_idx,
            is_active,
            grey_color,
        ),
        AvailableGameMode::Bot => render_bot_form(
            frame,
            app,
            &form_chunks,
            &mut chunk_idx,
            is_active,
            grey_color,
        ),
        AvailableGameMode::PGNLoader => {}
    }
}
