//! Deserialization and board setup for the two puzzle payload shapes seen in the wild.
//!
//! lichess.org sends a game PGN and a clock; a self-hosted lila whose puzzle database
//! has no game replay sends an empty PGN, no clock, and the puzzle's own FEN instead.

use chess_tui::game_logic::game_board::GameBoard;
use chess_tui::game_logic::puzzle::PuzzleGame;
use chess_tui::lichess::models::{OngoingGame, Puzzle};
use shakmaty::Position;

/// Captured verbatim from a self-hosted instance: no `game.clock`, empty `game.pgn`,
/// and a `puzzle.fen` that is the only way to set up the board.
const SELF_HOSTED_PUZZLE: &str = r#"{
  "game": {
    "id": "xMCTSnVy",
    "perf": {"key": "blitz", "name": "Blitz"},
    "rated": true,
    "players": [
      {"name": "ghost", "id": "ghost", "color": "white"},
      {"name": "ghost", "id": "ghost", "color": "black"}
    ],
    "pgn": ""
  },
  "puzzle": {
    "id": "Jj0di",
    "rating": 1527,
    "plays": 3494,
    "solution": ["b2b4", "a5b5", "c3c4", "b5c4", "a3a4"],
    "themes": ["crushing", "deflection", "endgame", "long"],
    "fen": "r7/8/p7/k2P1p2/q3p3/Q1P5/1P3P1P/7K w - - 1 1",
    "lastMove": "b5a4",
    "initialPly": 79
  }
}"#;

/// Captured verbatim from lichess.org: full PGN and a clock, no FEN.
const LICHESS_ORG_PUZZLE: &str = r#"{
  "game": {
    "id": "1tYAcSIe",
    "perf": {"key": "rapid", "name": "Rapid"},
    "rated": true,
    "players": [
      {"name": "CHESS-NO-MESS", "id": "chess-no-mess", "color": "white", "rating": 1837},
      {"name": "CheerUp27", "id": "cheerup27", "color": "black", "rating": 1852}
    ],
    "pgn": "Nf3 e5 e4 Nc6 Bc4 Nf6 Nc3 Bb4 d3 O-O O-O Nd4 Nd5 Nxd5 Bxd5 c6 Bxf7+ Rxf7 Nxe5 Re7 f4 Bc5 Kh1 d6 Nf3 Nxf3 Qxf3 Be6 f5 Rf7 Qg3 Bd7 Bg5 Qe8 f6 g6 Bh6 Bd4 Bg7 Qe6 Qh4 Bxb2 Rab1 Qxa2 c4 Bd4 Rxb7 Qe2 Rbb1 Qxd3 Rbd1 Qxc4 e5 dxe5 Rc1 Qe6 Rce1 a5 Qh6 a4 Re4 Qc4",
    "clock": "10+0"
  },
  "puzzle": {
    "id": "yM2Xo",
    "rating": 1404,
    "plays": 23047,
    "solution": ["h6h7", "g8h7", "e4h4", "h7g8", "h4h8"],
    "themes": ["middlegame", "operaMate", "attraction", "long", "mateIn3", "sacrifice"],
    "initialPly": 61
  }
}"#;

#[test]
fn self_hosted_puzzle_without_clock_or_pgn_deserializes() {
    let puzzle: Puzzle =
        serde_json::from_str(SELF_HOSTED_PUZZLE).expect("missing clock must not fail the decode");

    assert_eq!(puzzle.game.clock, None);
    assert_eq!(puzzle.game.pgn, "");
    assert_eq!(
        puzzle.puzzle.fen.as_deref(),
        Some("r7/8/p7/k2P1p2/q3p3/Q1P5/1P3P1P/7K w - - 1 1")
    );
    assert_eq!(puzzle.puzzle.last_move.as_deref(), Some("b5a4"));
}

#[test]
fn lichess_org_puzzle_still_deserializes() {
    let puzzle: Puzzle = serde_json::from_str(LICHESS_ORG_PUZZLE).expect("decode");

    assert_eq!(puzzle.game.clock.as_deref(), Some("10+0"));
    assert!(puzzle.game.pgn.starts_with("Nf3 e5"));
    assert_eq!(puzzle.puzzle.fen, None);
}

/// The FEN branch sets the board straight from `puzzle.fen`, with no history to replay.
#[test]
fn empty_pgn_sets_the_board_from_the_puzzle_fen() {
    let puzzle: Puzzle = serde_json::from_str(SELF_HOSTED_PUZZLE).expect("decode");
    let mut game_board = GameBoard::default();

    let expected = shakmaty::fen::Fen::from_ascii(b"r7/8/p7/k2P1p2/q3p3/Q1P5/1P3P1P/7K w - - 1 1")
        .expect("fixture fen parses")
        .into_position::<shakmaty::Chess>(shakmaty::CastlingMode::Standard)
        .expect("fixture fen is a legal standard position");

    PuzzleGame::setup_board(puzzle, &mut game_board).expect("fen branch must succeed");

    assert_eq!(
        game_board.position_history.len(),
        1,
        "the instance sent no move history, so there is nothing to step back through"
    );
    assert!(game_board.move_history.is_empty());
    assert_eq!(game_board.history_position_index, None);
    assert_eq!(
        game_board.position_history[0].turn(),
        shakmaty::Color::White
    );
    assert_eq!(game_board.position_history[0].board(), expected.board());
}

/// The PGN branch is unchanged: the whole game is replayed into the history.
#[test]
fn lichess_org_pgn_still_replays_into_history() {
    let puzzle: Puzzle = serde_json::from_str(LICHESS_ORG_PUZZLE).expect("decode");
    let mut game_board = GameBoard::default();

    PuzzleGame::setup_board(puzzle, &mut game_board).expect("pgn branch must succeed");

    // 62 SAN moves in the fixture, plus the starting position.
    assert_eq!(game_board.move_history.len(), 62);
    assert_eq!(game_board.position_history.len(), 63);
}

/// Neither shape available is a clear error, not a silent start-position board.
#[test]
fn neither_pgn_nor_fen_is_a_named_error() {
    let raw = SELF_HOSTED_PUZZLE.replace(
        "\"fen\": \"r7/8/p7/k2P1p2/q3p3/Q1P5/1P3P1P/7K w - - 1 1\",\n    ",
        "",
    );
    let puzzle: Puzzle = serde_json::from_str(&raw).expect("decode");
    assert_eq!(puzzle.puzzle.fen, None, "the fen must actually be gone");

    let mut game_board = GameBoard::default();
    let err = PuzzleGame::setup_board(puzzle, &mut game_board)
        .expect_err("no pgn and no fen has no position to set up");
    assert!(err.contains("Jj0di"), "{err}");
    assert!(err.contains("neither a game PGN nor a FEN"), "{err}");
}

/// A Horde game reaches the ongoing-games list with a variant key, and its FEN is not a
/// legal standard-chess position - which is why the variant has to be checked first.
#[test]
fn horde_ongoing_game_is_flagged_and_its_fen_is_unplayable() {
    let raw = r#"{
      "fullId": "ZlERnWEOkxyb",
      "gameId": "ZlERnWEO",
      "fen": "rnbqkbnr/1p1p1p2/pP1P4/PPPPPPPP/PPPPPPPP/PPP1P1P1/2P1PPP1/PP3PPP b q - 0 35",
      "color": "black",
      "variant": {"key": "horde", "name": "Horde"},
      "opponent": {"id": null, "username": "Stockfish level 8", "ai": 8},
      "isMyTurn": true
    }"#;

    let game: OngoingGame = serde_json::from_str(raw).expect("decode");
    let variant = game
        .variant
        .as_ref()
        .expect("horde game reports its variant");
    assert!(!variant.is_standard());
    assert_eq!(variant.name, "Horde");

    let position = shakmaty::fen::Fen::from_ascii(game.fen.as_bytes())
        .expect("the fen itself is syntactically valid")
        .into_position::<shakmaty::Chess>(shakmaty::CastlingMode::Standard);
    assert!(
        position.is_err(),
        "a Horde position must not parse as standard chess"
    );
}

/// A game with no `variant` field is standard, which is how the API omits it.
#[test]
fn missing_variant_is_treated_as_standard() {
    let raw = r#"{
      "fullId": "abcdefghijkl",
      "gameId": "abcdefgh",
      "fen": "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
      "color": "white",
      "opponent": {"id": "someone", "username": "someone", "rating": 1500},
      "isMyTurn": true
    }"#;

    let game: OngoingGame = serde_json::from_str(raw).expect("decode");
    assert!(game.variant.is_none());
    assert_eq!(
        chess_tui::lichess::models::unsupported_variant_name(game.variant.as_ref()),
        None
    );
}
