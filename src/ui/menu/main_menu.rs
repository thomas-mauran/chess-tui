//! Renders the title banner, navigation menu, skin indicator, and keyboard hints.

use crate::{
    app::App,
    constants::{DisplayMode, TITLE},
};
use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Paragraph},
};

/// Renders the home menu with navigation options, skin selector, and title banner.
pub fn render_menu_ui(frame: &mut Frame, app: &App, main_area: Rect) {
    // Determine the "skin" text
    let display_mode_menu = {
        let skin_name = match app.game.ui.display_mode {
            DisplayMode::DEFAULT => "Default",
            DisplayMode::ASCII => "ASCII",
            DisplayMode::CUSTOM => app.game.ui.skin.name.as_str(),
        };
        format!("Skin: {skin_name}")
    };

    // Determine the "sound" text (only if sound feature is enabled)
    #[cfg(feature = "sound")]
    let sound_menu = {
        let sound_status = if app.sound_enabled {
            "On 🔊"
        } else {
            "Off 🔇"
        };
        format!("Sound: {sound_status}")
    };

    // Menu items with descriptions
    let mut menu_items: Vec<(&str, &str)> = vec![
        ("Play Game", "Local, Multiplayer, or Bot game"),
        ("Play on Lichess", "Play on Lichess.org"),
        (&display_mode_menu, "Change display theme"),
    ];

    // Add sound menu item only if sound feature is enabled
    #[cfg(feature = "sound")]
    {
        menu_items.push((&sound_menu, "Toggle sound effects"));
    }

    menu_items.extend(vec![
        ("Help", "View keyboard shortcuts and controls"),
        ("About", "Project information and credits"),
    ]);

    // Menu height depends on number of items, each takes 3 lines (item + description/empty + spacing), plus padding
    let menu_height = menu_items.len() as u16 * 3 + 4;

    let main_layout_horizontal = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Ratio(1, 5),         // Title
                Constraint::Length(1),           // Subtitle
                Constraint::Min(0),              // Flexible space above menu
                Constraint::Length(menu_height), // Menu (fixed height)
                Constraint::Min(0),              // Flexible space below menu
                Constraint::Ratio(1, 10),        // Footer/hints
            ]
            .as_ref(),
        )
        .split(main_area);

    // Title
    let title_paragraph = Paragraph::new(TITLE)
        .alignment(Alignment::Center)
        .block(Block::default());
    frame.render_widget(title_paragraph, main_layout_horizontal[0]);

    // Subtitle
    let sub_title = Paragraph::new("A chess game made in 🦀")
        .alignment(Alignment::Center)
        .block(Block::default());
    frame.render_widget(sub_title, main_layout_horizontal[1]);

    // Menu area (centered in the middle section)
    let menu_area = main_layout_horizontal[3];

    // Render menu items
    let mut menu_lines: Vec<Line<'_>> = vec![];

    for (i, (item, description)) in menu_items.iter().enumerate() {
        let is_selected = app.ui_state.menu_cursor == i as u8;

        // Create styled menu item
        let item_style = if is_selected {
            Style::default()
                .fg(Color::Black)
                .bg(Color::LightBlue)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::White)
        };

        // Menu item line with indicator
        let indicator = if is_selected { "▶ " } else { "  " };
        let item_text = format!("{}{}", indicator, item);
        menu_lines.push(Line::from(vec![Span::styled(item_text, item_style)]));

        // Description line (only for selected item to save space)
        if is_selected {
            menu_lines.push(Line::from(vec![Span::styled(
                format!("   {}", description),
                Style::default()
                    .fg(Color::Gray)
                    .add_modifier(Modifier::ITALIC),
            )]));
        } else {
            menu_lines.push(Line::from(""));
        }

        // Add spacing between menu items
        menu_lines.push(Line::from(""));
    }

    let menu_paragraph = Paragraph::new(menu_lines)
        .alignment(Alignment::Center)
        .block(Block::default());
    frame.render_widget(menu_paragraph, menu_area);

    // Footer with keyboard hints
    let version = env!("CARGO_PKG_VERSION");
    let footer_text = vec![
        Line::from(""),
        Line::from(vec![
            Span::styled("Navigation: ", Style::default().fg(Color::Gray)),
            Span::styled("↑/k ", Style::default().fg(Color::Yellow)),
            Span::styled("↓/j ", Style::default().fg(Color::Yellow)),
            Span::styled("| Select: ", Style::default().fg(Color::Gray)),
            Span::styled("Enter/Space", Style::default().fg(Color::Yellow)),
            Span::styled(" | Help: ", Style::default().fg(Color::Gray)),
            Span::styled("?", Style::default().fg(Color::Yellow)),
        ]),
        Line::from(format!("Version: {}", version))
            .alignment(Alignment::Center)
            .style(Style::default().fg(Color::Gray)),
    ];

    let footer = Paragraph::new(footer_text)
        .alignment(Alignment::Center)
        .block(Block::default());
    frame.render_widget(footer, main_layout_horizontal[5]);
}
