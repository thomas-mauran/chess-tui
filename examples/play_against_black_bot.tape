Output examples/play_against_black_bot.gif

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
# Default is Rapid (index 3), let's select Blitz (index 2) to show the feature
Left @0.3s
Sleep 0.4s

# Press Enter to move to Color field
Space @0.3s
Sleep 0.4s

# Now on Color field (index 1), White is default
# Press Enter to confirm and move to Depth field
Space @0.3s
Sleep 0.4s

# Now on Bot Depth field (index 2), adjust depth
Space @0.3s
Sleep 0.2s

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

# Bot plays 1...e5
# 2. Nf3 (g1->f3)
Right @0.3s
Right @0.3s
Down @0.3s
Space @0.3s
Space @0.3s

# Bot plays 2...Nc6
# 3. Bc4 (f1->c4)

Left @0.3s
Space @0.3s
Right @0.3s
Right @0.3s
Space @0.3s
Sleep 0.6s


Left @0.3s
Space @0.3s
Space @0.3s
Sleep 0.6s

Up @0.3s
Space @0.3s
Space @0.3s
Sleep 0.6s

Up @0.3s
Left @0.3s
Space @0.3s
Space @0.3s
Sleep 0.6s

Right @0.3s
Space @0.3s
Space @0.3s
Sleep 0.6s

Left @0.3s
Space @0.3s
Space @0.3s
Sleep 0.6s

Left @0.3s
Space @0.3s
Space @0.3s
Sleep 0.6s

Left @0.3s
Space @0.3s
Space @0.3s
Sleep 0.6s

Up @0.3
Left @0.3s
Space @0.3s
Space @0.3s
Sleep 0.6s

Up @0.3s
Right @0.3s
Space @0.3s
Space @0.3s
Sleep 0.6s

Sleep 0.8s
Type "h"
Sleep 3s
