//! Time-control picker shown before seeking a Lichess game.

use crate::{
    app::App,
    constants::{
        LICHESS_CLASSICAL_ASCII_ART, LICHESS_RAPID_ASCII_ART, LICHESS_TIME_CONTROLS, WHITE,
    },
};
use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Clear, Paragraph},
};

const POPUP_WIDTH: u16 = 72;
const POPUP_HEIGHT: u16 = 18;
const CARD_GAP: u16 = 2;

const OPTION_DESCRIPTIONS: [&str; 2] = ["Fast and tactical", "Slower, deeper games"];

/// Renders the Rapid/Classical choice before a Lichess seek.
pub fn render_lichess_time_control_popup(frame: &mut Frame, app: &App) {
    let area = popup_area(frame.area());
    let block = Block::default()
        .title("Seek Game - Choose Time Control")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(WHITE));
    let inner = block.inner(area);

    frame.render_widget(Clear, area);
    frame.render_widget(block, area);

    let sections = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(1)])
        .split(inner);

    let card_width = sections[0].width.saturating_sub(CARD_GAP) / 2;
    let leading = sections[0].width.saturating_sub(card_width * 2 + CARD_GAP) / 2;
    let card_areas = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(leading),
            Constraint::Length(card_width),
            Constraint::Length(CARD_GAP),
            Constraint::Length(card_width),
        ])
        .split(sections[0]);
    let cards = [card_areas[1], card_areas[3]];

    let selected = usize::from(app.game_mode_state.clock_cursor >= 4);

    for (idx, art) in [LICHESS_RAPID_ASCII_ART, LICHESS_CLASSICAL_ASCII_ART]
        .iter()
        .enumerate()
    {
        render_card(
            frame,
            cards[idx],
            art,
            LICHESS_TIME_CONTROLS[idx].name,
            &LICHESS_TIME_CONTROLS[idx].notation(),
            OPTION_DESCRIPTIONS[idx],
            idx == selected,
        );
    }

    let footer = Paragraph::new(Line::from(vec![
        Span::styled("←/→", Style::default().fg(Color::Cyan)),
        Span::raw(" Choose   "),
        Span::styled("Enter", Style::default().fg(Color::Cyan)),
        Span::raw(" Play   "),
        Span::styled("Esc", Style::default().fg(Color::Cyan)),
        Span::raw(" Cancel"),
    ]))
    .alignment(Alignment::Center);
    frame.render_widget(footer, sections[1]);
}

fn render_card(
    frame: &mut Frame,
    area: Rect,
    art: &str,
    name: &str,
    notation: &str,
    description: &str,
    selected: bool,
) {
    let (border_color, art_color, name_style) = if selected {
        (
            Color::Yellow,
            Color::White,
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )
    } else {
        (
            Color::DarkGray,
            Color::DarkGray,
            Style::default().fg(Color::Gray),
        )
    };

    let block = Block::default()
        .title(
            Line::from(Span::styled(format!(" {name} "), name_style)).alignment(Alignment::Center),
        )
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(border_color));
    let inner = block.inner(area);
    frame.render_widget(block, area);

    let sections = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(0),
            Constraint::Length(1),
            Constraint::Length(1),
        ])
        .split(inner);

    let mut art_lines = vec![Line::from("")];
    art_lines.extend(
        art.lines()
            .map(|line| Line::from(Span::styled(line, Style::default().fg(art_color)))),
    );
    frame.render_widget(
        Paragraph::new(art_lines).alignment(Alignment::Center),
        sections[0],
    );

    frame.render_widget(
        Paragraph::new(Line::from(Span::styled(
            notation,
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        )))
        .alignment(Alignment::Center),
        sections[1],
    );
    frame.render_widget(
        Paragraph::new(Line::from(Span::styled(
            description,
            Style::default().fg(Color::Gray),
        )))
        .alignment(Alignment::Center),
        sections[2],
    );
}

fn popup_area(area: Rect) -> Rect {
    let width = POPUP_WIDTH.min(area.width);
    let height = POPUP_HEIGHT.min(area.height);
    Rect {
        x: area.x + area.width.saturating_sub(width) / 2,
        y: area.y + area.height.saturating_sub(height) / 2,
        width,
        height,
    }
}
