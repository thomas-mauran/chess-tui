use std::sync::LazyLock;

use ratatui::style::Color;
use crate::constants::{BLACK, WHITE};

static HAS_TRUE_COLOR: LazyLock<bool> = LazyLock::new(|| {
    matches!(
        std::env::var("COLORTERM").as_deref(),
        Ok("truecolor") | Ok("24bit")
    )
});

fn nearest_ansi256(r: u8, g: u8, b: u8) -> u8 {
    if r == g && g == b {
        if r < 8 { return 16; }
        if r > 248 { return 231; }
        return 232 + (r - 8) / 10;
    }
    let r6 = ((r as u16 * 5 + 127) / 255) as u8;
    let g6 = ((g as u16 * 5 + 127) / 255) as u8;
    let b6 = ((b as u16 * 5 + 127) / 255) as u8;
    16 + 36 * r6 + 6 * g6 + b6
}

/// Convert `Color::Rgb` to nearest 256-color index when TrueColor is
/// unavailable. Non-Rgb variants pass through unchanged.
pub fn adapt_color(color: Color) -> Color {
    if *HAS_TRUE_COLOR {
        return color;
    }
    match color {
        Color::Rgb(r, g, b) => Color::Indexed(nearest_ansi256(r, g, b)),
        other => other,
    }
}

/// Light board square color, adapted to terminal capability.
pub fn board_white() -> Color {
    adapt_color(WHITE)
}

/// Dark board square color, adapted to terminal capability.
pub fn board_black() -> Color {
    adapt_color(BLACK)
}
