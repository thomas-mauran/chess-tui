Output examples/play_against_black_bot.gif

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
# Default is Rapid (index 3), let's select Blitz (index 2) to show the feature
Left @0.3s
Sleep 0.3s

# Press Enter to move to Color field
Space @0.3s
Sleep 0.3s

# Now on Color field (index 1), White is default
# Press Enter to confirm and move to Depth field
Space @0.3s
Sleep 0.3s

# Now on Bot Depth field (index 2), adjust depth
Space @0.3s
Sleep 0.2s

# Start game
Right @0.3s
Right @0.3s
Space @0.3s
Sleep 0.5s

# White's move 1. e2-e4
Down @0.5s
Down @0.5s
Space @0.5s
Sleep 0.4s
# Show free cursor navigation - move to destination square
Up @0.5s
Up @0.5s
Space @0.5s
Sleep 0.7s

# Bot plays 1...e5
Sleep 0.5s

# White's move 2. Nf3 (g1->f3)
Right @0.5s
Right @0.5s
Right @0.5s
Space @0.5s
Sleep 0.4s
Up @0.5s
Left @0.5s
Space @0.5s
Sleep 0.7s

# Bot plays 2...Nc6
Sleep 0.5s

# White's move 3. Bc4 (f1->c4)
Down @0.5s
Left @0.5s
Space @0.5s
Sleep 0.4s
Right @0.5s
Right @0.5s
Right @0.5s
Space @0.5s
Sleep 0.7s

# Bot plays 3...Nf6
Sleep 0.5s

# White's move 4. d2-d4 (open center)
Down @0.5s
Left @0.5s
Space @0.5s
Sleep 0.4s
# Show free cursor navigation on opponent's turn - move around before confirming
Left @0.3s
Right @0.3s
Space @0.5s
Sleep 0.7s

# Bot plays 4...Nxe4
Sleep 0.5s

# White's move 5. d4xe5
Right @0.5s
Space @0.5s
Sleep 0.4s
Up @0.5s
Space @0.5s
Sleep 0.7s

# Continue playing
Sleep 2s
