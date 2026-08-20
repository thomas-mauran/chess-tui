//! Precedence rules that depend on the process environment.
//!
//! This file deliberately holds a single test: `set_var` mutates process-global
//! state, so it must not run alongside another test that reads the environment.

use chess_tui::constants::{LICHESS_API_URL_ENV, startup_lichess_api_url};

#[test]
fn the_environment_variable_beats_the_config_but_not_the_cli_flag() {
    // SAFETY: this is the only test in this binary, so no other thread is
    // reading the environment while it is modified.
    unsafe {
        std::env::set_var(LICHESS_API_URL_ENV, "https://env.example/api");
    }

    // Nothing is applied on top of the environment value, which the global
    // already holds, so the persisted config is ignored.
    assert_eq!(startup_lichess_api_url(None, Some("https://cfg/api")), None);

    // The flag still wins.
    assert_eq!(
        startup_lichess_api_url(Some("https://cli.example/api"), Some("https://cfg/api")),
        Some("https://cli.example/api".to_string())
    );

    // SAFETY: same as above.
    unsafe {
        std::env::remove_var(LICHESS_API_URL_ENV);
    }

    assert_eq!(
        startup_lichess_api_url(None, Some("https://cfg/api")),
        Some("https://cfg/api".to_string())
    );
}
