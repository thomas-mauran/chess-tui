<div align="center">
<h1>chess-tui</h1>
A rusty chess game in your terminal 🦀

![board](./examples/play_against_white_bot.gif)

<div>

  ![GitHub CI](https://github.com/thomas-mauran/chess-tui/actions/workflows/flow_test_build_push.yml/badge.svg)[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)[![GitHub release](https://img.shields.io/github/v/release/thomas-mauran/chess-tui?color=success)](https://github.com/thomas-mauran/chess-tui/releases/latest)
  </div>
</div>

### Description

Chess-tui is a simple chess game you can play from your terminal. It supports local 2 players mode, online multiplayer, playing against any UCI compatible chess engine, custom skins and more !

### Quick install

```bash
cargo install chess-tui
chess-tui
```

If you want to install the game with your favorite package manager, you can find the installation guide [here](https://thomas-mauran.github.io/chess-tui/docs/Installation/Packaging%20status).

### Available on
[![Packaging status](https://repology.org/badge/vertical-allrepos/chess-tui.svg)](https://repology.org/project/chess-tui/versions)

### Features

<details>
  <summary>Helper menu</summary>
  <img src="./examples/helper.gif" alt="Helper menu" />
</details>
<details>
  <summary>Local 2 player mode</summary>
  <img src="./examples/demo.gif" alt="Local 2 players" />
</details>
<details>
  <summary>Online multiplayer</summary>
  <img src="./website/static/gif/multiplayer.gif" alt="Online multiplayer" />
</details>
<details>
  <summary>Draws</summary>
  <ul>
  <li>Stalemate</li>
  <li>50 moves rules</li>
  <li>3 time repetition of the same position</li>
  </ul>
</details>
<details>
  <summary>Piece Promotion</summary>
  no demo available yet
</details>
<details>
  <summary>Move History Navigation (Solo Mode)</summary>
  Navigate through move history with <strong>P</strong> (previous) and <strong>N</strong> (next) keys. Make moves from any historical position to create alternate game branches.
</details>
<details>
  <summary>Play against any UCI chess engine as black or white</summary>
  <h3>Play the white pieces</h3>
  <img src="./examples/play_against_white_bot.gif" alt="Play against a chess engine as white" />

  <h3>Play the black pieces</h3>
  <img src="./examples/play_against_black_bot.gif" alt="Play against a chess engine as black" />
</details>
<details>
  <summary>Lichess Integration</summary>
  <p>Play online on Lichess directly from your terminal! Features include:</p>
  <ul>
    <li><strong>Seek Games:</strong> Find opponents for correspondence games (3 days per move)</li>
    <li><strong>Join by ID:</strong> Join specific games using a game code</li>
    <li><strong>Ongoing Games:</strong> View and resume all your active Lichess games</li>
    <li><strong>Puzzles:</strong> Solve rated chess puzzles to improve your skills</li>
    <li><strong>Profile Stats:</strong> View your ratings, game statistics, and performance charts</li>
  </ul>
  <p>To use Lichess features, you need an API token. Get one at <a href="https://lichess.org/account/oauth/token">lichess.org/account/oauth/token</a> and configure it with:</p>
  <pre><code>chess-tui -l YOUR_LICHESS_TOKEN_HERE</code></pre>
  <img src="./examples/lichess-menu.gif" alt="Lichess menu" />

</details>


### Connect a chess engine

You can play chess-tui with any UCI compatible chess engines. To do so you will need to use the -e command to give the chess engine binary path.

Example:

```bash
chess-tui -e /your/bin/path
```

Here I installed stockfish using homebrew and gave chess-tui the path the the engine binary.
This command will store in your home directory the chess engine path so you don't have to relink it everytime !

You can also set the bot thinking depth directly from the command line:

```bash
chess-tui -e /your/bin/path --depth 15
```

This will set the bot to think at depth 15 for this session.

### Configuration

Chess-tui uses a TOML configuration file located at `~/.config/chess-tui/config.toml`. Here are the available configuration options:

```toml
# Path to the chess engine binary
engine_path = "/path/to/engine"

# Display mode: "DEFAULT" or "ASCII"
display_mode = "DEFAULT"

# Logging level: "Off", "Error", "Warn", "Info", "Debug", "Trace"
log_level = "Off"

# Bot thinking depth for chess engine (1-255, default: 10)
bot_depth = 10

# Lichess API token for online features
lichess_token = "YOUR_LICHESS_TOKEN_HERE"
```

#### Configuration Options:

- **engine_path**: Path to your UCI-compatible chess engine binary
- **display_mode**:
  - `DEFAULT`: Uses unicode chess pieces
  - `ASCII`: Uses ASCII characters for pieces
- **log_level**: Controls the verbosity of logging
  - `Off`: No logging (default)
  - `Error`: Only errors
  - `Warn`: Warnings and errors
  - `Info`: General information, warnings and errors
  - `Debug`: Debugging information
  - `Trace`: Very verbose debugging information
- **bot_depth**: Controls how deep the chess engine thinks (1-255, default: 10)
  - Higher values make the bot stronger but slower
  - Lower values make the bot faster but weaker
- **lichess_token**: Your Lichess API token for online features
  - Get your token at [lichess.org/account/oauth/token](https://lichess.org/account/oauth/token)
  - Can also be set via command line: `chess-tui -l your_token_here`

The config file is automatically created when you first run chess-tui. You can manually edit it to customize your experience.

All logs are stored in `~/.config/chess-tui/logs`.

### Game Controls

**Basic Controls:**
- Arrow keys or `h/j/k/l`: Move cursor
- `Space` or `Enter`: Select/move piece
- `Esc`: Deselect piece or close popups
- `?`: Show help menu
- `q`: Quit game
- `b`: Return to home menu
- `s`: Cycle through available skins
- `r`: Restart current game
- `p`: Navigate to previous position in history
- `n`: Navigate to next position in history

Base config:
```toml
# no engine path
display_mode = "DEFAULT"
log_level = "Off"
bot_depth = 10
```

### Documentation

You can find the documentation of the project [here](https://thomas-mauran.github.io/chess-tui/docs/intro)

### Roadmap

You can find the roadmap of the project [here](https://github.com/users/thomas-mauran/projects/4) if you want to contribute.

### Crates.io

The project is also available on crates.io [here](https://crates.io/crates/chess-tui)
