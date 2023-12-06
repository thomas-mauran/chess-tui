<div align="center">
<h1>chess-tui</h1>
A chess TUI implementation in rust 🦀
</div>

![board](./examples/demo.gif)

![GitHub CI](https://github.com/thomas-mauran/chess-tui/actions/workflows/flow_test_build_push.yml/badge.svg)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![GitHub release](https://img.shields.io/github/v/release/thomas-mauran/chess-tui?color=success)](https://github.com/thomas-mauran/chess-tui/releases/latest)

### Demo

**With docker**

```bash
docker run --rm -it ghcr.io/thomas-mauran/chess-tui:main
```

**With Cargo**

```
cargo install chess-tui
chess-tui
```

**With Github**

```
git clone git@github.com:thomas-mauran/chess-tui.git
cd chess-tui
cargo build --release
./target/release/chess-tui
```

**NetBSD**

On NetBSD a pre-compiled binary is available from the official repositories. To install it, simply run:

```
pkgin install chess-tui
```

### Features

<details>
  <summary>Helper menu</summary>
  <img src="./examples/helper.gif" alt="Helper menu" />
</details>

<details>
  <summary>Piece moves and checkmate</summary>
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

### Roadmap

You can find the roadmap of the project [here](https://github.com/users/thomas-mauran/projects/4) if you want to contribute.

### Crates.io

The project is also available on crates.io [here](https://crates.io/crates/chess-tui)
