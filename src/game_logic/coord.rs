use crate::constants::UNDEFINED_POSITION;

#[derive(PartialEq, Clone, Debug, Eq, PartialOrd, Ord, Copy)]
pub struct Coord {
    /// rank, horizontal row, line, y axis
    pub row: u8,
    /// file, vertical column, x axis
    pub col: u8,
}
impl Coord {
    pub fn new<U1: Into<u8>, U2: Into<u8>>(row: U1, col: U2) -> Self {
        Coord {
            row: row.into(),
            col: col.into(),
        }
    }
    /// optional new: try making a valid [`Coord`], if can't, return [`None`]
    pub fn opt_new<U1: TryInto<u8>, U2: TryInto<u8>>(row: U1, col: U2) -> Option<Self> {
        let row: u8 = row.try_into().ok()?;
        let col: u8 = col.try_into().ok()?;

        let ret = Coord { row, col };
        if ret.is_valid() {
            Some(ret)
        } else {
            None
        }
    }
    /// not yet set position, has to later be set and only used afterwards
    pub fn undefined() -> Self {
        Coord {
            row: UNDEFINED_POSITION,
            col: UNDEFINED_POSITION,
        }
    }
    /// checks whether `self` is valid as a chess board coordinate
    pub fn is_valid(&self) -> bool {
        (0..8).contains(&self.col) && (0..8).contains(&self.row)
    }
}
