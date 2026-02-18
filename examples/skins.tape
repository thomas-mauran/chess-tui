Output examples/skins.gif

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

# Start a normal game: Play Game (already selected) -> Local Game -> Start
Space @0.5s
Sleep 0.7s

# Select Local Game (index 0)
Space @0.5s
Sleep 0.7s

# Start game (default settings)
Space @0.5s
Sleep 1s

# Make a move so we see the board clearly
Down @0.5s
Down @0.5s
Space @0.5s
Sleep 0.5s
Space @0.5s
Sleep 1s

# Cycle through skins during gameplay using 's' key
Type "s"
Sleep 0.8s

Type "s"
Sleep 0.8s

Type "s"
Sleep 0.8s

Type "s"
Sleep 0.8s

Type "s"
Sleep 0.8s

Type "s"
Sleep 0.8s

Type "s"
Sleep 0.8s

Type "s"
Sleep 0.8s

Type "s"
Sleep 0.8s

Type "s"
Sleep 0.8s

Type "s"
Sleep 0.8s

Type "s"
Sleep 0.8s

Type "s"
Sleep 0.8s

# Show final skin
Sleep 1.5s
