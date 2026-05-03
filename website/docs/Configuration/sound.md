---
id: sound
title: Sound
sidebar_position: 4
description: "Enable and configure sound effects in Chess TUI for move feedback and game events."
---

# Sound

Chess-tui plays sound effects for piece moves and menu navigation. Sound is enabled by default when an audio device is available.

## Disabling Sound

### Command Line

```bash
chess-tui --no-sound
```

### Configuration File

```toml
# CONFIG_DIR/chess-tui/config.toml
sound_enabled = false
```

## Building Without Sound

If you are building from source and don't need sound, you can compile without the audio dependency entirely:

```bash
cargo build --release --no-default-features
```

Or when installing via Cargo:

```bash
cargo install chess-tui --no-default-features
```

## Troubleshooting

### No Sound on Linux

Make sure ALSA development libraries are installed:

```bash
# Debian/Ubuntu
sudo apt-get install libasound2-dev

# Fedora
sudo dnf install alsa-lib-devel

# Arch Linux
sudo pacman -S alsa-lib
```

### No Sound in Docker / Headless Environments

Sound requires a connected audio device. In Docker containers or headless servers there is typically no audio device available. chess-tui detects this at startup and silently disables sound no error is shown. You can also explicitly disable it:

```bash
chess-tui --no-sound
```

### Sound Cuts Off or Crackles

Try lowering your system audio sample rate, or disable sound with `--no-sound`.
