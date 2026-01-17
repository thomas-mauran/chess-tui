Output examples/play_against_white_bot.gif

Require echo

Set BorderRadius 10
Set Shell zsh

Set FontSize 14
Set Width 2000
Set Height 1300

Set WindowBar Colorful
Set WindowBarSize 40

Type "cargo run" Sleep 0.7s  Enter

Sleep 2s
# Navigate to "Play Game" (index 0) - already selected
Space @0.5s
Sleep 0.7s

# Now in Game Mode Menu, navigate to "Play Bot" (index 2)
Right @0.5s
Right @0.5s
Space @0.5s
Sleep 0.7s

# Form is now active, cursor is on Color field (index 0)
# Select White (already default, but confirm with Enter)
Space @0.5s
Sleep 0.7s

# Now on Bot Depth field (index 1), adjust depth
Right @0.5s
Space @0.5s
Sleep 0.7s

# Start game
Space @0.5s
Sleep 0.7s

# Now playing - make some moves
Down @0.5s
Down @0.5s
Space @0.5s
Sleep 0.7s
Space @0.5s
Sleep 0.7s

Up @0.5s
Space @0.5s
Sleep 0.7s
Space @0.5s
Sleep 0.7s

Up @0.5s
Left @0.5s
Space @0.5s
Sleep 0.7s
Space @0.5s
Sleep 0.7s

Up @0.5s
Left @0.5s
Space @0.5s
Sleep 0.7s
Right @0.5s
Space @0.5s
Sleep 0.7s

Left @0.5s
Down @0.5s
Space @0.5s
Sleep 0.7s
Space @0.5s
Sleep 0.7s

Left @0.5s
Space @0.5s
Sleep 0.7s
Space @0.5s
Sleep 0.7s

Left @0.5s
Up @0.5s
Space @0.5s
Sleep 0.7s
Space @0.5s
Sleep 0.7s

Sleep 1.5s
Type "h"

Sleep 5s
