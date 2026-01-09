# Features

![Lichess menu](../../static/gif/lichess-menu.gif)

Once authenticated, you can access the Lichess menu from the main screen. Here are the available features:

## User Profile

Upon entering the Lichess menu, you will see your profile information on the right side of the screen, including:
*   **Username** and online status.
*   **Ratings** for different time controls (Blitz, Rapid, Classical, Bullet, Puzzle).
*   **Game Statistics**: Total games, wins, losses, and draws.
*   **Visual Charts**: A bar chart visualizing your win/loss/draw ratio or rating distribution.

## Play Online

### Seek a Game (Quick Pairing)
Select **Seek Game** to find an opponent for a correspondence game (3 days per move). The application will connect to Lichess and find a match for you. This is perfect for playing at your own pace without time pressure.

### Join by Code
If you have a specific game ID (e.g., from a friend or a tournament), you can use **Join by Code** to enter the game ID and join directly. When joining an ongoing game, the current board state and last move are displayed immediately, so you can see exactly where the game stands.

### Ongoing Games
The **My Ongoing Games** option lists all your currently active games on Lichess. Select one to jump right back into the action. This is perfect for correspondence games or reconnecting to a live game.

## Puzzles

Select **Puzzle** to play rated chess puzzles from Lichess.
*   **Solve**: Make moves on the board to solve the puzzle.
*   **Feedback**: You'll get immediate feedback on whether your move was correct or incorrect.
*   **Rating**: Your puzzle rating will update automatically after each puzzle.

## Technical Details: Streaming System

`chess-tui` uses Lichess's streaming APIs to provide real-time game updates.

### How It Works

1. **Event Stream** (`/api/stream/event`): Monitors for new games starting, challenges, and game events
2. **Game Stream** (`/api/board/game/stream/{id}`): Streams move updates in real-time during active games
3. When you join a game, a background streaming thread connects to the game stream
4. Moves are received instantly as they happen via `gameState` events
5. When joining an ongoing game, the current board state (FEN) and last move are fetched immediately for instant display

This ensures you see moves in real-time as they happen, and allows you to make moves both in `chess-tui` and on the Lichess website while staying synchronized.
