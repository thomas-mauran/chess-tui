use chrono::Local;
use log::LevelFilter;
use simplelog::{CombinedLogger, Config, WriteLogger};
use std::fs;
use std::path::Path;

pub fn setup_logging(
    config_dir: &Path,
    log_level: &LevelFilter,
) -> Result<(), Box<dyn std::error::Error>> {
    match log_level {
        LevelFilter::Off => Ok(()), // No logging setup needed
        level => {
            // Create logs directory
            let log_dir = config_dir.join("logs");
            fs::create_dir_all(&log_dir)?;

            // Create log file with timestamp
            let timestamp = Local::now().format("%Y-%m-%d_%H-%M-%S");
            let log_file = log_dir.join(format!("chess-tui_{}.log", timestamp));

            CombinedLogger::init(vec![WriteLogger::new(
                *level,
                Config::default(),
                fs::File::create(log_file)?,
            )])?;

            log::info!("Logging initialized at {level} level");
            Ok(())
        }
    }
}
