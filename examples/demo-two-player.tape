Output examples/demo-two-player.gif

Require echo

Set BorderRadius 10
Set Shell zsh

Set FontSize 14
Set Width 2000
Set Height 1300

Set WindowBar Colorful
Set WindowBarSize 40

Type "cargo run -- --no-sound --depth 10 --difficulty hard --skin Default" Sleep 100ms  Enter

Sleep 0.5s
# Navigate to "Play Game" (index 0) - already selected, just press Enter
Space @0.5s
Sleep 1s

# Now in Game Mode Menu, "Local Game" (index 0) is selected
# Press Enter to activate form
Space @0.5s
Sleep 0.5s

# Form is now active, cursor is on Time Control field (index 0)
# Default is Rapid (index 3), let's select Blitz (index 2) to show the feature
Left @0.5s
Sleep 0.5s

# Now press Enter to start game with Blitz time control
Space @0.5s
Sleep 0.5s

# Now playing the game - play naturally with subtle free cursor navigation
# Move pawn e2-e4
Down @0.5s
Down @0.5s
Space @0.5s
Sleep 0.7s
Space @0.5s

Sleep 0.7s
# Black's move - pawn e7-e5
Left @0.5s
Space @0.5s
Sleep 0.7s
Space @0.5s

Sleep 0.7s
# White's move - knight g1-f3
Right @0.5s
Right @0.5s
Right @0.5s
Space @0.5s
Sleep 0.7s
# Show free cursor navigation naturally - move cursor a bit before confirming
Up @0.3s
Left @0.3s
Space @0.5s
Sleep 0.7s

Sleep 0.7s
# Black's move - knight b8-c6
Up @0.5s
Up @0.5s
Space @0.5s
Sleep 0.7s
# Free cursor navigation - navigate slightly before confirming destination
Right @0.3s
Space @0.5s
Sleep 0.7s

Sleep 0.7s
# White's move - bishop f1-c4
Down @0.5s
Left @0.5s
Space @0.5s
Sleep 0.7s
Right @0.3s
Right @0.3s
Space @0.5s
Sleep 0.7s
