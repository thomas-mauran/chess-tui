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

#[cfg(test)]
mod tests {
    use super::*;
    use shakmaty::Color;
    use std::time::{Duration, Instant};

    fn five_minute_clock() -> Clock {
        Clock::new(TimeControl::Custom(5))
    }

    #[test]
    fn test_new_clock_initial_time() {
        let clock = five_minute_clock();
        let duration = Duration::from_secs(300);
        let instant = Instant::now();

        assert_eq!(clock.get_time(Color::White, instant), duration);
        assert_eq!(clock.get_time(Color::Black, instant), duration);
        assert_eq!(clock.state(), ClockState::NotStarted);
    }

    #[test]
    fn test_default_clock_is_ten_minutes() {
        let clock = Clock::default();
        let instant = Instant::now();
        let duration = Duration::from_secs(600);
        assert_eq!(clock.get_time(Color::White, instant), duration);
        assert_eq!(clock.get_time(Color::Black, instant), duration);
    }

    #[test]
    fn test_start_sets_running_state() {
        let mut clock = five_minute_clock();
        let instant = Instant::now();
        clock.start(instant);
        assert_eq!(
            ClockState::Running {
                active_color: Color::White,
                turn_start: instant
            },
            clock.state()
        );
    }

    #[test]
    fn test_stop_clears_running_state() {
        let mut clock = five_minute_clock();
        let instant = Instant::now();
        clock.start(instant);
        clock.pause(instant + Duration::from_secs(10));
        assert_eq!(
            ClockState::Paused {
                active_color: Color::White
            },
            clock.state()
        );
        assert_eq!(clock.white_time(), Duration::from_secs(290));
    }

    #[test]
    fn test_stop_without_start_does_change_state() {
        let mut clock = five_minute_clock();
        let instant = Instant::now();
        clock.pause(instant);
        assert_eq!(ClockState::NotStarted, clock.state());
    }

    #[test]
    fn test_start_switches_active_color() {
        let mut clock = five_minute_clock();
        let instant = Instant::now();
        let instant_plus_hundred_secs = instant + Duration::from_secs(100);

        let whites_turn = ClockState::Running {
            active_color: Color::White,
            turn_start: instant,
        };

        let blacks_turn = ClockState::Running {
            active_color: Color::Black,
            turn_start: instant_plus_hundred_secs,
        };

        clock.start(instant);
        assert_eq!(clock.state(), whites_turn);

        clock.switch_turn(instant_plus_hundred_secs);

        assert_eq!(clock.state(), blacks_turn);
        assert_eq!(clock.white_time(), Duration::from_secs(200));
        assert_eq!(clock.black_time(), Duration::from_secs(300));
    }

    #[test]
    fn test_get_time_returns_full_time_when_stopped() {
        let clock = five_minute_clock();
        let duration = Duration::from_secs(300);
        assert_eq!(clock.white_time(), duration);
        assert_eq!(clock.black_time(), duration);
    }

    #[test]
    fn test_is_time_up_false_with_time_remaining() {
        let mut clock = five_minute_clock();

        let i = Instant::now();
        clock.start(i);
        let i = i + Duration::from_secs(10);
        clock.switch_turn(i);
        let i = i + Duration::from_secs(10);
        clock.switch_turn(i);

        assert!(!clock.is_time_up(Color::White, i));
        assert!(!clock.is_time_up(Color::Black, i));
    }

    #[test]
    fn test_is_time_up_true_with_zero_duration() {
        let mut clock = Clock::new(TimeControl::Custom(0));

        let i = Instant::now();
        clock.start(i);

        assert!(clock.is_time_up(Color::White, i));
        assert!(clock.is_time_up(Color::Black, i));
    }

    #[test]
    fn test_format_time_over_one_minute() {
        let clock = five_minute_clock(); // 5:00
        let i = Instant::now();
        assert_eq!(clock.format_time(Color::White, i), "05:00");
        assert_eq!(clock.format_time(Color::Black, i), "05:00");
    }

    #[test]
    fn test_format_time_exactly_one_minute() {
        let clock = Clock::new(TimeControl::Custom(1));
        let i = Instant::now();
        assert_eq!(clock.format_time(Color::White, i), "01:00");
    }

    #[test]
    fn test_format_time_under_one_minute() {
        let mut clock = five_minute_clock();
        let i = Instant::now();
        clock.sync_from_server(45_000, 45_000, Color::White, i);
        assert_eq!(clock.format_time(Color::White, i), "45.000");
    }

    #[test]
    fn test_format_time_zero() {
        let clock = Clock::new(TimeControl::Custom(0));
        let i = Instant::now();
        assert_eq!(clock.format_time(Color::White, i), "00.000");
    }

    #[test]
    fn test_format_time_mixed_minutes_seconds() {
        let mut clock = five_minute_clock();
        let i = Instant::now();
        clock.sync_from_server(65_000, 65_000, Color::White, i);
        assert_eq!(clock.format_time(Color::White, i), "01:05");
    }

    #[test]
    fn test_sync_from_server_sets_times_and_running_state() {
        let mut clock = five_minute_clock();
        let i = Instant::now();

        clock.sync_from_server(120_000, 3_000, Color::Black, i);

        assert_eq!(clock.white_time(), Duration::from_secs(120));
        assert_eq!(clock.black_time(), Duration::from_secs(3));
        assert_eq!(
            ClockState::Running {
                active_color: Color::Black,
                turn_start: i
            },
            clock.state()
        );
        // Active clock ticks down from the synced value.
        assert_eq!(
            clock.get_time(Color::Black, i + Duration::from_secs(1)),
            Duration::from_secs(2)
        );
        assert_eq!(
            clock.get_time(Color::White, i + Duration::from_secs(1)),
            Duration::from_secs(120)
        );
    }

    #[test]
    fn test_force_time_up_sets_loser_color() {
        let mut clock = five_minute_clock();
        let i = Instant::now();
        clock.sync_from_server(120_000, 3_000, Color::Black, i);

        clock.force_time_up(Color::Black);

        assert_eq!(
            ClockState::TimeUp {
                loser_color: Color::Black
            },
            clock.state()
        );
    }
}
