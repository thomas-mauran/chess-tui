---
id: keyboard-shortcuts
title: Keyboard Shortcuts
sidebar_position: 1
description: "Complete keyboard shortcut reference for Chess TUI - navigation, piece moves, and game controls."
---

# Keyboard Shortcuts

Press **?** at any time during gameplay to see context-sensitive controls in the help popup.

## Universal

| Key | Action |
|-----|--------|
| **?** | Open help popup |
| **q** | Quit the application |
| **Esc** | Close popup |

## Main Menu

| Key | Action |
|-----|--------|
| **↑ / ↓** or **k / j** | Navigate menu items |
| **Enter** or **Space** | Confirm selection |
| **← / →** or **h / l** | Cycle skins (on skin row) |

## In-Game

| Key | Action |
|-----|--------|
| **↑ ↓ ← →** or **h j k l** | Move cursor |
| **Enter** or **Space** | Select piece / confirm move |
| **Esc** | Deselect piece |
| **b** | Return to home menu |
| **r** | Restart game (solo mode) |
| **s** | Cycle skins |
| **p / P** | Previous position in history (solo mode) |
| **n / N** | Next position in history (solo mode) |
| **t / T** | Show hint (puzzle mode) |
| **Ctrl +** | Zoom in |
| **Ctrl -** | Zoom out |

:::note
The mouse is also supported for piece selection and movement during gameplay.
:::

## PGN Viewer

In the game menu, select **Load PGN** and press **Enter** to activate its configuration.
Choose **File path** or **Paste PGN** with **← / →** (or **h / l**), then press
**Enter** (or **Space**) to open the input dialog. **Esc** cancels the selector or
closes the dialog and returns control to the game menu.
Use your terminal's paste shortcut to insert PGN text, then press **Enter** to load it.
If the terminal does not support bracketed paste (such as a legacy Windows console),
an error dialog appears. Use **File path** or switch to a terminal with bracketed paste support.

| Key | Action |
|-----|--------|
| **← / h** | Previous move |
| **→ / l** | Next move |
| **g** | Jump to start |
| **G** | Jump to end |
| **Space** | Toggle auto-play |
| **+** | Increase auto-play speed |
| **-** | Decrease auto-play speed |
| **Tab** | Next game |
| **Shift+Tab** | Previous game |
| **Esc** | Return to home menu |

## Board Color Reference

| Color | Meaning |
|-------|---------|
| **Blue** | Cursor position |
| **Green** | Selected piece |
| **Purple** | King in check |
| **Grey** | Legal move targets |
| **Yellow** | Last move |
