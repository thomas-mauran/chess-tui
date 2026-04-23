use chess_tui::game_logic::coord::Coord;
use chess_tui::utils::{
    convert_position_into_notation, flip_square_if_needed, get_coord_from_square,
    get_int_from_char, get_opposite_square, get_square_from_coord,
};
use shakmaty::Square;

#[test]
fn test_flip_square_if_needed_unflipped() {
    assert_eq!(flip_square_if_needed(Square::E4, false), Square::E4);
    assert_eq!(flip_square_if_needed(Square::A1, false), Square::A1);
    assert_eq!(flip_square_if_needed(Square::H8, false), Square::H8);
}

#[test]
fn test_flip_square_if_needed_flipped() {
    assert_eq!(flip_square_if_needed(Square::A1, true), Square::H8);
    assert_eq!(flip_square_if_needed(Square::H8, true), Square::A1);
    assert_eq!(flip_square_if_needed(Square::E4, true), Square::D5);
}

#[test]
fn test_get_square_from_coord_unflipped() {
    assert_eq!(get_square_from_coord(Coord::new(7, 0), false), Square::A1);
    assert_eq!(get_square_from_coord(Coord::new(0, 7), false), Square::H8);
    assert_eq!(get_square_from_coord(Coord::new(4, 4), false), Square::E4);
}

#[test]
fn test_get_square_from_coord_flipped() {
    // Flipped: coord is mirrored before conversion
    assert_eq!(get_square_from_coord(Coord::new(7, 0), true), Square::H8);
    assert_eq!(get_square_from_coord(Coord::new(0, 7), true), Square::A1);
}

#[test]
fn test_get_coord_from_square_unflipped() {
    assert_eq!(get_coord_from_square(Square::A1, false), Coord::new(7, 0));
    assert_eq!(get_coord_from_square(Square::H8, false), Coord::new(0, 7));
    assert_eq!(get_coord_from_square(Square::E4, false), Coord::new(4, 4));
}

#[test]
fn test_get_coord_from_square_flipped() {
    // Flipped: square coord is reversed
    assert_eq!(get_coord_from_square(Square::A1, true), Coord::new(0, 7));
    assert_eq!(get_coord_from_square(Square::H8, true), Coord::new(7, 0));
}

#[test]
fn test_get_int_from_char_letter_mapping() {
    assert_eq!(get_int_from_char(Some('b')), 1);
    assert_eq!(get_int_from_char(Some('c')), 2);
    assert_eq!(get_int_from_char(Some('d')), 3);
    assert_eq!(get_int_from_char(Some('e')), 4);
    assert_eq!(get_int_from_char(Some('f')), 5);
    assert_eq!(get_int_from_char(Some('g')), 6);
    assert_eq!(get_int_from_char(Some('h')), 7);
}

#[test]
fn test_get_int_from_char_digit_mapping() {
    assert_eq!(get_int_from_char(Some('1')), 1);
    assert_eq!(get_int_from_char(Some('2')), 2);
    assert_eq!(get_int_from_char(Some('7')), 7);
}

#[test]
fn test_get_int_from_char_defaults_to_zero() {
    assert_eq!(get_int_from_char(None), 0);
    assert_eq!(get_int_from_char(Some('a')), 0);
    assert_eq!(get_int_from_char(Some('0')), 0);
    assert_eq!(get_int_from_char(Some('z')), 0);
}

#[test]
fn test_get_opposite_square() {
    assert_eq!(get_opposite_square(Some(Square::A1)), Some(Square::H8));
    assert_eq!(get_opposite_square(Some(Square::H8)), Some(Square::A1));
    assert_eq!(get_opposite_square(Some(Square::E4)), Some(Square::D5));
    assert_eq!(get_opposite_square(None), None);
}

#[test]
fn test_convert_position_into_notation() {
    // E2->E4: row=6,col=4 -> row=4,col=4
    assert_eq!(convert_position_into_notation("6444"), "e2e4");
    // A1->A2: row=7,col=0 -> row=6,col=0
    assert_eq!(convert_position_into_notation("7060"), "a1a2");
    // A1->H8: row=7,col=0 -> row=0,col=7
    assert_eq!(convert_position_into_notation("7007"), "a1h8");
}

#[test]
fn test_convert_position_into_notation_short_input() {
    assert_eq!(convert_position_into_notation(""), "");
    assert_eq!(convert_position_into_notation("123"), "");
}

#[test]
fn test_normalize_lowercase_to_san() {
    // possible lower case first letters
    let moves = ["ne4", "be4", "re4", "qe4", "ke4"];
    let expected = ["Ne4", "Be4", "Re4", "Qe4", "Ke4"];

    for (&input, &expected) in moves.iter().zip(expected.iter()) {
        assert_eq!(
            chess_tui::utils::normalize_lowercase_to_san(input),
            expected
        );
    }

    // check for castling
    let moves = ["o-o", "o-o-o"];
    let expected = ["O-O", "O-O-O"];
    for (&input, &expected) in moves.iter().zip(expected.iter()) {
        assert_eq!(
            chess_tui::utils::normalize_lowercase_to_san(input),
            expected
        );
    }

    // check for promotion
    let moves = ["e8=q", "e8=r", "e8=b", "e8=n"];
    let expected = ["e8=Q", "e8=R", "e8=B", "e8=N"];
    for (&input, &expected) in moves.iter().zip(expected.iter()) {
        assert_eq!(
            chess_tui::utils::normalize_lowercase_to_san(input),
            expected
        );
    }

    // check empty string
    assert_eq!(
        chess_tui::utils::normalize_lowercase_to_san("                       "),
        ""
    );
    assert_eq!(
        chess_tui::utils::normalize_lowercase_to_san("      kh4             "),
        "Kh4"
    );
}
