//! PGN viewer UI — renders the chess board + move history for a saved game.
//!
//! Re-uses the existing `render_game_ui` layout by syncing the PgnViewer's current
//! position into `app.game.logic.game_board` before rendering.

use crate::app::App;
use crate::game_logic::coord::Coord;
use crate::game_logic::game::GameState;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Clear, Padding, Paragraph, Wrap},
    Frame,
};

use super::main_ui::centered_rect;

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
    // Hide the square cursor — viewer is read-only, nothing to select.
    app.game.ui.cursor_coordinates = Coord::undefined();
    // Set player_turn so history panel colouring is correct
    app.game.logic.player_turn = if current_ply % 2 == 0 {
        shakmaty::Color::White
    } else {
        shakmaty::Color::Black
    };
}

pub fn render_pgn_viewer(frame: &mut Frame, app: &mut App) {
    let area = frame.area();

    // Split: board area (top) + footer bar (bottom).
    // Footer needs 4 rows: 2 borders + 2 content lines (header + controls).
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(0),    // board + history (reuses render_game_ui)
            Constraint::Length(4), // footer with controls
        ])
        .split(area);

    // Sync board state from PGN viewer
    sync_pgn_to_board(app);

    // Render the board using the existing game UI
    super::main_ui::render_game_ui(frame, app, layout[0]);

    // Footer
    render_footer(frame, app, layout[1]);

    // If sitting on the last position, overlay a small result banner
    // over the sidebar so the outcome is immediately clear.
    render_end_banner(frame, app, layout[0]);
}

fn render_footer(frame: &mut Frame, app: &App, area: Rect) {
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

    let ply_str = if current_ply == 0 {
        "Start".to_string()
    } else {
        let move_n = (current_ply + 1).div_ceil(2);
        let side = if current_ply % 2 == 1 { "W" } else { "B" };
        format!("Move {}.{}", move_n, side)
    };

    let play_label = if auto_play { "Pause" } else { "Play" };
    let speed_str = match speed {
        1 => "Fast",
        2..=4 => "Normal",
        _ => "Slow",
    };

    let mut nav_spans = vec![
        Span::styled("←/→", Style::default().fg(Color::Cyan)),
        Span::raw(" or "),
        Span::styled("P/N", Style::default().fg(Color::Cyan)),
        Span::raw(" Prev/Next  "),
        Span::styled("Space", Style::default().fg(Color::Cyan)),
        Span::raw(format!(" {}  ", play_label)),
        Span::styled("+/-", Style::default().fg(Color::Cyan)),
        Span::raw(format!(" Speed:{speed_str}  ")),
        Span::styled("g/G", Style::default().fg(Color::Cyan)),
        Span::raw(" Start/End  "),
    ];
    if game_count > 1 {
        nav_spans.push(Span::styled(
            format!("Tab {}/{}  ", game_idx, game_count),
            Style::default().fg(Color::Cyan),
        ));
    }
    nav_spans.extend([
        Span::styled("?", Style::default().fg(Color::Cyan)),
        Span::raw(" Help  "),
        Span::styled("Esc", Style::default().fg(Color::Cyan)),
        Span::raw(" Back"),
    ]);

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

    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(1), Constraint::Length(1)])
        .split(inner);

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

    frame.render_widget(
        Paragraph::new(Line::from(nav_spans)).alignment(Alignment::Center),
        rows[1],
    );
}

/// When the viewer sits on the final position, overlay a small banner with
/// the final result and termination reason. Non-modal — navigation keys
/// continue to work and the banner disappears as soon as the user steps back.
fn render_end_banner(frame: &mut Frame, app: &App, board_area: Rect) {
    let Some(viewer) = app
        .pgn_viewer_state
        .as_ref()
        .and_then(|v| v.get(app.pgn_viewer_game_idx))
    else {
        return;
    };

    if !viewer.is_at_end() {
        return;
    }

    let area = centered_rect(45, 28, board_area);

    let mut lines = vec![
        Line::from(""),
        Line::from(Span::styled(
            "Game Over",
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        ))
        .alignment(Alignment::Center),
        Line::from(""),
        Line::from(Span::styled(
            viewer.result_summary(),
            Style::default()
                .fg(Color::LightGreen)
                .add_modifier(Modifier::BOLD),
        ))
        .alignment(Alignment::Center),
    ];
    if !viewer.termination.is_empty() {
        lines.push(Line::from(""));
        lines.push(
            Line::from(Span::styled(
                viewer.termination.clone(),
                Style::default().fg(Color::Gray),
            ))
            .alignment(Alignment::Center),
        );
    }
    lines.extend([
        Line::from(""),
        Line::from(Span::styled(
            "← / P to step back",
            Style::default().fg(Color::DarkGray),
        ))
        .alignment(Alignment::Center),
    ]);

    let block = Block::default()
        .title(" Result ")
        .title_alignment(Alignment::Center)
        .borders(Borders::ALL)
        .border_type(BorderType::Double)
        .border_style(Style::default().fg(Color::Yellow))
        .padding(Padding::horizontal(1))
        .style(Style::default().bg(Color::DarkGray));

    let paragraph = Paragraph::new(lines)
        .block(block)
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true });

    frame.render_widget(Clear, area);
    frame.render_widget(paragraph, area);
}
