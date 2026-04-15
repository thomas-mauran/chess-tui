use shakmaty::Square;

/// A coordinate on the chess board (row, col format)
/// This wraps around shakmaty's Square type for compatibility with existing UI code
///
/// Row and Col should be private to ensure that the Coord is valid through the code.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Coord {
    row: u8,
    col: u8,
}

pub enum MoveDirection {
    Up,
    Down,
    Left,
    Right,
}

impl Default for Coord {
    fn default() -> Self {
        Coord::new(4, 4)
    }
}

impl From<Square> for Coord {
    fn from(square: Square) -> Self {
        let index = square as u8;
        let rank = index / 8;
        let file = index % 8;
        Self {
            row: 7 - rank,
            col: file,
        }
    }
}

impl From<Coord> for Square {
    fn from(value: Coord) -> Self {
        let rank = 7 - value.row;
        let file = value.col;

        let index = (rank * 8) + file;

        Square::new(index.into())
    }
}

impl Coord {
    // This initialization makes the coord always compatible with shakmaty::Square
    pub fn new(row: u8, col: u8) -> Self {
        let row = row.min(7);
        let col = col.min(7);

        Self { row, col }
    }

    pub fn row(&self) -> u8 {
        self.row
    }

    pub fn col(&self) -> u8 {
        self.col
    }

    pub fn move_to(&mut self, direction: MoveDirection) {
        match direction {
            MoveDirection::Up => self.row = self.row.saturating_sub(1),
            MoveDirection::Down => self.row = (self.row + 1).min(7),
            MoveDirection::Left => self.col = self.col.saturating_sub(1),
            MoveDirection::Right => self.col = (self.col + 1).min(7),
        };
    }

    pub fn reverse(&self) -> Self {
        Self {
            row: 7 - self.row,
            col: 7 - self.col,
        }
    }
}

impl PartialOrd for Coord {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Coord {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.row.cmp(&other.row) {
            std::cmp::Ordering::Equal => self.col.cmp(&other.col),
            other => other,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Test bijection
    #[test]
    fn test_coord_square_conversion() {
        let coord = Coord::new(4, 4);
        let square: Square = coord.into();
        let back = Coord::from(square);
        assert_eq!(coord, back);
    }

    #[test]
    fn test_all_coord_combinations_yield_some_square() {
        // 65k iterations, shouldn't be too much to matter
        for r in 0..=255 {
            for c in 0..=255 {
                let coord = Coord::new(r, c);
                let square: Square = coord.into();
                let coord: Coord = square.into();

                assert!(coord.row() <= 7);
                assert!(coord.col() <= 7);
            }
        }
    }

    #[test]
    fn test_specific_chess_mappings() {
        // Coord(0, 0) is Row 0, Col 0 -> Rank 8, File A -> Index 56
        let a8_coord = Coord::new(0, 0);
        assert_eq!(
            Square::from(a8_coord),
            Square::A8,
            "Coord(0,0) should be A8 (56)"
        );

        // Coord(0, 7) is Row 0, Col 7 -> Rank 8, File H -> Index 63
        let h8_coord = Coord::new(0, 7);
        assert_eq!(
            Square::from(h8_coord),
            Square::H8,
            "Coord(0,7) should be H8 (63)"
        );

        // Coord(7, 0) is Row 7, Col 0 -> Rank 1, File A -> Index 0
        let a1_coord = Coord::new(7, 0);
        assert_eq!(
            Square::from(a1_coord),
            Square::A1,
            "Coord(7,0) should be A1 (0)"
        );

        // Coord(7, 7) is Row 7, Col 7 -> Rank 1, File H -> Index 7
        let h1_coord = Coord::new(7, 7);
        assert_eq!(
            Square::from(h1_coord),
            Square::H1,
            "Coord(7,7) should be H1 (7)"
        );
    }

    #[test]
    fn test_new_clamping() {
        // Ensure that even if we try to create a coord outside 7x7, it stays valid
        let coord = Coord::new(10, 255);
        assert!(coord.row() <= 7);
        assert!(coord.col() <= 7);
        assert_eq!(coord.row(), 7);
        assert_eq!(coord.col(), 7);
    }

    #[test]
    fn test_move_boundaries_up_left() {
        let mut coord = Coord::new(0, 0);

        // Should stay at 0,0 due to saturating_sub
        coord.move_to(MoveDirection::Up);
        coord.move_to(MoveDirection::Left);

        assert_eq!(coord.row(), 0);
        assert_eq!(coord.col(), 0);
    }

    #[test]
    fn test_move_boundaries_down_right() {
        let mut coord = Coord::new(7, 7);

        // Should stay at 7,7 due to .min(7)
        coord.move_to(MoveDirection::Down);
        coord.move_to(MoveDirection::Right);

        assert_eq!(coord.row(), 7);
        assert_eq!(coord.col(), 7);
    }

    #[test]
    fn test_reverse_is_always_valid() {
        // Test reverse at the extremes
        let coord_min = Coord::new(0, 0).reverse();
        let coord_max = Coord::new(7, 7).reverse();

        assert_eq!(coord_min.row(), 7);
        assert_eq!(coord_min.col(), 7);
        assert_eq!(coord_max.row(), 0);
        assert_eq!(coord_max.col(), 0);
    }

    #[test]
    fn test_sequence_of_moves() {
        let mut coord = Coord::new(5, 5);

        // Move way past the limit
        for _ in 0..10 {
            coord.move_to(MoveDirection::Right);
            coord.move_to(MoveDirection::Down);
        }

        assert!(coord.row() <= 7);
        assert!(coord.col() <= 7);
        assert_eq!(coord.row(), 7);
        assert_eq!(coord.col(), 7);
    }
}
