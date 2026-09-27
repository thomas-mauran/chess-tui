use std::{
    panic,
    path::{Path, PathBuf},
};

use dirs::config_dir;
use log::LevelFilter;

use crate::{
    app::{App, AppResult},
    config::{Args, Config},
    constants::{DisplayMode, Pages, SKIN_NAME_ASCII, SKIN_NAME_DEFAULT},
    graphics,
    lichess::models::LichessClient,
    logging,
    pgn_viewer::PgnViewer,
    skin::{PieceStyle, Skin},
    sound,
    state::{bot_state::BotState, lichess_state::LichessState},
};

pub fn setup_app(args: &Args) -> AppResult<App> {
    setup_panic();

    let mut app = App::default();

    let config_dir = config_dir().ok_or("Error retrieving config directory.")?;
    let folder_path = config_dir.join("chess-tui");
    let config_path = config_dir.join("chess-tui/config.toml");

    let config = Config::config_create(args, &folder_path, &config_path)?;

    setup_logging(&mut app, &config, &folder_path);
    setup_sound(&mut app, &config, args);

    setup_display_mode(&mut app, &config);
    setup_skin_and_piece_styles(&mut app, &config_dir)?;
    apply_skin(&mut app, &config, args);

    setup_animations(&mut app, &config);
    setup_bot(&mut app.bot_state, &config, args);
    setup_lichess(&mut app.lichess_state, &config, args);
    setup_pgn(&mut app, args)?;
    setup_kitty(&mut app, &config);

    enable_mouse_capture()?;

    Ok(app)
}

fn setup_panic() {
    let default_panic = std::panic::take_hook();
    panic::set_hook(Box::new(move |info| {
        ratatui::restore();
        let _ = ratatui::crossterm::execute!(
            std::io::stdout(),
            ratatui::crossterm::event::DisableMouseCapture
        );
        default_panic(info);
    }));
}

fn enable_mouse_capture() -> AppResult<()> {
    Ok(ratatui::crossterm::execute!(
        std::io::stdout(),
        ratatui::crossterm::event::EnableMouseCapture
    )?)
}

/// Check audio availability and disable sound if not available (e.g., in Docker)
fn setup_sound(app: &mut App, config: &Config, args: &Args) {
    let audio_available = sound::check_audio_availability();
    if !audio_available {
        // Automatically disable sound if audio is not available
        app.sound_enabled = false;
    }
    // Initialize global sound state from app default
    sound::set_sound_enabled(app.sound_enabled);

    // Add sound enabled handling
    if let Some(sound_enabled) = config.sound_enabled {
        app.sound_enabled = sound_enabled;
        sound::set_sound_enabled(sound_enabled);
    }

    // Command line no-sound flag takes precedence over configuration file
    if args.no_sound {
        app.sound_enabled = false;
        sound::set_sound_enabled(false);
    }
}

fn setup_display_mode(app: &mut App, config: &Config) {
    if let Some(ref display_mode) = config.display_mode {
        app.game.ui.display_mode = match display_mode.as_str() {
            "ASCII" => DisplayMode::ASCII,
            "CUSTOM" => DisplayMode::CUSTOM,
            _ => DisplayMode::DEFAULT,
        };
    }
}

fn setup_logging(app: &mut App, config: &Config, folder_path: &Path) {
    if let Some(ref log_level) = config.log_level {
        app.log_level = log_level.parse().unwrap_or(LevelFilter::Off);
    }

    if let Err(e) = logging::setup_logging(folder_path, &app.log_level) {
        eprintln!("Failed to initialize logging: {}", e);
    }
}

fn setup_bot(bot_state: &mut BotState, config: &Config, args: &Args) {
    // We store the chess engine path if there is one
    if let Some(ref engine_path) = config.engine_path {
        bot_state.chess_engine_path = Some(engine_path.clone());
    }

    // Add bot depth handling
    if let Some(bot_depth) = config.bot_depth {
        bot_state.bot_depth = bot_depth;
    }

    // Bot difficulty
    bot_state.bot_difficulty = config.bot_difficulty;

    // Command line arguments take precedence over configuration file

    if !args.engine_path.is_empty() {
        bot_state.chess_engine_path = Some(args.engine_path.clone());
    }

    if let Some(depth) = args.depth {
        bot_state.bot_depth = depth;
    }

    if let Some(ref d) = args.difficulty {
        let idx = match d.to_lowercase().as_str() {
            "easy" => Some(0),
            "medium" => Some(1),
            "hard" => Some(2),
            "magnus" => Some(3),
            _ => None,
        };
        if let Some(i) = idx {
            bot_state.bot_difficulty = Some(i);
        }
    }
}

fn setup_animations(app: &mut App, config: &Config) {
    if let Some(animations_enabled) = config.animations_enabled {
        app.animations_enabled = animations_enabled;
    }
}

fn setup_lichess(lichess_state: &mut LichessState, config: &Config, args: &Args) {
    // Add lichess token handling
    if let Some(ref lichess_token) = config.lichess_token {
        lichess_state.token = Some(lichess_token.clone());
        lichess_state.client = Some(LichessClient::new(lichess_token.clone()));
    }

    // Command line lichess token takes precedence over configuration file
    if let Some(token) = &args.lichess_token {
        lichess_state.token = Some(token.clone());
        lichess_state.client = Some(LichessClient::new(token.clone()));
    }
}

/// Load PGN file(s) if --pgn was provided
fn setup_pgn(app: &mut App, args: &Args) -> AppResult<()> {
    let Some(ref pgn_path) = args.pgn else {
        return Ok(());
    };

    let games = load_pgn_games(Path::new(pgn_path)).unwrap_or_else(|e| {
        eprintln!("Failed to load PGN '{pgn_path}': {e}");
        std::process::exit(1);
    });

    app.pgn_viewer_state = Some(games);
    app.pgn_viewer_game_idx = 0;
    app.ui_state.current_page = Pages::PgnViewer;

    Ok(())
}

fn load_pgn_games(path: &Path) -> Result<Vec<PgnViewer>, String> {
    if path.is_dir() {
        load_pgn_from_dir(path)
    } else {
        PgnViewer::from_file(path.to_str().unwrap_or(path.to_string_lossy().as_ref()))
    }
}

fn load_pgn_from_dir(dir_path: &Path) -> Result<Vec<PgnViewer>, String> {
    let mut entries: Vec<_> = std::fs::read_dir(dir_path)
        .map_err(|e| e.to_string())?
        .filter_map(Result::ok)
        .filter(|entry| entry.path().extension().is_some_and(|ext| ext == "pgn"))
        .collect();

    entries.sort_by_key(|e| e.path());

    let mut all_games = Vec::new();
    for entry in entries {
        let file_path = entry.path();
        match PgnViewer::from_file(file_path.to_str().unwrap_or("")) {
            Ok(mut games) => all_games.append(&mut games),
            Err(e) => eprintln!("Skipping {file_path:?}: {e}"),
        }
    }

    if all_games.is_empty() {
        Err(format!(
            "No valid .pgn files found in '{}'",
            dir_path.display()
        ))
    } else {
        Ok(all_games)
    }
}

fn get_skins(skins_path: &PathBuf) -> AppResult<Vec<Skin>> {
    let skins = Skin::load_all_skins(skins_path)?
        .into_iter()
        // Filter out any "Default" or "ASCII" skins from JSON to avoid duplicates
        .filter(|s| s.name != SKIN_NAME_DEFAULT && s.name != SKIN_NAME_ASCII)
        .collect();

    Ok(skins)
}

fn get_piece_styles(skins_path: &PathBuf) -> AppResult<Vec<PieceStyle>> {
    let piece_styles = Skin::load_all_piece_styles(skins_path)?
        .into_iter()
        .filter(|ps| ps.name != SKIN_NAME_DEFAULT && ps.name != SKIN_NAME_ASCII)
        .collect();

    Ok(piece_styles)
}

fn setup_skin_and_piece_styles(app: &mut App, config_dir: &Path) -> AppResult<()> {
    // Always start with Default and ASCII display modes at the beginning
    app.theme_state.available_skins.push(Skin::default());
    app.theme_state
        .available_skins
        .push(Skin::ascii_display_mode());

    let skins_path = config_dir.join("chess-tui/skins.json");

    // Create skins.json if it doesn't exist
    if !skins_path.exists() {
        Skin::create_default_skins_file(&skins_path)?;
    }

    let skins = get_skins(&skins_path)?;
    let piece_styles = get_piece_styles(&skins_path)?;

    app.theme_state.available_skins.extend(skins);
    app.theme_state.available_piece_styles.extend(piece_styles);

    // Sync loaded piece styles to the game UI so rendering can use them
    app.game.ui.available_piece_styles = app.theme_state.available_piece_styles.clone();

    Ok(())
}

fn apply_skin(app: &mut App, config: &Config, args: &Args) {
    if let Some(ref selected_skin_name) = config.selected_skin_name {
        app.theme_state.selected_skin_name = selected_skin_name.clone();
    }

    // Apply selected skin
    if let Some(skin) = Skin::get_skin_by_name(
        &app.theme_state.available_skins,
        &app.theme_state.selected_skin_name,
    ) {
        app.theme_state.loaded_skin = Some(skin.clone());
        app.game.ui.skin = skin.clone();
        // Set display mode based on skin name
        match app.theme_state.selected_skin_name.as_str() {
            SKIN_NAME_DEFAULT => app.game.ui.display_mode = DisplayMode::DEFAULT,
            SKIN_NAME_ASCII => app.game.ui.display_mode = DisplayMode::ASCII,
            _ => {
                // For custom skins, set to CUSTOM if not already set
                if app.game.ui.display_mode == DisplayMode::DEFAULT {
                    app.game.ui.display_mode = DisplayMode::CUSTOM;
                }
            }
        }
    } else {
        // Fallback: use the first available skin if selected skin not found
        if let Some(first_skin) = app.theme_state.available_skins.first() {
            app.theme_state.selected_skin_name = first_skin.name.clone();
            app.theme_state.loaded_skin = Some(first_skin.clone());
            app.game.ui.skin = first_skin.clone();
            // Set display mode based on skin name
            match app.theme_state.selected_skin_name.as_str() {
                "Default" => app.game.ui.display_mode = DisplayMode::DEFAULT,
                "ASCII" => app.game.ui.display_mode = DisplayMode::ASCII,
                _ => app.game.ui.display_mode = DisplayMode::CUSTOM,
            }
        }
    }

    // Command line skin takes precedence over configuration file (reproducible theme)
    if let Some(ref skin_name) = args.skin {
        app.theme_state.selected_skin_name = skin_name.clone();
        if let Some(skin) = Skin::get_skin_by_name(&app.theme_state.available_skins, skin_name) {
            app.theme_state.loaded_skin = Some(skin.clone());
            app.game.ui.skin = skin.clone();
            match skin_name.as_str() {
                "Default" => app.game.ui.display_mode = DisplayMode::DEFAULT,
                "ASCII" => app.game.ui.display_mode = DisplayMode::ASCII,
                _ => app.game.ui.display_mode = DisplayMode::CUSTOM,
            }
        }
    }
}

fn setup_kitty(app: &mut App, config: &Config) {
    if let Some(kitty_graphics_enabled) = config.kitty_graphics_enabled {
        app.kitty_graphics_enabled = kitty_graphics_enabled;
    }

    app.kitty_pieces = graphics::KittyPieces::detect();
}
