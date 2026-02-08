#[test]
fn test_normalize_lowercase_to_san() {
    // possible lower case first letters
    let moves = vec!["ne4", "be4", "re4", "qe4", "ke4"];
    let expected = vec!["Ne4", "Be4", "Re4", "Qe4", "Ke4"];

    for (&input, &expected) in moves.iter().zip(expected.iter()) {
        assert_eq!(
            chess_tui::utils::normalize_lowercase_to_san(input),
            expected
        );
    }

    // check for castling
    let moves = vec!["o-o", "o-o-o"];
    let expected = vec!["O-O", "O-O-O"];
    for (&input, &expected) in moves.iter().zip(expected.iter()) {
        assert_eq!(
            chess_tui::utils::normalize_lowercase_to_san(input),
            expected
        );
    }

    // check for promotion
    let moves = vec!["e8=q", "e8=r", "e8=b", "e8=n"];
    let expected = vec!["e8=Q", "e8=R", "e8=B", "e8=N"];
    for (&input, &expected) in moves.iter().zip(expected.iter()) {
        assert_eq!(
            chess_tui::utils::normalize_lowercase_to_san(input),
            expected
        );
    }

    // check empty string
    assert_eq!(chess_tui::utils::normalize_lowercase_to_san("                       "), "");
}
