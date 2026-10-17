//! Helpers shared by integration tests.

/// Keeps resume saves in an empty tempdir for this test binary. Starting a local or bot game
/// replaces that mode's save, so without this a test run would trip over (or delete) a real
/// saved game on the developer's machine.
pub fn isolate_resume_saves() {
    static DIR: std::sync::LazyLock<tempfile::TempDir> =
        std::sync::LazyLock::new(|| tempfile::tempdir().expect("tempdir"));
    static SET: std::sync::Once = std::sync::Once::new();
    // SAFETY: set once, before this binary's first App reads it.
    SET.call_once(|| unsafe { std::env::set_var("CHESS_TUI_RESUME_DIR", DIR.path()) });
}
