use shakmaty::Square;

/// A coordinate on the chess board (row, col format)
/// This wraps around shakmaty's Square type for compatibility with existing UI code
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Coord {
    pub row: u8,
    pub col: u8,
}

impl Coord {
    pub fn new(row: u8, col: u8) -> Self {
        Self { row, col }
    }

    pub fn undefined() -> Self {
        Self { row: 255, col: 255 }
    }

    pub fn is_valid(&self) -> bool {
        self.row < 8 && self.col < 8
    }

    pub fn reverse(&self) -> Self {
        Self {
            row: 7 - self.row,
            col: 7 - self.col,
        }
    }

    /// Convert to shakmaty Square
    /// Our board representation: row 0 is rank 8, row 7 is rank 1
    pub fn to_square(&self) -> Option<Square> {
        if !self.is_valid() {
            return None;
        }
        // Flip row: our row 0 = rank 8, row 7 = rank 1
        let rank = 7 - self.row;
        let file = self.col;
        Square::try_from(rank * 8 + file).ok()
    }

    /// Safe conversion to Square with validation
    pub fn try_to_square(&self) -> Option<Square> {
        self.to_square()
    }

    /// Convert from shakmaty Square
    pub fn from_square(square: Square) -> Self {
        let index = square as u8;
        let rank = index / 8;
        let file = index % 8;
        // Flip row back: rank 7 = our row 0, rank 0 = our row 7
        Self {
            row: 7 - rank,
            col: file,
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

    #[test]
    fn test_coord_square_conversion() {
        // Test e4 (row 4, col 4 in our system = rank 4, file e in chess)
        let coord = Coord::new(4, 4);
        let square = coord.to_square().unwrap();
        let back = Coord::from_square(square);
        assert_eq!(coord, back);
    }

    #[test]
    fn test_undefined() {
        let undef = Coord::undefined();
        assert!(!undef.is_valid());
        assert_eq!(undef.to_square(), None);
    }
}
