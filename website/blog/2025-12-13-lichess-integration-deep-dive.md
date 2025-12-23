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

### Our Solution: Continuous Polling

To work around this limitation, we implemented a continuous polling system that provides reliable move detection:

#### 1. **Polling Thread (Primary Method)**

We spawn a dedicated polling thread that:
- Polls the game state **every 3 seconds** using `/api/stream/game/{id}`
- Compares the `turns` count and `lastMove` field to detect new moves
- **Polls continuously** even when it's the player's turn, to detect moves made on the Lichess website
- Uses the public stream endpoint, reading the first line (gameFull event) and closing the connection

```rust
// Poll every 3 seconds to avoid the 3-60 second delay in the stream
std::thread::sleep(std::time::Duration::from_secs(3));
```

#### 2. **Initial Game State Setup**

When joining an ongoing game, we:
- Fetch the current board state using FEN (Forsyth-Edwards Notation) from the game state
- Immediately fetch the turn count and last move to display the current position
- Set up the board position directly from FEN, ensuring accurate state representation
- Display the last move immediately (highlighted in green) without waiting for the first poll

This ensures that when you join a game in progress, you see the exact current board state right away.

#### 3. **Simplified Architecture**

We've simplified the system by:
- **Removing the streaming thread** - relying solely on polling for all updates
- Using FEN-based board setup instead of trying to reconstruct move history
- Fetching game state information directly from the stream endpoint when needed

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
- Display the current board state accurately
- Show the last move immediately without delay

We solved this by:
- Fetching the FEN (Forsyth-Edwards Notation) directly from the game state
- Setting up the board position from FEN, which provides the exact current state
- Fetching the turn count and last move immediately when joining, so it displays right away
- Using an `INIT_MOVES` command that tells the game logic how many moves to expect, preventing duplicate move application

#### Challenge 3: Continuous Polling

We initially tried to optimize by skipping polls when it's the player's turn, but this prevented detection of moves made on the Lichess website. Our solution:
- Polls continuously every 3 seconds regardless of whose turn it is
- This allows users to make moves both in `chess-tui` and on the Lichess website while staying synchronized
- The polling interval is short enough to catch moves quickly while being respectful of API rate limits

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
   - We use the public stream endpoint which doesn't require authentication and is more reliable

3. **Architecture Simplification**
   - We've removed the streaming thread to simplify the codebase
   - All updates now come through the polling mechanism
   - This reduces complexity while maintaining reliability

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
