extern crate buscaminas;

mod structure_test {
    use buscaminas::cell::Cell;

    #[test]
    pub(crate) fn create_bomb_cell() {
        let result = Cell::new(b'*');
        assert_eq!(result.unwrap(), Cell::BombCell)
    }

    #[test]
    pub(crate) fn create_empty_cell() {
        let result = Cell::new(b'.');
        assert_eq!(result.unwrap(), Cell::EmptyCell { bombs: (0) })
    }

    #[test]
    pub(crate) fn create_cell_invalid_character() {
        let result = Cell::new(b'e');
        assert!(result.is_none())
    }
}
