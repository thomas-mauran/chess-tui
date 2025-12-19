---
title: The Lichess Integration - A Deep Dive into Challenges, Solutions, and Future Plans
authors:
  - thomas-mauran
tags:
  - lichess
  - integration
  - technical
  - architecture
date: 2025-12-13
---

# The Lichess Integration - A Deep Dive into Challenges, Solutions, and Future Plans

**Published:** December 13, 2025

The Lichess integration in Chess TUI represents one of our most ambitious features, bringing online chess play directly to your terminal. In this blog post, we'll explore the technical challenges we faced, particularly around the polling system, the current state of the integration, and what's coming next.

<!-- truncate -->

## What is the Lichess Integration?

The Lichess integration allows Chess TUI users to connect their Lichess accounts and play chess online directly from the terminal. It provides several key features:

- **Seek Games**: Find opponents for correspondence games (3 days per move)
- **Puzzles**: Play rated chess puzzles from Lichess
- **Ongoing Games**: View and rejoin your active games
- **Join by Code**: Enter a game ID to join specific games
- **User Profile**: Display your ratings, statistics, and game history

All of this is accessible through an intuitive TUI menu that shows your profile information, ratings, and game statistics in real-time.

## The Polling System Challenge

One of the biggest technical challenges we faced was ensuring reliable move updates during games. This led us to implement a sophisticated polling system that works around limitations in the Lichess API.

### The Problem: Streaming API Delays

Initially, we tried to use Lichess's streaming API (`/api/stream/game/{id}`) for real-time move updates. However, we discovered that the streaming API has significant and unpredictable delays:

- **Delays range from 3 to 60 seconds** when delivering move updates
- These delays are inconsistent and can vary dramatically between requests
- For faster time controls (blitz, rapid), this makes the streaming API unreliable

This was unacceptable for a chess application where timely move updates are critical. A 60-second delay could mean missing your opponent's move entirely, especially in time-critical situations.

### Our Solution: Intelligent Polling

To work around this limitation, we implemented a hybrid approach that combines both streaming and polling:

#### 1. **Polling Thread (Primary Method)**

We spawn a dedicated polling thread that:
- Polls the game state **every 3 seconds** using `/api/stream/game/{id}`
- Compares the `turns` count and `lastMove` field to detect new moves
- Skips polling when it's the player's turn (to avoid unnecessary API calls)
- Resumes polling immediately after the player makes a move

```rust
// Poll every 3 seconds to avoid the 3-60 second delay in the stream
std::thread::sleep(std::time::Duration::from_secs(3));
```

#### 2. **Turn Detection Logic**

The polling system includes sophisticated logic to determine whose turn it is:

- Parses the FEN (Forsyth-Edwards Notation) to determine the active color
- Checks the `player` field in the game state
- Skips polling cycles when it's the player's turn
- Automatically resumes when the player makes a move (via a signal channel)

This optimization reduces unnecessary API calls while ensuring we catch opponent moves as quickly as possible.

#### 3. **Streaming Thread (Secondary Method)**

We also maintain a streaming connection as a backup:
- Provides game status updates (checkmate, draw, resignation)
- Handles initial game state when joining an ongoing game
- Acts as a fallback if polling fails

### The Struggles We Faced

Implementing this system wasn't straightforward. Here are some of the key challenges:

#### Challenge 1: Detecting New Moves

The Lichess API doesn't provide a simple "new move" event. We had to:
- Track the `turns` count (number of half-moves)
- Compare the `lastMove` field between polls
- Handle edge cases where moves might be detected out of order
- Deal with games that already have moves when we join

#### Challenge 2: Initial Game State

When joining an ongoing game, we need to:
- Detect how many moves have already been played
- Send the existing moves to the game board in the correct order
- Ensure the board state matches the server state exactly

We solved this by sending an `INIT_MOVES` command that tells the game logic how many moves to expect, preventing duplicate move application.

#### Challenge 3: Turn Detection Edge Cases

Determining whose turn it is can be tricky:
- The API format varies between endpoints
- Some games might not have complete state information
- We need to handle cases where the player color isn't immediately available

Our solution checks multiple sources (FEN, player field, game state) and falls back gracefully.

#### Challenge 4: Game Status Updates

We need to detect when games end (checkmate, draw, resignation, etc.):
- Monitor the `status` field in poll responses
- Send status updates to the UI when the game state changes
- Handle games that end while we're polling

## Current Status and Features

### ✅ What's Working

1. **Seek Game System**
   - Successfully finds opponents for correspondence games
   - Uses background polling to detect when a game starts
   - Handles the seek stream and ongoing games API

2. **Game Play**
   - Reliable move detection via polling
   - Turn-based move submission
   - Game state synchronization
   - Support for joining ongoing games

3. **Puzzle System**
   - Fetch puzzles from Lichess
   - Submit puzzle results
   - Track puzzle ratings and activity

4. **User Profile**
   - Display ratings for all time controls
   - Show game statistics (wins, losses, draws)
   - Visual charts for statistics
   - Online/offline status

5. **Ongoing Games**
   - List all active games
   - Join games by selecting from the list
   - Resume games in progress

### ⚠️ Known Limitations

1. **Time Controls**
   - Currently optimized for correspondence games (3 days per move)
   - Faster time controls work but may have slight delays
   - Real-time bullet games might miss some moves due to polling interval

2. **API Rate Limits**
   - Polling every 3 seconds means ~20 requests per minute per game
   - Lichess rate limits are generous, but multiple simultaneous games could be an issue
   - We skip polls when it's the player's turn to reduce load

3. **Stream Reliability**
   - The streaming API is used as a backup but isn't relied upon for moves
   - Some game status updates might be delayed if streaming fails

## Next Steps

We're continuously improving the integration based on user feedback. Upcoming improvements include:

- **Adaptive polling intervals** based on game time controls
- **Better error handling** with retry logic and graceful degradation
- **Challenge system** for custom time controls and friend challenges
- **WebSocket support** if available from Lichess for true real-time updates
- **Multi-game support** to play multiple games simultaneously

## Conclusion

The Lichess integration has been a challenging but rewarding feature. The polling system provides a reliable workaround for the streaming API's limitations, ensuring timely move updates even with the API's unpredictable delays.

We're always looking for feedback and contributions! Check out our [GitHub repository](https://github.com/thomas-mauran/chess-tui) or join our [Discussions](https://github.com/thomas-mauran/chess-tui/discussions).

---

**Resources:**
- [Lichess API Documentation](https://lichess.org/api)
- [Chess TUI Lichess Setup Guide](/docs/Lichess/setup)
- [Chess TUI Lichess Features](/docs/Lichess/features)
