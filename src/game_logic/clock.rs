use shakmaty::Color;
use std::time::{Duration, Instant};

/// Represents a chess clock that tracks time for both players
#[derive(Clone)]
pub struct Clock {
    /// Time remaining for White (in seconds)
    white_time: Duration,
    /// Time remaining for Black (in seconds)
    black_time: Duration,
    /// When the current player's turn started
    turn_start: Option<Instant>,
    /// Which player's clock is currently running
    pub active_color: Option<Color>,
    /// Whether the clock is running
    pub is_running: bool,
}

impl Clock {
    /// Create a new clock with the specified time per player (in seconds)
    pub fn new(seconds: u32) -> Self {
        let time_per_player = Duration::from_secs(seconds as u64);
        Self {
            white_time: time_per_player,
            black_time: time_per_player,
            turn_start: None,
            active_color: None,
            is_running: false,
        }
    }

    /// Start the clock for the given color
    pub fn start(&mut self, color: Color) {
        self.active_color = Some(color);
        self.turn_start = Some(Instant::now());
        self.is_running = true;
    }

    /// Stop the clock and update the time for the current player
    pub fn stop(&mut self) {
        if let (Some(start), Some(color)) = (self.turn_start, self.active_color) {
            let elapsed = start.elapsed();
            match color {
                Color::White => {
                    if elapsed < self.white_time {
                        self.white_time -= elapsed;
                    } else {
                        self.white_time = Duration::ZERO;
                    }
                }
                Color::Black => {
                    if elapsed < self.black_time {
                        self.black_time -= elapsed;
                    } else {
                        self.black_time = Duration::ZERO;
                    }
                }
            }
        }
        self.turn_start = None;
        self.active_color = None;
        self.is_running = false;
    }

    /// Get the current time remaining for a color (accounting for elapsed time on current turn)
    pub fn get_time(&self, color: Color) -> Duration {
        let base_time = match color {
            Color::White => self.white_time,
            Color::Black => self.black_time,
        };

        // If this color's clock is running, subtract elapsed time
        if self.is_running && self.active_color == Some(color) && self.turn_start.is_some() {
            let elapsed = self.turn_start.unwrap().elapsed();
            if elapsed < base_time {
                base_time - elapsed
            } else {
                Duration::ZERO
            }
        } else {
            base_time
        }
    }

    /// Check if a player has run out of time
    pub fn is_time_up(&self, color: Color) -> bool {
        self.get_time(color) == Duration::ZERO
    }

    /// Check if any player has run out of time
    pub fn any_time_up(&self) -> bool {
        self.is_time_up(Color::White) || self.is_time_up(Color::Black)
    }

    /// Get the color that ran out of time (if any)
    pub fn get_time_up_color(&self) -> Option<Color> {
        if self.is_time_up(Color::White) {
            Some(Color::White)
        } else if self.is_time_up(Color::Black) {
            Some(Color::Black)
        } else {
            None
        }
    }

    /// Format time as MM:SS or SS.mmm (with milliseconds only if under 1 minute)
    pub fn format_time(&self, color: Color) -> String {
        let time = self.get_time(color);
        let total_secs = time.as_secs();
        let millis = time.subsec_millis();
        let minutes = total_secs / 60;
        let seconds = total_secs % 60;

        if minutes > 0 {
            // Over 1 minute: show MM:SS without milliseconds
            format!("{:02}:{:02}", minutes, seconds)
        } else {
            // Under 1 minute: show SS.mmm with milliseconds
            format!("{:02}.{:03}", seconds, millis)
        }
    }
}

impl Default for Clock {
    fn default() -> Self {
        Self {
            white_time: Duration::from_secs(10 * 60),
            black_time: Duration::from_secs(10 * 60),
            turn_start: None,
            active_color: None,
            is_running: false,
        }
    }
}
