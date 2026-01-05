use std::sync::atomic::{AtomicBool, Ordering};

// Global sound enabled state
static SOUND_ENABLED: AtomicBool = AtomicBool::new(true);
// Track if audio is actually available (checked at startup)
static AUDIO_AVAILABLE: AtomicBool = AtomicBool::new(true);

/// Check if audio is available and update the availability state
/// This should be called at startup to detect if we're in an environment without audio (e.g., Docker)
pub fn check_audio_availability() -> bool {
    #[cfg(feature = "sound")]
    {
        use rodio::OutputStream;
        // Try to create an output stream
        // Note: ALSA may print errors to stderr, but we handle the failure gracefully
        let available = OutputStream::try_default().is_ok();
        AUDIO_AVAILABLE.store(available, Ordering::Relaxed);
        available
    }
    #[cfg(not(feature = "sound"))]
    {
        AUDIO_AVAILABLE.store(false, Ordering::Relaxed);
        false
    }
}

/// Set whether sounds are enabled
pub fn set_sound_enabled(enabled: bool) {
    SOUND_ENABLED.store(enabled, Ordering::Relaxed);
}

/// Get whether sounds are enabled
pub fn is_sound_enabled() -> bool {
    SOUND_ENABLED.load(Ordering::Relaxed) && AUDIO_AVAILABLE.load(Ordering::Relaxed)
}

/// Plays a move sound when a chess piece is moved.
/// This generates a pleasant, wood-like "click" sound using multiple harmonics.
pub fn play_move_sound() {
    #[cfg(feature = "sound")]
    {
        if !is_sound_enabled() {
            return;
        }
        // Spawn in a separate thread to avoid blocking the main game loop
        std::thread::spawn(|| {
            use rodio::{OutputStream, Sink};
            // Try to get an output stream, but don't fail if audio isn't available
            let Ok((_stream, stream_handle)) = OutputStream::try_default() else {
                return;
            };

            // Create a sink to play the sound
            let Ok(sink) = Sink::try_new(&stream_handle) else {
                return;
            };

            // Generate a pleasant wood-like click sound
            // Using a lower fundamental frequency with harmonics for a richer sound
            let sample_rate = 44100;
            let duration = 0.08; // 80 milliseconds - slightly longer for better perception
            let fundamental = 200.0; // Lower frequency for a more pleasant, less harsh sound

            let num_samples = (sample_rate as f64 * duration) as usize;
            let mut samples = Vec::with_capacity(num_samples);

            for i in 0..num_samples {
                let t = i as f64 / sample_rate as f64;

                // Create a more sophisticated envelope with exponential decay
                // Quick attack, smooth decay - like a wood piece being placed
                let envelope = if t < duration * 0.1 {
                    // Quick attack (10% of duration)
                    (t / (duration * 0.1)).powf(0.5)
                } else {
                    // Exponential decay
                    let decay_start = duration * 0.1;
                    let decay_time = t - decay_start;
                    let decay_duration = duration - decay_start;
                    (-decay_time * 8.0 / decay_duration).exp()
                };

                // Generate a richer sound with harmonics
                // Fundamental + 2nd harmonic (octave) + 3rd harmonic (fifth)
                let fundamental_wave = (t * fundamental * 2.0 * std::f64::consts::PI).sin();
                let harmonic2 = (t * fundamental * 2.0 * 2.0 * std::f64::consts::PI).sin() * 0.3;
                let harmonic3 = (t * fundamental * 2.0 * 3.0 * std::f64::consts::PI).sin() * 0.15;

                // Combine harmonics with envelope
                let sample = (fundamental_wave + harmonic2 + harmonic3) * envelope * 0.25;

                // Convert to i16 sample
                samples.push(
                    (sample * i16::MAX as f64).clamp(i16::MIN as f64, i16::MAX as f64) as i16,
                );
            }

            // Convert to a source that rodio can play
            let source = rodio::buffer::SamplesBuffer::new(1, sample_rate, samples);
            sink.append(source);
            sink.sleep_until_end();
        });
    }
}

/// Plays a light navigation sound when moving through menu items.
/// This generates a subtle, high-pitched "tick" sound for menu navigation.
pub fn play_menu_nav_sound() {
    #[cfg(feature = "sound")]
    {
        if !is_sound_enabled() {
            return;
        }
        // Spawn in a separate thread to avoid blocking the main game loop
        std::thread::spawn(|| {
            use rodio::{OutputStream, Sink};
            // Try to get an output stream, but don't fail if audio isn't available
            let Ok((_stream, stream_handle)) = OutputStream::try_default() else {
                return;
            };

            // Create a sink to play the sound
            let Ok(sink) = Sink::try_new(&stream_handle) else {
                return;
            };

            // Generate a light, high-pitched tick sound for menu navigation
            let sample_rate = 44100;
            let duration = 0.04;
            let frequency = 600.0;

            let num_samples = (sample_rate as f64 * duration) as usize;
            let mut samples = Vec::with_capacity(num_samples);

            for i in 0..num_samples {
                let t = i as f64 / sample_rate as f64;

                let envelope = if t < duration * 0.2 {
                    (t / (duration * 0.2)).powf(0.3)
                } else {
                    let decay_start = duration * 0.2;
                    let decay_time = t - decay_start;
                    let decay_duration = duration - decay_start;
                    (-decay_time * 12.0 / decay_duration).exp()
                };

                let sample = (t * frequency * 2.0 * std::f64::consts::PI).sin() * envelope * 0.3;

                samples.push(
                    (sample * i16::MAX as f64).clamp(i16::MIN as f64, i16::MAX as f64) as i16,
                );
            }

            // Convert to a source that rodio can play
            let source = rodio::buffer::SamplesBuffer::new(1, sample_rate, samples);
            sink.append(source);
            sink.sleep_until_end();
        });
    }
}
