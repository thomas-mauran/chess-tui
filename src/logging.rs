use std::fs;
use std::path::PathBuf;
use log::LevelFilter;
use simplelog::*;
use chrono::Local;

pub fn setup_logging(config_dir: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    // Create logs directory
    let log_dir = config_dir.join("logs");
    fs::create_dir_all(&log_dir)?;

    // Create log file with timestamp
    let timestamp = Local::now().format("%Y-%m-%d_%H-%M-%S");
    let log_file = log_dir.join(format!("chess-tui_{}.log", timestamp));
    
    // Initialize logging
    CombinedLogger::init(vec![
        // Write logs to file
        WriteLogger::new(
            LevelFilter::Debug,
            Config::default(),
            fs::File::create(log_file)?
        ),
    ])?;

    log::info!("Logging initialized");
    Ok(())
} 