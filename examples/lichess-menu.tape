Output examples/lichess-menu.gif

Require echo

Set BorderRadius 10
Set Shell zsh

Set FontSize 14
Set Width 2000
Set Height 1300

Set WindowBar Colorful
Set WindowBarSize 40

Type "cargo run" Sleep 500ms  Enter

Sleep 2s

# Navigate to "Play on Lichess" (index 1)
Down @0.5s
Space @0.5s

Sleep 2s

# Navigate through Lichess menu options
# Show "Seek Game" option
Sleep 1s

Down @0.5s
Sleep 1s

Down @0.5s
Sleep 1s

Down @0.5s
Sleep 1s

Up @0.5s
Sleep 1s

# Show the menu for a bit longer to see the stats panel
Sleep 3s

# Press Esc to go back to home menu
Type Esc

Sleep 2s
