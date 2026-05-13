Output examples/play_against_white_bot.gif

Require echo

Set BorderRadius 10
Set Shell zsh

Set FontSize 14
Set Width 2000
Set Height 1300

Set WindowBar Colorful
Set WindowBarSize 40

Type "cargo run -- --no-sound --depth 10 --difficulty hard --skin Default" Sleep 100ms  Enter

Sleep 2s
# Navigate to "Play Game" (index 0) - already selected
Space @0.3s
Sleep 0.3s

# Now in Game Mode Menu, navigate to "Play Bot" (index 2)
Right @0.3s
Right @0.3s
Space @0.3s
Sleep 0.3s

# Form is now active, cursor is on Time Control field (index 0)
# Default is Rapid (index 3), let's select Bullet (index 1) to show the feature
Left @0.3s
Left @0.3s
Sleep 0.3s

# Press Enter to move to Color field
Space @0.3s
Sleep 0.3s

# Now on Color field (index 1), select Black (press Right to toggle to Black)
Right @0.3s
Space @0.3s
Sleep 0.3s

# Press Enter to move to Depth field
Space @0.3s
Sleep 0.3s

# Start game
Right @0.3s
Right @0.3s
Space @0.3s
Sleep 0.3s

# Black's move - demonstrate free cursor navigation feature
Down @0.3s
Down @0.3s
Space @0.3s
Sleep 0.3s
# Free cursor navigation - move around before confirming
Left @0.2s
Right @0.2s
Up @0.2s
Space @0.3s
Sleep 0.5s

# White plays 1. e4
Sleep 0.3s

# Black's move 1...e5 with free cursor navigation
Right @0.3s
Right @0.3s
Down @0.3s
Space @0.3s
Sleep 0.3s
Up @0.2s
Left @0.2s
Space @0.3s
Sleep 0.5s

# White plays 2. Nf3
Sleep 0.3s

# Black's move 2...Nc6
Up @0.3s
Space @0.3s
Sleep 0.3s
Right @0.2s
Down @0.2s
Space @0.3s
Sleep 0.5s

# White plays 3. Bc4
Sleep 0.3s

# Black's move 3...Bc5
Left @0.3s
Left @0.3s
Left @0.3s
Down @0.3s
Space @0.3s
Sleep 0.3s
Right @0.2s
Space @0.3s
Sleep 0.5s

# Continue with free navigation feature
Sleep 0.3s

Sleep 0.8s
Type "h"
Sleep 3s
