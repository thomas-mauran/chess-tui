---
title: Real-Time Streaming Improvements - From Polling to Instant Updates
authors:
  - thomas-mauran
tags:
  - lichess
  - streaming
  - improvements
  - technical
date: 2026-01-08
---

# Real-Time Streaming Improvements - From Polling to Instant Updates

**Published:** January 8, 2026

We're excited to announce a major improvement to Chess TUI's Lichess integration: we've replaced the polling system with true real-time streaming, resulting in instant move updates and a much smoother playing experience.

<!-- truncate -->

## The Problem We Had

Previously, Chess TUI used a polling system that checked for game updates every 3 seconds. While this worked reliably, it introduced noticeable delays:

- **Up to 3 seconds delay** before seeing your opponent's move
- **Small but noticeable lag** in the demo videos
- **Unnecessary API requests** every few seconds

The original implementation used polling because we observed delays in the Lichess streaming API. However, as pointed out in [GitHub issue #188](https://github.com/thomas-mauran/chess-tui/issues/188), the issue might have been on the client side rather than the API itself.

## The Solution: True Real-Time Streaming
We've completely refactored the Lichess integration to use streaming APIs properly:

### Event Stream (`/api/stream/event`)

- Opens **before** creating a seek to ensure we don't miss game start events
- Monitors for `gameStart`, `gameFinish`, and challenge events
- Provides instant notification when a game begins

### Game Stream (`/api/board/game/stream/{gameId}`)

- Uses the **authenticated** `/api/board/game/stream/{gameId}` endpoint
- Maintains a persistent connection throughout the game
- Receives `gameState` events with moves **as they happen**
- No polling, no delays - moves appear instantly

## What This Means for You

- **Faster gameplay**: See moves instantly, especially important for blitz and rapid games
- **Smoother experience**: No more noticeable delays or lag
- **Better performance**: Fewer API requests means less server load
- **More reliable**: Properly configured streams are more stable than polling

## Looking Forward

This change sets the foundation for future improvements:

- Better support for faster time controls (bullet, blitz)
- More responsive UI updates

## Acknowledgments

Special thanks to [@niklasf](https://github.com/niklasf) (shakmaty creator) and [@ornicar](https://github.com/ornicar) (lichess creator himself) for the detailed investigation and for pointing us in the right direction. The community feedback and technical analysis helped us identify and fix the root cause.

---

**Resources:**
- [GitHub Issue #188](https://github.com/thomas-mauran/chess-tui/issues/188)
- [Lichess API Documentation](https://lichess.org/api)
- [Chess TUI Lichess Setup Guide](/docs/Lichess/setup)
- [Chess TUI Lichess Features](/docs/Lichess/features)

