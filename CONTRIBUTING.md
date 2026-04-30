# Chess-tui contribution guidelines

Thank you for your interest in improving Chess-tui! We'd love to have your contribution. We expect all contributors to abide by the [Rust code of conduct].

[Rust code of conduct]: https://www.rust-lang.org/policies/code-of-conduct

## License

Chess-tui is an MIT licensed project and so are all contributions. Please see the [`LICENSE`] file in this directory for more details.

[`LICENSE`]: https://github.com/thomas-mauran/chess-tui/blob/main/LICENSE

## Pull Requests

To make changes to Chess-tui, please send in pull requests on GitHub to the `main` branch. We'll review them and either merge or request changes. CI runs automatically, so you may get feedback from it too.

If you make additions or other changes to a pull request, feel free to either amend previous commits or only add new ones, however you prefer. Commits are squashed on merge.

## Issue Tracker

You can find the issue tracker [on GitHub](https://github.com/thomas-mauran/chess-tui/issues). If you've found a problem with Chess-tui, please open an issue there.

We use the following labels:

- `enhancement`: Request for new sections or functionality.
- `bug`: Something in Chess-tui that is incorrect or not working.
- `documentation`: Anything related to documentation.
- `help wanted`: Issues we'd like to fix but don't have time for ourselves. Leave a comment if you'd like to pick one up.
- `good first issue`: Good entry points for people new to the project or open source.

## Development workflow

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (stable toolchain)
- A UCI-compatible chess engine (e.g. [Stockfish](https://stockfishchess.org/)) if you want to test bot mode

### Build and run

```bash
git clone https://github.com/thomas-mauran/chess-tui
cd chess-tui

# Dev build
cargo run

# Release build
cargo build --release
./target/release/chess-tui
```

### Build without sound

If you don't have audio libraries available, disable the sound feature:

```bash
cargo run --no-default-features
```

### Testing

```bash
# Run all tests (unit + integration)
cargo test

# Run a specific test file
cargo test --test game_board_tests
```

Integration tests live in the `tests/` directory and cover game board logic, clock behaviour, and utility functions.

### Linting and formatting

CI enforces both run these before opening a PR to avoid surprises:

```bash
cargo fmt
cargo clippy -- -D warnings
```

## Project structure

| Path | Purpose |
|------|---------|
| `src/app/` | Top-level application state and lifecycle (event loop, config loading) |
| `src/state/` | Individual state structs (game mode, UI, Lichess, bot, multiplayer) |
| `src/handlers/` | Keyboard input handlers, one file per screen |
| `src/ui/` | Ratatui rendering, one file per screen / popup |
| `src/game_logic/` | Core chess logic: board, clock, puzzle, coordinate utilities |
| `src/pieces/` | Piece definitions and move generation |
| `src/lichess/` | Lichess API client (streaming, token auth) |
| `src/server/` | Local multiplayer TCP server and client |

## Bot testing

To test bot mode locally you need a UCI engine on your `PATH` or at an explicit path:

```bash
# Using Stockfish
cargo run -- -e stockfish

# Explicit path or flags (e.g. GNU Chess)
cargo run -- -e "/usr/bin/gnuchess --uci"
```

See the [Bot Configuration](https://thomas-mauran.github.io/chess-tui/docs/Configuration/bot) docs for full details.
