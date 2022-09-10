#[derive(Debug)]
#[derive(PartialEq)]
pub enum Cell {
    EmptyCell { bombs: usize },
    BombCell,
}

impl Cell {
    pub fn new(element: u8) -> Option<Self> {
        if b'*' == element {
            Some(Cell::BombCell)
        } else if b'.' == element {
            Some(Cell::EmptyCell { bombs: 0 })
        } else {
            None
        }
    }
}
