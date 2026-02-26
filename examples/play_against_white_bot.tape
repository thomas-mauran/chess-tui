Output examples/play_against_white_bot.gif

Require echo

Set BorderRadius 10
Set Shell zsh

Set FontSize 14
Set Width 2000
Set Height 1300

Set WindowBar Colorful
Set WindowBarSize 40

Type "cargo run -- --no-sound --depth 10 --difficulty hard --skin Default" Sleep 300ms  Enter

Sleep 3s
# Navigate to "Play Game" (index 0) - already selected
Space @0.3s
Sleep 0.4s

# Now in Game Mode Menu, navigate to "Play Bot" (index 2)
Right @0.3s
Right @0.3s
Space @0.3s
Sleep 0.4s

# Form is now active, cursor is on Time Control field (index 0)
# Default is Rapid (index 3), let's select Bullet (index 1) to show the feature
Left @0.3s
Left @0.3s
Sleep 0.4s

# Press Enter to move to Color field
Space @0.3s
Sleep 0.4s

# Now on Color field (index 1), select Black (press Right to toggle to Black)
Right @0.3s
Space @0.3s
Sleep 0.4s

# Press Enter to move to Depth field
Space @0.3s
Sleep 0.4s

# Start game
Right @0.3s
Right @0.3s
Space @0.3s
Sleep 0.4s

Down @0.3s
Down @0.3s
Space @0.3s
Space @0.3s
Sleep 0.8s

Right @0.3s
Right @0.3s
Down @0.3s
Space @0.3s
Space @0.3s

Up @0.3s
Space @0.3s
Right @0.3s
Space @0.3s

Left @0.3s
Left @0.3s
Left @0.3s
Down @0.3s
Space @0.3s
Space @0.3s
Sleep 0.6s

Up @0.3s
Right @0.3s
Space @0.3s
Space @0.3s

Up @0.3s
Left @0.3s
Space @0.3s
Space @0.3s

Up @0.3s
Left @0.3s
Space @0.3s
Space @0.3s

Up @0.3s
Left @0.3s
Space @0.3s
Space @0.3s

Left @0.3s
Space @0.3s
Space @0.3s

Sleep 0.8s
Type "h"
Sleep 3s
