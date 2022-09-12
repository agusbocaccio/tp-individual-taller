/// There are two types of Cells. A Cell that represents an empty space and a Cell that represents a bomb.
///
#[derive(Debug, PartialEq, Eq)]
pub enum Cell {
    /// Represents an empty cell. The bombs usize represents the adjacent bombs to the cell. It will be traslated from and to the character '.'
    EmptyCell { bombs: usize },
    /// Represents a bomb cell. It will be traslated from and to the character '*'
    BombCell,
}

/// Binary character that represents a bomb.
const BOMB: u8 = b'*';

/// Binary character that represents an empty cell.
const EMPTY: u8 = b'.';

impl Cell {
    /// Creates a new `Cell`.
    ///
    /// It returns an Option containing the Cell created:
    /// If the element received is the character that represents a bomb (*), it will return the `BombCell` type.
    ///
    /// On the other hand, if the element received is the character that represents an empty field(.), it will return the `EmptyCell` instead.
    ///
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
    #[must_use]
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
