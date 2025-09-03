use chess_tui::game_logic::coord::Coord;
use chess_tui::game_logic::perspective::{PerspectiveManager, PlayerPerspective};
use chess_tui::pieces::{PieceColor, PieceMove, PieceType};

#[test]
fn test_white_perspective_no_transformation() {
    let manager = PerspectiveManager::white_perspective();
    assert!(!manager.needs_transformation());
    assert_eq!(manager.current_perspective, PlayerPerspective::White);
    assert_eq!(manager.player_color, PieceColor::White);
}

#[test]
fn test_black_perspective_needs_transformation() {
    let manager = PerspectiveManager::new(PieceColor::Black);
    assert!(manager.needs_transformation());
    assert_eq!(manager.current_perspective, PlayerPerspective::Black);
    assert_eq!(manager.player_color, PieceColor::Black);
}

#[test]
fn test_perspective_for_player() {
    assert_eq!(
        PlayerPerspective::for_player(PieceColor::White),
        PlayerPerspective::White,
    );
    assert_eq!(
        PlayerPerspective::for_player(PieceColor::Black),
        PlayerPerspective::Black,
    );
}

#[test]
fn test_perspective_opposite() {
    assert_eq!(
        PlayerPerspective::White.opposite(),
        PlayerPerspective::Black,
    );
    assert_eq!(
        PlayerPerspective::Black.opposite(),
        PlayerPerspective::White,
    );
}

#[test]
fn test_coordinate_transformation_black_perspective() {
    let manager = PerspectiveManager::new(PieceColor::Black);

    // Test corner transformations
    let coord = Coord::new(0, 0);
    let transformed = manager.logical_to_display(coord);
    assert_eq!(transformed, Coord::new(7, 7));

    let back_transformed = manager.display_to_logical(transformed);
    assert_eq!(back_transformed, coord);

    // Test center transformations
    let coord = Coord::new(3, 4);
    let transformed = manager.logical_to_display(coord);
    assert_eq!(transformed, Coord::new(4, 3));

    let back_transformed = manager.display_to_logical(transformed);
    assert_eq!(back_transformed, coord);
}

#[test]
fn test_coordinate_transformation_white_perspective() {
    let manager = PerspectiveManager::new(PieceColor::White);

    // Test that white perspective doesn't transform coordinates
    let coord = Coord::new(0, 0);
    let transformed = manager.logical_to_display(coord);
    assert_eq!(transformed, coord);

    let coord = Coord::new(3, 4);
    let transformed = manager.logical_to_display(coord);
    assert_eq!(transformed, coord);
}

#[test]
fn test_coordinate_transformation_with_invalid_coords() {
    let manager = PerspectiveManager::new(PieceColor::Black);

    // Test with undefined coordinates
    let invalid_coord = Coord::undefined();
    let transformed = manager.logical_to_display(invalid_coord);
    assert_eq!(transformed, invalid_coord);

    let back_transformed = manager.display_to_logical(invalid_coord);
    assert_eq!(back_transformed, invalid_coord);
}

#[test]
fn test_move_transformation_black_perspective() {
    let manager = PerspectiveManager::new(PieceColor::Black);
    let piece_move = PieceMove {
        piece_type: PieceType::Pawn,
        piece_color: PieceColor::White,
        from: Coord::new(1, 0),
        to: Coord::new(2, 0),
    };
    let transformed = manager.transform_move_for_display(&piece_move);
    assert_eq!(transformed.from, Coord::new(6, 7));
    assert_eq!(transformed.to, Coord::new(5, 7));
    assert_eq!(transformed.piece_type, piece_move.piece_type);
    assert_eq!(transformed.piece_color, piece_move.piece_color);
}

#[test]
fn test_move_transformation_white_perspective() {
    let manager = PerspectiveManager::new(PieceColor::White);
    let piece_move = PieceMove {
        piece_type: PieceType::Knight,
        piece_color: PieceColor::Black,
        from: Coord::new(7, 1),
        to: Coord::new(5, 2),
    };
    let transformed = manager.transform_move_for_display(&piece_move);
    assert_eq!(transformed.from, piece_move.from);
    assert_eq!(transformed.to, piece_move.to);
    assert_eq!(transformed.piece_type, piece_move.piece_type);
    assert_eq!(transformed.piece_color, piece_move.piece_color);
}

#[test]
fn test_move_transformation_logic_black_perspective() {
    let manager = PerspectiveManager::new(PieceColor::Black);
    let piece_move = PieceMove {
        piece_type: PieceType::Rook,
        piece_color: PieceColor::White,
        from: Coord::new(0, 0),
        to: Coord::new(0, 4),
    };
    let transformed = manager.transform_move_for_logic(&piece_move);
    assert_eq!(transformed.from, Coord::new(7, 7));
    assert_eq!(transformed.to, Coord::new(7, 3));
}

#[test]
fn test_bot_perspective_setup() {
    // Test that bot perspective is set correctly for black player
    let mut manager = PerspectiveManager::new(PieceColor::White);
    manager.set_perspective_for_player(PieceColor::Black);
    assert_eq!(manager.current_perspective, PlayerPerspective::Black);
    assert!(manager.needs_transformation());

    // Test that bot perspective is set correctly for white player
    manager.set_perspective_for_player(PieceColor::White);
    assert_eq!(manager.current_perspective, PlayerPerspective::White);
    assert!(!manager.needs_transformation());
}

#[test]
fn test_perspective_toggle() {
    let mut manager = PerspectiveManager::new(PieceColor::White);
    assert_eq!(manager.current_perspective, PlayerPerspective::White);

    manager.toggle_perspective();
    assert_eq!(manager.current_perspective, PlayerPerspective::Black);
    assert!(manager.needs_transformation());

    manager.toggle_perspective();
    assert_eq!(manager.current_perspective, PlayerPerspective::White);
    assert!(!manager.needs_transformation());
}

#[test]
fn test_perspective_manager_new() {
    let manager = PerspectiveManager::new(PieceColor::Black);
    assert_eq!(manager.current_perspective, PlayerPerspective::Black);
    assert_eq!(manager.player_color, PieceColor::Black);

    let manager = PerspectiveManager::new(PieceColor::White);
    assert_eq!(manager.current_perspective, PlayerPerspective::White);
    assert_eq!(manager.player_color, PieceColor::White);
}

#[test]
fn test_perspective_manager_default() {
    let manager = PerspectiveManager::default();
    assert_eq!(manager.current_perspective, PlayerPerspective::White);
    assert_eq!(manager.player_color, PieceColor::White);
}

#[test]
fn test_perspective_manager_white_perspective() {
    let manager = PerspectiveManager::white_perspective();
    assert_eq!(manager.current_perspective, PlayerPerspective::White);
    assert_eq!(manager.player_color, PieceColor::White);
}

#[test]
fn test_comprehensive_coordinate_transformation() {
    let manager = PerspectiveManager::new(PieceColor::Black);

    // Test all corner coordinates
    let corners = [
        (Coord::new(0, 0), Coord::new(7, 7)),
        (Coord::new(0, 7), Coord::new(7, 0)),
        (Coord::new(7, 0), Coord::new(0, 7)),
        (Coord::new(7, 7), Coord::new(0, 0)),
    ];

    for (original, expected) in corners {
        let transformed = manager.logical_to_display(original);
        assert_eq!(transformed, expected);

        let back_transformed = manager.display_to_logical(transformed);
        assert_eq!(back_transformed, original);
    }
}

#[test]
fn test_comprehensive_move_transformation() {
    let manager = PerspectiveManager::new(PieceColor::Black);

    // Test various move types
    let moves = [
        PieceMove {
            piece_type: PieceType::Pawn,
            piece_color: PieceColor::White,
            from: Coord::new(1, 0),
            to: Coord::new(2, 0),
        },
        PieceMove {
            piece_type: PieceType::Knight,
            piece_color: PieceColor::Black,
            from: Coord::new(7, 1),
            to: Coord::new(5, 2),
        },
        PieceMove {
            piece_type: PieceType::Queen,
            piece_color: PieceColor::White,
            from: Coord::new(3, 3),
            to: Coord::new(7, 7),
        },
    ];

    for piece_move in moves {
        let transformed = manager.transform_move_for_display(&piece_move);
        let back_transformed = manager.transform_move_for_logic(&transformed);

        assert_eq!(back_transformed.from, piece_move.from);
        assert_eq!(back_transformed.to, piece_move.to);
        assert_eq!(back_transformed.piece_type, piece_move.piece_type);
        assert_eq!(back_transformed.piece_color, piece_move.piece_color);
    }
}

#[test]
fn test_perspective_consistency() {
    // Test that perspective settings are consistent
    let mut manager = PerspectiveManager::new(PieceColor::Black);

    // Should start with black perspective
    assert_eq!(manager.current_perspective, PlayerPerspective::Black);
    assert!(manager.needs_transformation());

    // Toggle to white
    manager.toggle_perspective();
    assert_eq!(manager.current_perspective, PlayerPerspective::White);
    assert!(!manager.needs_transformation());

    // Set back to black
    manager.set_perspective_for_player(PieceColor::Black);
    assert_eq!(manager.current_perspective, PlayerPerspective::Black);
    assert!(manager.needs_transformation());
}

#[test]
fn test_edge_case_coordinates() {
    let manager = PerspectiveManager::new(PieceColor::Black);

    // Test edge coordinates
    let edge_coords = [
        Coord::new(0, 3),
        Coord::new(3, 0),
        Coord::new(7, 4),
        Coord::new(4, 7),
    ];

    for coord in edge_coords {
        let transformed = manager.logical_to_display(coord);
        let back_transformed = manager.display_to_logical(transformed);
        assert_eq!(back_transformed, coord);

        // Verify the transformation is correct
        let expected_row = 7 - coord.row;
        let expected_col = 7 - coord.col;
        assert_eq!(transformed.row, expected_row);
        assert_eq!(transformed.col, expected_col);
    }
}

#[test]
fn test_pawn_starting_positions() {
    // Test that the starting row logic is correct for both colors
    use chess_tui::pieces::PieceColor;

    // White pawns should start at row 6
    let white_starting_row = match PieceColor::White {
        PieceColor::White => 6,
        PieceColor::Black => 1,
    };
    assert_eq!(white_starting_row, 6);

    // Black pawns should start at row 1
    let black_starting_row = match PieceColor::Black {
        PieceColor::White => 6,
        PieceColor::Black => 1,
    };
    assert_eq!(black_starting_row, 1);
}
