//! Kitty chess-piece images.

use image::load_from_memory;
use ratatui_image::{
    picker::{Picker, ProtocolType},
    protocol::StatefulProtocol,
};
use shakmaty::{Color, Role};

const PIECES: [&[u8]; 12] = [
    include_bytes!("../assets/pieces/livius/wp.png"),
    include_bytes!("../assets/pieces/livius/wn.png"),
    include_bytes!("../assets/pieces/livius/wb.png"),
    include_bytes!("../assets/pieces/livius/wr.png"),
    include_bytes!("../assets/pieces/livius/wq.png"),
    include_bytes!("../assets/pieces/livius/wk.png"),
    include_bytes!("../assets/pieces/livius/bp.png"),
    include_bytes!("../assets/pieces/livius/bn.png"),
    include_bytes!("../assets/pieces/livius/bb.png"),
    include_bytes!("../assets/pieces/livius/br.png"),
    include_bytes!("../assets/pieces/livius/bq.png"),
    include_bytes!("../assets/pieces/livius/bk.png"),
];

pub struct KittyPieces(Vec<StatefulProtocol>);

impl KittyPieces {
    /// Loads image states only when the application is running in Kitty.
    pub fn detect() -> Option<Self> {
        std::env::var_os("KITTY_WINDOW_ID")?;
        if std::env::var_os("TMUX").is_some() {
            // SAFETY: startup calls this before the event handler creates its worker thread.
            unsafe { std::env::set_var("TERM_PROGRAM", "tmux") };
        }
        let mut picker = Picker::from_query_stdio().ok()?;
        if std::env::var_os("TMUX").is_some() {
            let _ = std::process::Command::new("tmux")
                .args(["set", "-g", "allow-passthrough", "on"])
                .status();
        }
        picker.set_protocol_type(ProtocolType::Kitty);
        PIECES
            .iter()
            .map(|png| {
                load_from_memory(png)
                    .ok()
                    .map(|image| picker.new_resize_protocol(image))
            })
            .collect::<Option<Vec<_>>>()
            .map(Self)
    }

    pub fn get_mut(&mut self, color: Color, role: Role) -> &mut StatefulProtocol {
        let color_offset = usize::from(color == Color::Black) * 6;
        &mut self.0[color_offset + role_index(role)]
    }
}

const fn role_index(role: Role) -> usize {
    match role {
        Role::Pawn => 0,
        Role::Knight => 1,
        Role::Bishop => 2,
        Role::Rook => 3,
        Role::Queen => 4,
        Role::King => 5,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_bundled_pieces_are_decodable_pngs() {
        assert!(PIECES.iter().all(|png| load_from_memory(png).is_ok()));
    }
}
