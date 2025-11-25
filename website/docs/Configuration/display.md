---
id: display
title: Display Mode
sidebar_position: 2
---

import DefaultBoard from '@site/static/img/default-display-mode-board.png';
import AsciiBoard from '@site/static/img/ascii-display-mode-board.png';

# Display Mode

Chess-tui supports two display modes for rendering the chess pieces:

## Default Mode
```toml
display_mode = "DEFAULT"
```
Uses Unicode chess pieces for a richer visual experience.

<div style={{ textAlign: 'center', marginBottom: '20px' }}>
    <img src={DefaultBoard} alt="Default display mode" style={{ maxWidth: '500px' }}/>
    <p><em>Default mode with Unicode chess pieces</em></p>
</div>

## ASCII Mode
```toml
display_mode = "ASCII"
```
Uses ASCII characters for better compatibility with terminals that don't support Unicode.

<div style={{ textAlign: 'center', marginBottom: '20px' }}>
    <img src={AsciiBoard} alt="ASCII display mode" style={{ maxWidth: '500px' }}/>
    <p><em>ASCII mode for better compatibility</em></p>
</div>

You can toggle between display modes in-game using the menu option or by editing the configuration file.

## Custom Skins

When using custom color skins (see [Skins](/docs/Configuration/skins)), the display mode is automatically set to `CUSTOM`. Custom skins allow you to personalize the board colors, piece colors, and UI element colors while still using Unicode chess pieces.

:::tip
Use ASCII mode if you experience display issues with the default Unicode pieces in your terminal.
::: 