---
id: bot-engines
title: Engines
sidebar_position: 6
description: "List of UCI-compatible chess engines that work with Chess TUI, including Stockfish."
---

# UCI-Compatible Engines

Any UCI-compatible chess engine works with chess-tui. Below are the most popular options.

## Stockfish Recommended ⭐

The strongest open-source engine in the world. Fully supports `UCI_LimitStrength` and `UCI_Elo`, so all [difficulty presets](bot-settings#difficulty-presets) work perfectly.

| Platform | Install |
|----------|---------|
| macOS | `brew install stockfish` |
| Debian/Ubuntu | `sudo apt install stockfish` |
| Fedora | `sudo dnf install stockfish` |
| Arch Linux | `sudo pacman -S stockfish` |
| Windows | [stockfishchess.org/download](https://stockfishchess.org/download/) |

```bash
chess-tui -e stockfish
```

## GNU Chess

Classic open-source engine. Requires the `--uci` flag. Difficulty presets work via depth/time limiting (ELO limiting not supported).

| Platform | Install |
|----------|---------|
| macOS | `brew install gnu-chess` |
| Debian/Ubuntu | `sudo apt install gnuchess` |
| Fedora | `sudo dnf install gnuchess` |
| Arch Linux | `sudo pacman -S gnuchess` |

```bash
chess-tui -e "gnuchess --uci"
```
