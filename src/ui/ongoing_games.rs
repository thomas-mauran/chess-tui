use crate::app::App;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
};

pub fn render_ongoing_games(frame: &mut Frame, app: &App) {
    let area = frame.area();

    // Create layout
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Title
            Constraint::Min(10),   // Games list
            Constraint::Length(3), // Footer
        ])
        .split(area);

    // Title
    let title = Paragraph::new("My Ongoing Lichess Games")
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
    frame.render_widget(title, chunks[0]);

    // Games list
    let games = &app.ongoing_games;
    let mut game_lines = vec![Line::from("")];

    if games.is_empty() {
        game_lines
            .push(Line::from("No ongoing games found.").style(Style::default().fg(Color::Gray)));
        game_lines.push(Line::from(""));
        game_lines.push(Line::from("Use 'Seek Game' to start a new game."));
    } else {
        for (idx, game) in games.iter().enumerate() {
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

            let opponent_name = &game.opponent.username;
            let rating = game
                .opponent
                .rating
                .map(|r| format!(" ({})", r))
                .unwrap_or_default();
            let turn_indicator = if game.is_my_turn {
                " ⏰ Your turn"
            } else {
                ""
            };

            game_lines.push(Line::from(vec![
                Span::styled(prefix, style),
                Span::styled(
                    format!("vs {} {}{}", opponent_name, rating, turn_indicator),
                    style,
                ),
            ]));

            game_lines.push(Line::from(vec![
                Span::raw("    "),
                Span::styled(
                    format!("Game ID: {} | Color: {}", game.game_id, game.color),
                    Style::default().fg(Color::Gray),
                ),
            ]));

            game_lines.push(Line::from(""));
        }
    }

    let games_widget = Paragraph::new(game_lines)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .title(format!("{} game(s)", games.len())),
        )
        .alignment(Alignment::Left);
    frame.render_widget(games_widget, chunks[1]);

    // Footer
    let footer = Paragraph::new(vec![Line::from(vec![
        Span::styled("↑/↓", Style::default().fg(Color::Cyan)),
        Span::raw(" Navigate  "),
        Span::styled("Enter", Style::default().fg(Color::Cyan)),
        Span::raw(" Join  "),
        Span::styled("R", Style::default().fg(Color::Cyan)),
        Span::raw(" Resign  "),
        Span::styled("Esc", Style::default().fg(Color::Cyan)),
        Span::raw(" Back"),
    ])])
    .alignment(Alignment::Center)
    .block(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded),
    );
    frame.render_widget(footer, chunks[2]);
}
