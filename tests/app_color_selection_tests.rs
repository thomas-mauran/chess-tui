use chess_tui::app::App;
use shakmaty::Color;

#[test]
fn select_previous_color_moves_random_to_black_then_white() {
    let mut app = App {
        is_random_color: true,
        ..App::default()
    };

    app.select_previous_color_option();
    assert_eq!(app.selected_color, Some(Color::Black));
    assert!(!app.is_random_color);

    app.select_previous_color_option();
    assert_eq!(app.selected_color, Some(Color::White));
    assert!(!app.is_random_color);
}

#[test]
fn select_next_color_cycles_white_black_random() {
    let mut app = App::default();

    app.select_next_color_option();
    assert_eq!(app.selected_color, Some(Color::Black));
    assert!(!app.is_random_color);

    app.select_previous_color_option();
    assert_eq!(app.selected_color, Some(Color::White));

    app.select_next_color_option();
    assert_eq!(app.selected_color, Some(Color::Black));
    assert!(!app.is_random_color);

    app.select_next_color_option();
    assert_eq!(app.selected_color, None);
    assert!(app.is_random_color);
}

#[test]
fn resolve_selected_color_defaults_to_white() {
    let mut app = App::default();

    app.resolve_selected_color();

    assert_eq!(app.selected_color, Some(Color::White));
    assert!(!app.is_random_color);
}

#[test]
fn resolve_selected_color_preserves_random_selection_state() {
    let mut app = App {
        is_random_color: true,
        ..App::default()
    };

    app.resolve_selected_color();

    assert!(matches!(
        app.selected_color,
        Some(Color::White) | Some(Color::Black)
    ));
    assert!(app.is_random_color);
}
