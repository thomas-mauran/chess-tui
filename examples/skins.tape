Output examples/skins.gif

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

# Start a normal game: Play Game (already selected) -> Local Game -> Start
Space @0.5s
Sleep 0.5s

# Select Local Game (index 0)
Space @0.5s
Sleep 0.5s

# Start game (default settings)
Space @0.5s
Sleep 0.8s

# 1. e4 (Italian Game opening) - demonstrating free cursor navigation
Down @0.5s
Down @0.5s
Space @0.5s
Sleep 0.4s
# Navigate freely on board before confirming
Left @0.2s
Right @0.2s
Up @0.2s
Space @0.5s
Sleep 0.8s

# Black's response e7-e5
Up @0.5s
Up @0.5s
Space @0.5s
Sleep 0.4s
Right @0.2s
Left @0.2s
Space @0.5s
Sleep 0.8s

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
Sleep 1s
