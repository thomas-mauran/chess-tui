use chess_tui::game_logic::clock::Clock;
use shakmaty::Color;
use std::time::Duration;

#[test]
fn test_new_clock_initial_time() {
    let clock = Clock::new(300);
    assert_eq!(clock.get_time(Color::White), Duration::from_secs(300));
    assert_eq!(clock.get_time(Color::Black), Duration::from_secs(300));
    assert!(!clock.is_running);
    assert!(clock.active_color.is_none());
}

#[test]
fn test_default_clock_is_ten_minutes() {
    let clock = Clock::default();
    assert_eq!(clock.get_time(Color::White), Duration::from_secs(600));
    assert_eq!(clock.get_time(Color::Black), Duration::from_secs(600));
}

#[test]
fn test_start_sets_running_state() {
    let mut clock = Clock::new(300);
    clock.start(Color::White);
    assert!(clock.is_running);
    assert_eq!(clock.active_color, Some(Color::White));
}

#[test]
fn test_stop_clears_running_state() {
    let mut clock = Clock::new(300);
    clock.start(Color::White);
    clock.stop();
    assert!(!clock.is_running);
    assert!(clock.active_color.is_none());
}

#[test]
fn test_stop_without_start_does_not_panic() {
    let mut clock = Clock::new(300);
    clock.stop();
    assert!(!clock.is_running);
}

#[test]
fn test_start_switches_active_color() {
    let mut clock = Clock::new(300);
    clock.start(Color::White);
    assert_eq!(clock.active_color, Some(Color::White));
    clock.stop();
    clock.start(Color::Black);
    assert_eq!(clock.active_color, Some(Color::Black));
    clock.stop();
    assert!(clock.active_color.is_none());
}

#[test]
fn test_get_time_returns_full_time_when_stopped() {
    let clock = Clock::new(300);
    assert_eq!(clock.get_time(Color::White), Duration::from_secs(300));
    assert_eq!(clock.get_time(Color::Black), Duration::from_secs(300));
}

#[test]
fn test_is_time_up_false_with_time_remaining() {
    let clock = Clock::new(300);
    assert!(!clock.is_time_up(Color::White));
    assert!(!clock.is_time_up(Color::Black));
}

#[test]
fn test_is_time_up_true_with_zero_duration() {
    let clock = Clock::new(0);
    assert!(clock.is_time_up(Color::White));
    assert!(clock.is_time_up(Color::Black));
}

#[test]
fn test_any_time_up_false_with_time_remaining() {
    let clock = Clock::new(300);
    assert!(!clock.any_time_up());
}

#[test]
fn test_any_time_up_true_with_zero_duration() {
    let clock = Clock::new(0);
    assert!(clock.any_time_up());
}

#[test]
fn test_get_time_up_color_none_when_time_remaining() {
    let clock = Clock::new(300);
    assert!(clock.get_time_up_color().is_none());
}

#[test]
fn test_get_time_up_color_white_when_out_of_time() {
    let clock = Clock::new(0);
    assert_eq!(clock.get_time_up_color(), Some(Color::White));
}

#[test]
fn test_format_time_over_one_minute() {
    let clock = Clock::new(300); // 5:00
    assert_eq!(clock.format_time(Color::White), "05:00");
    assert_eq!(clock.format_time(Color::Black), "05:00");
}

#[test]
fn test_format_time_exactly_one_minute() {
    let clock = Clock::new(60);
    assert_eq!(clock.format_time(Color::White), "01:00");
}

#[test]
fn test_format_time_under_one_minute() {
    let clock = Clock::new(45);
    assert_eq!(clock.format_time(Color::White), "45.000");
}

#[test]
fn test_format_time_zero() {
    let clock = Clock::new(0);
    assert_eq!(clock.format_time(Color::White), "00.000");
}

#[test]
fn test_format_time_mixed_minutes_seconds() {
    let clock = Clock::new(65); // 1:05
    assert_eq!(clock.format_time(Color::White), "01:05");
}
