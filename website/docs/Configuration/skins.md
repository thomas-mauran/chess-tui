---
id: skins
title: Skins
sidebar_position: 5
---

# Skins

Chess-tui supports custom color skins that allow you to personalize the appearance of the chess board, pieces, and UI elements.

## File Location

Skins are configured in a JSON file located at `CONFIG_DIR/chess-tui/skins.json`. This file contains an array of skin definitions.

CONFIG_DIR is typically:
- Linux: $XDG_CONFIG_HOME or $HOME/.config
- macOS: $HOME/Library/Application Support
- Windows: `%APPDATA%` (Roaming AppData folder)


## Using Skins

### Cycling Through Skins

You can cycle through available skins in two ways:

1. **From the Home Menu**: Navigate to the "Skin" menu item (use arrow keys) and press:
   - **Space** or **Enter** to cycle forward
   - **Left Arrow** (or 'h') to cycle backward
   - **Right Arrow** (or 'l') to cycle forward

2. **During Gameplay**: Press **'s'** to cycle through skins at any time

### Built-in Display Modes

Chess-tui includes two built-in display modes that are always available:

- **Default**: Uses Unicode chess pieces with default colors
- **ASCII**: Uses ASCII characters for better terminal compatibility

These are always the first two options when cycling through skins.

## Skin Configuration Format

The `skins.json` file follows this structure:

```json
{
    "skins": [
        {
            "name": "Skin Name",
            "board_white_color": "color",
            "board_black_color": "color",
            "piece_white_color": "color",
            "piece_black_color": "color",
            "cursor_color": "color",
            "selection_color": "color",
            "last_move_color": "color"
        }
    ]
}
```

## Field Descriptions

| Field | Type | Description |
|-------|------|-------------|
| `name` | string | Display name of the skin (shown in UI) |
| `board_white_color` | color | Color for light squares on the board |
| `board_black_color` | color | Color for dark squares on the board |
| `piece_white_color` | color | Color for white pieces |
| `piece_black_color` | color | Color for black pieces |
| `cursor_color` | color | Color for the cursor highlight |
| `selection_color` | color | Color for selected piece highlight |
| `last_move_color` | color | Color for last move highlight |

## Color Formats

Colors can be specified in two ways:

### 1. Named Colors (String)

Use standard terminal color names:

- Basic colors: `"Black"`, `"Red"`, `"Green"`, `"Yellow"`, `"Blue"`, `"Magenta"`, `"Cyan"`, `"White"`
- Gray shades: `"Gray"`, `"DarkGray"`
- Light variants: `"LightRed"`, `"LightGreen"`, `"LightYellow"`, `"LightBlue"`, `"LightMagenta"`, `"LightCyan"`

Example:
```json
"board_white_color": "LightGray"
```

### 2. RGB Values (Object)

Use RGB values (0-255 for each component):

```json
"board_white_color": {"Rgb": [240, 217, 181]}
```

## Complete Example

Here's a complete example with multiple skins:

```json
{
    "skins": [
        {
            "name": "Ocean",
            "board_white_color": {"Rgb": [200, 220, 235]},
            "board_black_color": {"Rgb": [100, 140, 170]},
            "piece_white_color": {"Rgb": [245, 250, 255]},
            "piece_black_color": {"Rgb": [50, 80, 120]},
            "cursor_color": {"Rgb": [150, 200, 230]},
            "selection_color": {"Rgb": [120, 180, 220]},
            "last_move_color": {"Rgb": [100, 160, 200]}
        },
        {
            "name": "Forest",
            "board_white_color": {"Rgb": [200, 230, 200]},
            "board_black_color": {"Rgb": [100, 150, 100]},
            "piece_white_color": {"Rgb": [250, 255, 250]},
            "piece_black_color": {"Rgb": [60, 100, 60]},
            "cursor_color": {"Rgb": [150, 220, 150]},
            "selection_color": {"Rgb": [120, 200, 120]},
            "last_move_color": {"Rgb": [200, 220, 120]}
        },
        {
            "name": "Retro",
            "board_white_color": "White",
            "board_black_color": "Black",
            "piece_white_color": {"Rgb": [100, 150, 255]},
            "piece_black_color": {"Rgb": [200, 80, 80]},
            "cursor_color": {"Rgb": [255, 200, 0]},
            "selection_color": {"Rgb": [100, 200, 100]},
            "last_move_color": {"Rgb": [150, 200, 150]}
        }
    ]
}
```

## Tips for Creating Skins

- **Contrast**: Ensure good contrast between board squares and piece colors for visibility
- **Readability**: Make sure pieces are clearly visible on both light and dark squares
- **Cursor Visibility**: Choose cursor and selection colors that stand out on both square types
- **Color Harmony**: Use a cohesive color palette for a pleasing aesthetic
- **Testing**: Test your colors in different terminal emulators as color rendering can vary

## Default Skins

Chess-tui comes with several pre-configured skins:

- **Default**: Classic beige and brown board
- **Matrix**: Dark theme with cyan and green accents
- **Ocean**: Calming blue tones
- **Forest**: Nature-inspired green palette
- **Sunset**: Warm orange and peach colors
- **Midnight**: Dark theme with purple tones
- **Classic**: Traditional wooden chess board colors
- **Neon**: Vibrant colors on dark background
- **Retro**: Black and white board with colored pieces

These skins are included in the `skins.json` file in the repository. You can modify them or add your own custom skins to the file.

