#[derive(Debug, PartialEq, Eq)]
pub enum Cell {
    EmptyCell { bombs: usize },
    BombCell,
}

const BOMB: u8 = b'*';
const EMPTY: u8 = b'.';

impl Cell {
    /// Creates a new `Cell`.
    ///
    /// It returns an Option containing the Cell created:
    /// If the element received is the character that represents a bomb (*), it will return the BombCell type.
    /// On the other case, if the element received is the character that represents an empty field(.), it will return the EmptyCell instead
    /// On every other case, it returns None
    ///
    /// Basic usage:
    ///
    /// # Examples
    ///
    /// ```
    /// use buscaminas::cell::Cell;
    ///
    /// assert_eq!(Cell::new('*' as u8).unwrap(), Cell::BombCell);
    /// assert_eq!(Cell::new('=' as u8), None);
    /// 
    /// ```
    /// 
    
    pub fn new(element: u8) -> Option<Self> {
        if BOMB == element {
            Some(Cell::BombCell)
        } else if EMPTY == element {
            Some(Cell::EmptyCell { bombs: 0 })
        } else {
            None
        }
    }
}

#[cfg(test)]
mod structure_test {
    use crate::cell::Cell;

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
