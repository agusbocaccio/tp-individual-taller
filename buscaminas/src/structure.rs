#[derive(Debug)]
pub enum Cell {
    EmptyCell { bombs: usize },
    BombCell,
}

impl Cell {
    pub fn new(element: u8) -> Self {
        if b'*' == element {
            Cell::BombCell
        } else {
            Cell::EmptyCell { bombs: 0 }
        }
    }
}
