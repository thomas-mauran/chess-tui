//! Renders settings menu.

use crate::{app::App, constants::DisplayMode};
use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph},
};

pub fn render_settings_menu(frame: &mut Frame, app: &App) {
    let area = frame.area();

    // Create main layout: title, content (menu), footer
    let main_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Title
            Constraint::Min(10),   // Content
            Constraint::Length(3), // Footer
        ])
        .split(area);

    // Title
    let title = Paragraph::new("Settings")
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

    // Menu options
    // Determine the "Skin" text
    let display_mode_menu = {
        let skin_name = match app.game.ui.display_mode {
            DisplayMode::DEFAULT => "Default",
            DisplayMode::ASCII => "ASCII",
            DisplayMode::CUSTOM => app.game.ui.skin.name.as_str(),
        };
        format!("Skin: {skin_name}")
    };

    // Determine the "Sound" text (only if sound feature is enabled)
    #[cfg(feature = "sound")]
    let sound_menu = {
        let sound_status = if app.sound_enabled {
            "On 🔊"
        } else {
            "Off 🔇"
        };
        format!("Sound: {sound_status}")
    };

    // Determine the "Animations" text
    let animations_menu = {
        let animation_status = if app.animations_enabled { "On" } else { "Off" };
        format!("Animations: {animation_status}")
    };

    // Determine the "Chess Engine Path" text
    let chess_engine_path_menu = "Chess Engine Path".to_string();

    // Determine the "Bot Depth" text
    let bot_depth_menu = format!("Bot Depth: -  {}  +", app.bot_state.bot_depth);

    // Determine the "Bot Difficulty" text
    let bot_difficulty_menu = {
        let difficulty = match app.bot_state.bot_difficulty {
            Some(0) => "Easy",
            Some(1) => "Medium",
            Some(2) => "Hard",
            Some(3) => "Magnus",
            _ => "Off",
        };
        format!("Bot Difficulty: {difficulty}")
    };

    // Menu items with descriptions
    let mut menu_items: Vec<(&str, &str)> = vec![(&display_mode_menu, "Change display theme")];

    // Add sound menu item only if sound feature is enabled
    #[cfg(feature = "sound")]
    {
        menu_items.push((&sound_menu, "Toggle sound effects"));
    }

    menu_items.push((&animations_menu, "Toggle animation effects"));
    menu_items.push((
        &chess_engine_path_menu,
        "Set engine path with command-line arguments",
    ));
    menu_items.push((&bot_depth_menu, "Set bot thinking depth for chess engine"));
    menu_items.push((&bot_difficulty_menu, "Set bot difficulty for chess engine"));

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
                .title(""),
        )
        .alignment(Alignment::Center);
    frame.render_widget(menu, main_chunks[1]);

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
