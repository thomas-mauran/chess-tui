---
id: faq
title: FAQ
sidebar_position: 8
description: "Frequently asked questions about Chess TUI installation, configuration, and gameplay."
head:
  - tagName: script
    attributes:
      type: application/ld+json
    innerHTML: |
      {"@context":"https://schema.org","@type":"FAQPage","mainEntity":[{"@type":"Question","name":"The board looks corrupted or shows weird characters","acceptedAnswer":{"@type":"Answer","text":"Use a Unicode-capable font like Nerd Fonts. Switch to ASCII mode with display_mode = \"ASCII\" in config.toml or press s in-game. Try a terminal with good Unicode support like iTerm2, Alacritty, Kitty, or WezTerm."}},{"@type":"Question","name":"Pieces are tiny or huge","acceptedAnswer":{"@type":"Answer","text":"Use Ctrl + and Ctrl - to zoom in and out. Piece rendering adapts automatically to the cell size."}},{"@type":"Question","name":"The bot never moves or hangs","acceptedAnswer":{"@type":"Answer","text":"Verify the engine path is valid and the binary is executable. Test with: echo \"uci\" | /path/to/engine. Some engines like GNU Chess require a --uci flag. Enable debug logging with log_level = \"DEBUG\" and check logs in CONFIG_DIR/chess-tui/logs/."}},{"@type":"Question","name":"No sound","acceptedAnswer":{"@type":"Answer","text":"Sound requires an audio device. In Docker or headless environments it is automatically disabled. Disable explicitly with --no-sound or sound_enabled = false in config."}},{"@type":"Question","name":"chess-tui does not work well on Windows","acceptedAnswer":{"@type":"Answer","text":"Use Windows Terminal for best Unicode support. Use --no-sound if sound causes issues. Ensure your terminal uses a font with Unicode chess symbol support."}},{"@type":"Question","name":"Lichess token keeps getting rejected","acceptedAnswer":{"@type":"Answer","text":"Ensure the token has correct scopes: Read preferences, Create/read/update/delete games, Read puzzle activity. Tokens can expire - generate a new one at lichess.org/account/oauth/token. Check your internet connection."}},{"@type":"Question","name":"How do I update chess-tui?","acceptedAnswer":{"@type":"Answer","text":"Homebrew: brew upgrade chess-tui. Cargo: cargo install chess-tui. Debian/Ubuntu: re-run the install one-liner from the Installation guide."}},{"@type":"Question","name":"Where is the config file?","acceptedAnswer":{"@type":"Answer","text":"Linux: $XDG_CONFIG_HOME/chess-tui/ or $HOME/.config/chess-tui/. macOS: $HOME/Library/Application Support/chess-tui/. Windows: %APPDATA%/chess-tui/. The config file is config.toml and skins file is skins.json."}}]}
---

# Frequently Asked Questions

## The board looks corrupted or shows weird characters

chess-tui uses Unicode chess symbols and box-drawing characters. If your board looks broken:

1. **Use a Unicode-capable font** Install a patched font like [Nerd Fonts](https://www.nerdfonts.com/) and set your terminal to use it.
2. **Switch to ASCII mode** If your terminal has poor Unicode support, switch the display mode in your config:
   ```toml
   # CONFIG_DIR/chess-tui/config.toml
   display_mode = "ASCII"
   ```
   Or press **s** in-game to cycle to the ASCII skin.
3. **Try a different terminal** iTerm2, Alacritty, Kitty, and WezTerm have good Unicode support.

## Pieces are tiny or huge

Use **Ctrl +** and **Ctrl -** to zoom in and out. Piece rendering automatically adapts to the cell size.

:::note
Zoom shortcuts may differ depending on your terminal. If Ctrl+/- doesn't work, try resizing the terminal window directly.
:::

## The bot never moves / hangs

1. Make sure your engine path is valid and the binary is executable.
2. Test the engine manually: `echo "uci" | /path/to/engine` you should see `uciok`.
3. Some engines like GNU Chess require a `--uci` flag:
   ```bash
   chess-tui -e "/path/to/gnuchess --uci"
   ```
4. Enable debug logging to see what's happening:
   ```toml
   log_level = "DEBUG"
   ```
   Then check `CONFIG_DIR/chess-tui/logs/`.

See [Bot Configuration](/docs/Bot) for full setup details.

## No sound

Sound requires an audio device. In Docker or headless environments it is automatically disabled. You can also explicitly disable it with `--no-sound` or `sound_enabled = false` in your config.

See [Sound](/docs/Configuration/sound) for full details.

## chess-tui doesn't work well on Windows

chess-tui is tested on Windows in CI. Known limitations:

- **Broken characters** cmd.exe and old PowerShell have poor Unicode support. Use [Windows Terminal](https://aka.ms/terminal) for the best experience.
- **Sound issues** Use `--no-sound` if sound causes problems.
- **Font** Make sure your terminal is using a font with Unicode chess symbol support.

## Lichess token keeps getting rejected

1. Make sure you generated the token with the correct scopes: **Read preferences**, **Create/read/update/delete games**, **Read puzzle activity**.
2. Tokens can expire generate a new one at [lichess.org/account/oauth/token](https://lichess.org/account/oauth/token).
3. Check your internet connection the token is validated against the Lichess API on first use.

See [Lichess Setup](/docs/Lichess/setup) for full details.

## How do I update chess-tui?

```bash
# Homebrew
brew upgrade chess-tui

# Cargo
cargo install chess-tui

# Debian/Ubuntu re-run the install one-liner from the Installation guide
```

## Where is the config file?

| Platform | Path |
|----------|------|
| Linux | `$XDG_CONFIG_HOME/chess-tui/` or `$HOME/.config/chess-tui/` |
| macOS | `$HOME/Library/Application Support/chess-tui/` |
| Windows | `%APPDATA%\chess-tui\` |

The config file is `config.toml` and the skins file is `skins.json` inside that directory.
