//! TCP multiplayer session setup and teardown.

use crate::app::App;
use crate::constants::{Popups, NETWORK_PORT, SLEEP_DURATION_LONG_MS};
use crate::game_logic::opponent::wait_for_game_start;
use crate::game_logic::opponent::{Opponent, OpponentKind};
use crate::server::game_server::get_host_ip;
use shakmaty::Color;
use std::net::UdpSocket;

impl App {
    /// Connects to a TCP multiplayer game. If hosting, opens a waiting popup and resolves the local IP.
    /// If joining, fetches the assigned color from the server. Flips the board for the Black player.
    pub fn create_opponent(&mut self) {
        // Host passes their opponent's color so the server knows the assignment.
        // Guest always passes None so get_color_from_stream fetches the real color from the server.
        let other_player_color = if self.multiplayer_state.hosting.unwrap_or(false) {
            self.game_mode_state.selected_color.map(|c| c.other())
        } else {
            None
        };

        if self.multiplayer_state.hosting.unwrap_or(false) {
            log::info!(
                "Setting up host with color: {:?}",
                self.game_mode_state.selected_color
            );
            self.ui_state.current_popup = Some(Popups::WaitingForOpponentToJoin);
            if let Some(ip) = get_host_ip() {
                self.multiplayer_state.host_ip = Some(format!("{}:{}", ip, NETWORK_PORT));
            } else {
                log::error!("Could not get local IP, defaulting to 127.0.0.1");
                self.multiplayer_state.host_ip = Some(format!("127.0.0.1:{}", NETWORK_PORT));
            }
        }

        let addr_with_port = self
            .multiplayer_state
            .host_ip
            .as_deref()
            .unwrap_or(&format!("127.0.0.1:{}", NETWORK_PORT))
            .to_string();
        log::info!("Attempting to connect to: {}", addr_with_port);

        // ping the server to see if it's up
        if let Err(e) = UdpSocket::bind(addr_with_port.clone()) {
            log::error!("Server is unreachable at {}: {}", addr_with_port, e);
            self.multiplayer_state.host_ip = None;
            return;
        }

        log::info!("Creating opponent with color: {:?}", other_player_color);

        match Opponent::new(addr_with_port, other_player_color) {
            Ok(mut opponent) => {
                if self.multiplayer_state.hosting.unwrap_or(false) {
                    log::info!("Setting up client (host) player");
                    log::info!("Starting background thread to monitor when the opponent is ready");

                    let (start_tx, start_rx) = std::sync::mpsc::channel();
                    self.multiplayer_state.game_start_rx = Some(start_rx);

                    // Create a separate thread that checks in background if the game can start
                    // Extract TcpStream from OpponentKind if it's a TCP connection
                    if let Some(OpponentKind::Tcp(stream)) = &mut opponent.kind {
                        let stream_clone = match stream.try_clone() {
                            Ok(s) => s,
                            Err(e) => {
                                log::error!("Failed to clone stream: {}", e);
                                return;
                            }
                        };
                        std::thread::spawn(move || {
                            // Create a temporary Opponent with the cloned stream to pass to wait_for_game_start
                            let mut temp_opponent = Opponent {
                                kind: Some(OpponentKind::Tcp(stream_clone)),
                                opponent_will_move: false,
                                color: Color::White,
                                game_started: false,
                                initial_move_count: 0,
                                moves_received: 0,
                            };
                            // Poll repeatedly until game starts
                            loop {
                                match wait_for_game_start(&mut temp_opponent) {
                                    Ok(true) => {
                                        let _ = start_tx.send(());
                                        break;
                                    }
                                    Ok(false) => {
                                        // Still waiting, sleep a bit and check again
                                        std::thread::sleep(std::time::Duration::from_millis(
                                            SLEEP_DURATION_LONG_MS,
                                        ));
                                    }
                                    Err(e) => {
                                        log::warn!("Failed to start hosted game: {}", e);
                                        break;
                                    }
                                }
                            }
                        });
                    }
                } else {
                    log::info!("Setting up client (non-host) player");
                    self.game_mode_state.selected_color = Some(opponent.color.other());
                    self.game_mode_state.is_random_color = false;
                    opponent.game_started = true;
                }
                self.game.logic.opponent = Some(opponent);
            }
            Err(e) => {
                log::error!("Failed to create opponent: {}", e);
                self.multiplayer_state.host_ip = None;
                self.ui_state
                    .show_message_popup(format!("Connection failed: {}", e), Popups::Error);
                return;
            }
        }

        if self.game_mode_state.selected_color.unwrap_or(Color::White) == Color::Black {
            log::debug!("Flipping board for black player");
            self.game.logic.game_board.flip_the_board();
        }

        // Ensure skin is preserved when starting multiplayer
        if let Some(skin) = &self.theme_state.loaded_skin {
            self.game.ui.skin = skin.clone();
        }
    }

    /// Cancels an in-progress hosting session. Shuts down the TCP socket and resets
    /// multiplayer, board, and UI state.
    pub fn cancel_hosting_cleanup(&mut self) {
        log::info!("Cancelling hosting and cleaning multiplayer state");

        // Close the socket
        if let Some(mut opponent) = self.game.logic.opponent.take() {
            if let Some(OpponentKind::Tcp(stream)) = opponent.kind.take() {
                let _ = stream.shutdown(std::net::Shutdown::Both);
            }
        }

        self.game.logic.opponent = None;
        self.multiplayer_state.reset();
        self.game_mode_state.reset_selected_color();
        self.game.logic.game_board.reset();
        self.game.ui.reset();
    }
}
