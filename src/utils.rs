use crate::game_logic::coord::Coord;
use crate::game_logic::game::Game;
use crate::game_logic::game_board::GameBoard;
use crate::{
    constants::{DisplayMode, UNDEFINED_POSITION},
    pieces::PieceType,
};
use ratatui::{
    layout::{Alignment, Rect},
    style::{Color, Stylize},
    widgets::{Block, Padding, Paragraph},
};

pub fn color_to_ratatui_enum(piece_color: Option<shakmaty::Color>) -> Color {
    match piece_color {
        Some(shakmaty::Color::Black) => Color::Black,
        Some(shakmaty::Color::White) => Color::White,
        None => Color::Red,
    }
}

pub fn get_cell_paragraph<'a>(
    game: &'a Game,
    cell_coordinates: &'a Coord,
    bounding_rect: Rect,
) -> Paragraph<'a> {
    // Get piece and color
    let piece_color = game.game_board.get_piece_color(cell_coordinates);
    let piece_type = game.game_board.get_piece_type(cell_coordinates);
    let piece_enum = PieceType::piece_type_to_string_enum(piece_type, &game.ui.display_mode);

    let paragraph = match game.ui.display_mode {
        DisplayMode::DEFAULT => {
            let color_enum = color_to_ratatui_enum(piece_color);

            // Place the pieces on the board
            Paragraph::new(piece_enum).fg(color_enum)
        }
        DisplayMode::ASCII => {
            // Determine piece letter case
            let paragraph = match piece_color {
                // pieces belonging to the player on top will be lower case
                Some(shakmaty::Color::Black) => Paragraph::new(piece_enum.to_lowercase()),
                // pieces belonging to the player on bottom will be upper case
                Some(shakmaty::Color::White) => {
                    Paragraph::new(piece_enum.to_uppercase().underlined())
                }
                // Pass through original value
                None => Paragraph::new(piece_enum),
            };

            // Place the pieces on the board
            paragraph.block(Block::new().padding(Padding::vertical(bounding_rect.height / 2)))
        }
    };

    paragraph.alignment(Alignment::Center)
}