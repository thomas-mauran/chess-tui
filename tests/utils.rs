#[cfg(test)]
mod tests {
    use chess_tui::utils::{convert_notation_into_position, convert_position_into_notation};

    #[test]
    fn convert_position_into_notation_1() {
        assert_eq!(convert_position_into_notation("7152"), "b1-c3")
    }

    #[test]
    fn convert_position_into_notation_2() {
        assert_eq!(convert_position_into_notation("0257"), "c8-h3")
    }

    #[test]
    fn convert_notation_into_position_1() {
        assert_eq!(convert_notation_into_position("c8b7"), "0211")
    }
    #[test]
    fn convert_notation_into_position_2() {
        assert_eq!(convert_notation_into_position("g7h8"), "1607")
    }
    #[test]
    fn convert_notation_into_position_3() {
        assert_eq!(convert_notation_into_position("g1f3"), "7655")
    }
}
