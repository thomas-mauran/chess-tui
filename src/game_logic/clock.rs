//! Chess clock for both players.

use shakmaty::Color;
use std::time::{Duration, Instant};

#[derive(Clone, Debug)]
pub struct Clock {
    /// Time remaining for White (in seconds)
    white_time: Duration,
    /// Time remaining for Black (in seconds)
    black_time: Duration,
    state: ClockState,
}

impl Default for Clock {
    fn default() -> Self {
        let time_per_player = TimeControl::Rapid.into();
        Self {
            white_time: time_per_player,
            black_time: time_per_player,
            state: ClockState::NotStarted,
        }
    }
}

impl Clock {
    /// Create a new clock with the specified time per player (in seconds)
    pub fn new(time_control: TimeControl) -> Self {
        let time_per_player = time_control.into();

        let state = if time_control == TimeControl::NoClock {
            ClockState::Disabled
        } else {
            ClockState::NotStarted
        };

        Self {
            white_time: time_per_player,
            black_time: time_per_player,
            state,
        }
    }

    /// Construct a clock with explicit remaining times — used when restoring
    /// a saved game so each side resumes with the time they had on disk.
    ///
    /// TODO: Need to restore the state of which player is active too.
    pub fn with_remaining(white: Duration, black: Duration) -> Self {
        Self {
            white_time: white,
            black_time: black,
            ..Default::default()
        }
    }

    /// Start the clock for the given color
    pub fn start(&mut self, now: Instant) {
        if let ClockState::NotStarted = self.state {
            self.state = ClockState::Running {
                active_color: Color::White,
                turn_start: now,
            };
        }
    }

    pub fn resume(&mut self, now: Instant) {
        if let ClockState::Paused { active_color } = self.state {
            self.state = ClockState::Running {
                active_color,
                turn_start: now,
            };
        }
    }

    pub fn pause(&mut self, now: Instant) {
        if let ClockState::Running {
            active_color,
            turn_start,
        } = self.state
        {
            let elapsed = now.saturating_duration_since(turn_start);
            self.deduct_time(active_color, elapsed);

            self.state = ClockState::Paused { active_color }
        }
    }

    /// Swich turn, processing the discount of the current player
    pub fn switch_turn(&mut self, now: Instant) {
        if let ClockState::Running {
            active_color,
            turn_start,
        } = self.state
        {
            let elapsed = now.saturating_duration_since(turn_start);
            self.deduct_time(active_color, elapsed);

            if self.is_time_up(active_color, now) {
                self.state = ClockState::TimeUp {
                    loser_color: active_color,
                };
                return;
            }

            self.state = ClockState::Running {
                active_color: !active_color,
                turn_start: now,
            };
        }
    }

    pub fn is_time_up(&self, color: Color, instant: Instant) -> bool {
        self.get_time(color, instant) == Duration::ZERO
    }

    pub fn get_time(&self, color: Color, now: Instant) -> Duration {
        let base_time = self.get_base_time(color);

        if let ClockState::Running {
            active_color,
            turn_start,
        } = self.state
            && active_color == color
        {
            let elapsed = now.saturating_duration_since(turn_start);
            return base_time.saturating_sub(elapsed);
        }
        base_time
    }

    pub fn format_time(&self, color: Color, now: Instant) -> String {
        let time = self.get_time(color, now);
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

    fn deduct_time(&mut self, color: Color, amount: Duration) {
        match color {
            Color::White => self.white_time = self.white_time.saturating_sub(amount),
            Color::Black => self.black_time = self.black_time.saturating_sub(amount),
        }
    }

    fn get_base_time(&self, color: Color) -> Duration {
        match color {
            Color::White => self.white_time,
            Color::Black => self.black_time,
        }
    }

    pub fn white_time(&self) -> Duration {
        self.white_time
    }

    /// Synchronize with the server-reported remaining times (milliseconds),
    /// as sent by Lichess on every game event. The server is the source of
    /// truth, so we reset both base times and restart the running state.
    pub fn sync_from_server(
        &mut self,
        white_ms: u64,
        black_ms: u64,
        active_color: Color,
        now: Instant,
    ) {
        self.white_time = Duration::from_millis(white_ms);
        self.black_time = Duration::from_millis(black_ms);
        self.state = ClockState::Running {
            active_color,
            turn_start: now,
        };
    }

    /// Mark the given color as having run out of time.
    pub fn force_time_up(&mut self, color: Color) {
        self.state = ClockState::TimeUp { loser_color: color };
    }

    pub fn black_time(&self) -> Duration {
        self.black_time
    }

    pub fn state(&self) -> ClockState {
        self.state
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum ClockState {
    #[default]
    NotStarted,
    Disabled,
    Running {
        active_color: Color,
        turn_start: Instant,
    },
    Paused {
        active_color: Color,
    },
    TimeUp {
        loser_color: Color,
    },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TimeControl {
    UltraBullet,
    Bullet,
    Blitz,
    Rapid,
    Classical,
    NoClock,
    Custom(u64),
}

impl From<TimeControl> for Duration {
    fn from(value: TimeControl) -> Self {
        match value {
            TimeControl::UltraBullet => Duration::from_secs(15),
            TimeControl::Bullet => Duration::from_mins(1),
            TimeControl::Blitz => Duration::from_mins(5),
            TimeControl::Rapid => Duration::from_mins(10),
            TimeControl::Classical => Duration::from_hours(1),
            TimeControl::Custom(m) => Duration::from_mins(m),
            TimeControl::NoClock => Duration::from_mins(0),
        }
    }
}
