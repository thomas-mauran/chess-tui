//! Sound effect synthesis and playback.

use std::{
    num::NonZero,
    sync::atomic::{AtomicBool, Ordering},
};

static SOUND_ENABLED: AtomicBool = AtomicBool::new(true);
static AUDIO_AVAILABLE: AtomicBool = AtomicBool::new(true);
const SAMPLE_RATE: NonZero<u32> = match NonZero::new(44100) {
    Some(v) => v,
    None => panic!("sample rate cannot be zero"),
};

pub fn check_audio_availability() -> bool {
    #[cfg(feature = "sound")]
    {
        use rodio::DeviceSinkBuilder;
        let available = DeviceSinkBuilder::open_default_sink().is_ok();
        AUDIO_AVAILABLE.store(available, Ordering::Relaxed);
        available
    }
    #[cfg(not(feature = "sound"))]
    {
        AUDIO_AVAILABLE.store(false, Ordering::Relaxed);
        false
    }
}

pub fn set_sound_enabled(enabled: bool) {
    SOUND_ENABLED.store(enabled, Ordering::Relaxed);
}

pub fn is_sound_enabled() -> bool {
    SOUND_ENABLED.load(Ordering::Relaxed) && AUDIO_AVAILABLE.load(Ordering::Relaxed)
}

pub fn play_move_sound() {
    #[cfg(feature = "sound")]
    {
        if !is_sound_enabled() {
            return;
        }
        std::thread::spawn(|| {
            use rodio::source::{Function, SignalGenerator, Source};
            use rodio::{DeviceSinkBuilder, Player};
            use std::time::Duration;

            let Ok(mut sink) = DeviceSinkBuilder::open_default_sink() else {
                return;
            };
            sink.log_on_drop(false);
            let player = Player::connect_new(sink.mixer());

            let source = SignalGenerator::new(SAMPLE_RATE, 220.0, Function::Triangle)
                .amplify(0.3)
                .fade_in(Duration::from_millis(4))
                .take_duration(Duration::from_millis(80))
                .fade_out(Duration::from_millis(60));

            player.append(source);
            player.sleep_until_end();
            std::thread::sleep(Duration::from_millis(50));
        });
    }
}

pub fn play_menu_nav_sound() {
    #[cfg(feature = "sound")]
    {
        if !is_sound_enabled() {
            return;
        }
        std::thread::spawn(|| {
            use rodio::source::{Function, SignalGenerator, Source};
            use rodio::{DeviceSinkBuilder, Player};
            use std::time::Duration;

            let Ok(mut sink) = DeviceSinkBuilder::open_default_sink() else {
                return;
            };
            sink.log_on_drop(false);
            let player = Player::connect_new(sink.mixer());

            let source = SignalGenerator::new(SAMPLE_RATE, 160.0, Function::Triangle)
                .amplify(0.25)
                .fade_in(Duration::from_millis(3))
                .take_duration(Duration::from_millis(50))
                .fade_out(Duration::from_millis(40));

            player.append(source);
            player.sleep_until_end();
            std::thread::sleep(Duration::from_millis(50));
        });
    }
}
