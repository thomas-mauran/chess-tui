use chess_tui::skin::Skin;
use ratatui::style::Color;
use std::io::Write;
use tempfile::NamedTempFile;

#[test]
fn test_default_skin() {
    let skin = Skin::default();
    assert_eq!(skin.name, "Default");
    assert_eq!(skin.board_white_color, Color::Rgb(160, 160, 160));
    assert_eq!(skin.piece_white_color, Color::White);
}

#[test]
fn test_load_skin() {
    let json = r#"{
        "name": "Test Skin",
        "board_white_color": "Red",
        "board_black_color": "Blue",
        "piece_white_color": "Green",
        "piece_black_color": "Yellow",
        "cursor_color": "LightBlue",
        "selection_color": "LightGreen",
        "last_move_color": "LightGreen"
    }"#;

    let mut file = NamedTempFile::new().unwrap();
    write!(file, "{}", json).unwrap();

    let skin = Skin::load_from_file(file.path()).unwrap();

    assert_eq!(skin.name, "Test Skin");
    assert_eq!(skin.board_white_color, Color::Red);
    assert_eq!(skin.board_black_color, Color::Blue);
    assert_eq!(skin.piece_white_color, Color::Green);
}
