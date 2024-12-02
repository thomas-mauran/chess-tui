use chess_tui::server::game_server::GameServer;

pub fn main() {
    let server = GameServer::new(true);
    server.run();
}