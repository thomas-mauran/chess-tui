//! PGN viewer UI — renders the chess board + move history for a saved game.
//!
//! Re-uses the existing `render_game_ui` layout by syncing the PgnViewer's current
//! position into `app.game.logic.game_board` before rendering.

use crate::app::App;
use crate::game_logic::game::GameState;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
};

/// Sync the PgnViewer's current ply into `app.game.logic.game_board` so that
/// existing board/history render functions see the right position.
pub fn sync_pgn_to_board(app: &mut App) {
    let (positions, moves, current_ply) = {
        if let Some(ref viewer) = app.pgn_viewer_state {
            let v = &viewer[app.pgn_viewer_game_idx];
            (
                v.positions[..=v.current_ply].to_vec(),
                v.moves[..v.current_ply].to_vec(),
                v.current_ply,
            )
        } else {
            return;
        }
    };

    app.game.logic.game_board.position_history = positions;
    app.game.logic.game_board.move_history = moves;
    app.game.logic.game_board.history_position_index = None;
    app.game.ui.selected_square = None;
    app.game.logic.game_state = GameState::Playing;
    // Set player_turn so history panel colouring is correct
    app.game.logic.player_turn = if current_ply % 2 == 0 {
        shakmaty::Color::White
    } else {
        shakmaty::Color::Black
    };
}

pub fn render_pgn_viewer(frame: &mut Frame, app: &mut App) {
    let area = frame.area();

    // Split: board area (top) + footer bar (bottom)
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(0),    // board + history (reuses render_game_ui)
            Constraint::Length(3), // footer with controls
        ])
        .split(area);

    // Sync board state from PGN viewer
    sync_pgn_to_board(app);

    // Render the board using the existing game UI
    super::main_ui::render_game_ui(frame, app, layout[0]);

    // Footer
    render_footer(frame, app, layout[1]);
}

fn render_footer(frame: &mut Frame, app: &App, area: ratatui::layout::Rect) {
    let viewer_opt = app
        .pgn_viewer_state
        .as_ref()
        .and_then(|v| v.get(app.pgn_viewer_game_idx));

    let (current_ply, total_plies, auto_play, speed, title, white, black, result, game_count) =
        if let Some(v) = viewer_opt {
            let count = app
                .pgn_viewer_state
                .as_ref()
                .map(|gs| gs.len())
                .unwrap_or(1);
            (
                v.current_ply,
                v.total_plies(),
                v.auto_play,
                v.auto_play_speed,
                v.title.as_str(),
                v.white.as_str(),
                v.black.as_str(),
                v.result.as_str(),
                count,
            )
        } else {
            return;
        };

    let game_idx = app.pgn_viewer_game_idx + 1;

    // Build footer text
    let ply_str = if current_ply == 0 {
        "Start".to_string()
    } else {
        let move_n = (current_ply + 1) / 2;
        let side = if current_ply % 2 == 1 { "W" } else { "B" };
        format!("Move {}.{}", move_n, side)
    };

    let play_icon = if auto_play { "⏸ Pause" } else { "▶ Play" };
    let speed_str = match speed {
        1 => "Fast",
        2..=4 => "Normal",
        _ => "Slow",
    };

    let nav_line = vec![
        Span::styled("←/→", Style::default().fg(Color::Yellow)),
        Span::raw(" Step  "),
        Span::styled("Space", Style::default().fg(Color::Yellow)),
        Span::raw(format!(" {}  ", play_icon)),
        Span::styled("+/-", Style::default().fg(Color::Yellow)),
        Span::raw(format!(" Speed:{speed_str}  ")),
        Span::styled("g/G", Style::default().fg(Color::Yellow)),
        Span::raw(" Start/End  "),
        if game_count > 1 {
            Span::styled(
                format!("Tab → Next game ({}/{})", game_idx, game_count),
                Style::default().fg(Color::Cyan),
            )
        } else {
            Span::raw(String::new())
        },
        Span::styled("  Esc", Style::default().fg(Color::Yellow)),
        Span::raw(" Quit"),
    ];

    // Progress bar
    let progress = if total_plies > 0 {
        let pct = current_ply * 20 / total_plies;
        format!(
            "[{}{}] {}/{}",
            "█".repeat(pct),
            "░".repeat(20 - pct),
            current_ply,
            total_plies
        )
    } else {
        "[────────────────────] 0/0".to_string()
    };

    let header_str = format!(
        "{} │ {} vs {} │ {} │  {}",
        title, white, black, result, ply_str
    );

    let footer_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(Color::DarkGray));

    let inner = footer_block.inner(area);
    frame.render_widget(footer_block, area);

    // Two rows inside the footer
    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(1), Constraint::Length(1)])
        .split(inner);

    // Row 1: game info + progress
    let row1_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Min(0), Constraint::Length(30)])
        .split(rows[0]);

    frame.render_widget(
        Paragraph::new(Line::from(vec![Span::styled(
            header_str,
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )]))
        .alignment(Alignment::Left),
        row1_layout[0],
    );
    frame.render_widget(
        Paragraph::new(Line::from(vec![Span::styled(
            progress,
            Style::default().fg(Color::Green),
        )]))
        .alignment(Alignment::Right),
        row1_layout[1],
    );

    // Row 2: key hints
    frame.render_widget(
        Paragraph::new(Line::from(nav_line)).alignment(Alignment::Left),
        rows[1],
    );
}
