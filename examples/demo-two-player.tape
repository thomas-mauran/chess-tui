Output examples/demo-two-player.gif

Require echo

Set BorderRadius 10
Set Shell zsh

Set FontSize 14
Set Width 2000
Set Height 1300

Set WindowBar Colorful
Set WindowBarSize 40

Type "cargo run" Sleep 500ms  Enter

Sleep 0.7s
# Navigate to "Play Game" (index 0) - already selected, just press Enter
Space @0.5s
Sleep 1.5s

# Now in Game Mode Menu, "Local Game" (index 0) is selected
# Press Enter to start (no form needed)
Space @0.5s
Sleep 0.7s

# Now playing the game - make some moves
Down @0.5s
Down @0.5s
Space @0.5s
Sleep 0.7s
Space @0.5s

Sleep 0.7s
Left @0.5s
Space @0.5s
Sleep 0.7s
Space @0.5s

Down @0.5s
Space @0.5s
Sleep 0.7s
Space @0.5s
Sleep 0.7s

Right @0.5s
Right @0.5s
Right @0.5s
Space @0.5s
Sleep 0.7s
Space @0.5s
Sleep 0.7s

Left @0.5s
Space @0.5s
Sleep 0.7s
Right @0.5s
Right @0.5s
Space @0.5s
Sleep 0.7s

Up @0.5s
Right @0.5s
Right @0.5s
Space @0.5s
Sleep 0.7s
Space @0.5s
Sleep 0.7s


Up @0.5s
Up @0.5s
Up @0.5s
Space @0.5s
Sleep 0.7s
Space @0.5s
Sleep 0.7s
