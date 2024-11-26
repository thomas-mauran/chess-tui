use crate::app::{App, AppResult};
use crate::event::EventHandler;
use crate::ui::main_ui;
use ratatui::backend::Backend;
use ratatui::{Frame, Terminal};

/// Representation of a terminal user interface.
///
/// It is responsible for setting up the terminal,
/// initializing the interface and handling the draw events.
#[derive(Debug)]
pub struct Tui<B: Backend> {
    /// Interface to the Terminal.
    terminal: Terminal<B>,
    /// Terminal event handler.
    pub events: EventHandler,
}

impl<B: Backend> Tui<B> {
    /// Constructs a new instance of [`Tui`].
    pub fn new(terminal: Terminal<B>, events: EventHandler) -> Self {
        Self { terminal, events }
    }

    /// [`Draw`] the terminal interface by [`rendering`] the widgets.
    ///
    /// [`Draw`]: ratatui::Terminal::draw
    /// [`rendering`]: crate::ui:render
    // Créer une fonction async pour le rendu
    async fn render_async<'a>(&mut self, app: &mut App, frame: &mut Frame<'a>) {
        main_ui::render(app, frame).await;
    }

    pub async fn draw(&mut self, app: &mut App) -> AppResult<()> {
        // Passe une closure synchrone qui appelle la fonction async
        self.terminal.draw(|frame| {
            // Appel de la fonction async avec `.await`
            tokio::spawn(self.render_async(app, frame)); // ou await dans un contexte async
            Ok(())
        })?;
        Ok(())
    }
}
