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

# Navigate to Lichess menu (option 2, index 2)
Down @0.3s
Down @0.3s
Space @0.3s

Sleep 2s

# Navigate through Lichess menu options
# Show "Seek Game" option
Sleep 1s

# Navigate to "Puzzle" option
Down @0.3s
Sleep 1s

# Navigate to "My Ongoing Games" option
Down @0.3s
Sleep 1s

# Navigate to "Join by Code" option
Down @0.3s
Sleep 1s

# Go back up to show "Puzzle" option
Up @0.3s
Sleep 1s

# Show the menu for a bit longer to see the stats panel
Sleep 3s

# Press Esc to go back to home menu
Type Esc

Sleep 2s
