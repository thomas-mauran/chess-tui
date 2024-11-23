<div align="center">
<h1>chess-tui</h1>
A chess TUI implementation in rust 🦀

![board](./examples/play_against_white_bot.gif)

<div>

  ![GitHub CI](https://github.com/thomas-mauran/chess-tui/actions/workflows/flow_test_build_push.yml/badge.svg)[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)[![GitHub release](https://img.shields.io/github/v/release/thomas-mauran/chess-tui?color=success)](https://github.com/thomas-mauran/chess-tui/releases/latest)
  </div>
</div>

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
  <img src="./examples/demo.gif" alt="Helper menu" />
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
  <summary>Play against any UCI chess engine as black or white</summary>
  <h3>Play the white pieces</h3>
  <img src="./examples/play_against_white_bot.gif" alt="Play against a chess engine as white" />

  <h3>Play the black pieces</h3>
  <img src="./examples/play_against_black_bot.gif" alt="Play against a chess engine as black" />
</details>

### Connect a chess engine

You can play chess-tui with any UCI compatible chess engines. To do so you will need to use the -e command to give the chess engine binary path.

Example:

```bash
chess-tui -e /your/bin/path
```

Here I installed stockfish using homebrew and gave chess-tui the path the the engine binary.
This command will store in your home directory the chess engine path so you don't have to relink it everytime !

### Documentation

You can find the documentation of the project [here](https://thomas-mauran.github.io/chess-tui/docs/intro)

### Roadmap

You can find the roadmap of the project [here](https://github.com/users/thomas-mauran/projects/4) if you want to contribute.

### Crates.io

The project is also available on crates.io [here](https://crates.io/crates/chess-tui)