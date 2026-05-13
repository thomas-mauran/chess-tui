Output examples/lichess-menu.gif

Require echo

Set BorderRadius 10
Set Shell zsh

Set FontSize 14
Set Width 2000
Set Height 1300

Set WindowBar Colorful
Set WindowBarSize 40

Type "cargo run -- --no-sound --depth 10 --difficulty hard --skin Default" Sleep 100ms  Enter

Sleep 1.5s

# Navigate to "Play on Lichess" (index 1)
Down @0.5s
Space @0.5s

Sleep 1.5s

# Navigate through Lichess menu options
# Show "Seek Game" option
Sleep 0.8s

Down @0.5s
Sleep 0.8s

Down @0.5s
Sleep 0.8s

Down @0.5s
Sleep 0.8s

Up @0.5s
Sleep 0.8s

# Show the menu for a bit longer to see the stats panel
Sleep 2s

# Press Esc to go back to home menu
Type Esc

Sleep 1.5s
