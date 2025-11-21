use super::game_logic::game::GameLogic;
use crate::{
    constants::DisplayMode,
    pieces::role_to_utf_enum,
    ui::main_ui::render_cell,
    utils::{color_to_ratatui_enum, get_square_from_coord},
};
use ratatui::{
    layout::{Alignment, Rect},
    text::Paragraph,
    widgets::{Block, Padding},
};
use shakmaty::Role;

pub fn get_cell_paragraph_from_logic<'a>(
    logic: &'a GameLogic,
    coord: &crate::game_logic::coord::Coord,
    square: Rect,
) -> Paragraph<'a> {
    use crate::pieces::{
        bishop::Bishop, king::King, knight::Knight, pawn::Pawn, queen::Queen, rook::Rook,
    };

    let square_index = get_square_from_coord(*coord, logic.game_board.is_flipped);
    let piece_color = logic.game_board.get_piece_color_at_square(&square_index);
    let piece_type = logic.game_board.get_role_at_square(&square_index);

    // Since we don't have access to UI from logic, we need to determine display mode differently
    // For now, we'll default to DEFAULT mode - this will need to be passed as a parameter
    // or we need to restructure this differently
    let display_mode = DisplayMode::DEFAULT; // TODO: Pass this as parameter

    let paragraph = match display_mode {
        DisplayMode::DEFAULT => {
            let color_enum = color_to_ratatui_enum(piece_color);

            let piece_str = match piece_type {
                Some(Role::King) => King::to_string(&display_mode),
                Some(Role::Queen) => Queen::to_string(&display_mode),
                Some(Role::Rook) => Rook::to_string(&display_mode),
                Some(Role::Bishop) => Bishop::to_string(&display_mode),
                Some(Role::Knight) => Knight::to_string(&display_mode),
                Some(Role::Pawn) => Pawn::to_string(&display_mode),
                None => " ",
            };

            Paragraph::new(piece_str)
                .fg(color_enum)
                .alignment(Alignment::Center)
        }
        DisplayMode::ASCII => {
            let paragraph = if let Some(role) = piece_type {
                let piece_enum = role_to_utf_enum(&role, piece_color);
                match piece_color {
                    Some(shakmaty::Color::Black) => Paragraph::new(piece_enum.to_lowercase()),
                    Some(shakmaty::Color::White) => {
                        Paragraph::new(piece_enum.to_uppercase().underlined())
                    }
                    None => Paragraph::new(piece_enum),
                }
            } else {
                Paragraph::new(" ")
            };

            paragraph
                .block(Block::new().padding(Padding::vertical(square.height / 2)))
                .alignment(Alignment::Center)
        }
    };

    paragraph
}
