use crate::errors::GameError;

#[derive(Debug)]
pub struct Field {
    cells: Vec<Cell>,
    rows: usize,
    columns: usize,
}

impl Field {
    pub fn new(cells: Vec<Cell>, rows: usize, columns: usize) -> Self {
        Field {
            cells,
            rows,
            columns,
        }
    }

    pub fn cells(&self) -> &[Cell] {
        self.cells.as_ref()
    }
    

    pub fn get_cell(&self, row: usize, column: usize) -> Option<&Cell> {
        for cell in self.cells.iter() {
            if cell.row() == row && cell.column() == column {
                return Some(cell);
            }
        }
        None
    }

    pub fn rows(&self) -> usize {
        self.rows
    }

    pub fn columns(&self) -> usize {
        self.columns
    }
}

#[derive(Debug)]
pub struct Coordinate {
    row: usize,
    column: usize,
}

impl Coordinate {
    pub fn new(row: usize, column: usize) -> Self {
        Coordinate { row, column }
    }

    pub fn row(&self) -> usize {
        self.row
    }

    pub fn column(&self) -> usize {
        self.column
    }
}

#[derive(Debug)]
pub enum Cell {
    EmptyCell {
        bombs: usize,
        coordinate: Coordinate,
    },
    BombCell {
        coordinate: Coordinate,
        symbol: u8,
    },
}

impl Cell {
    pub fn new(element: u8, row: usize, column: usize) -> Self {
        if element == '*' as u8 {
            Cell::BombCell {
                symbol: element,
                coordinate: Coordinate::new(row, column),
            }
        } else {
            Cell::EmptyCell {
                bombs: 0,
                coordinate: Coordinate::new(row, column),
            }
        }
    }

    pub fn row(&self) -> usize {
        match self {
            Cell::EmptyCell {
                bombs: _,
                coordinate,
            } => coordinate.row(),
            Cell::BombCell {
                coordinate,
                symbol: _,
            } => coordinate.row(),
        }
    }

    pub fn column(&self) -> usize {
        match self {
            Cell::EmptyCell {
                coordinate,
                bombs: _,
            } => coordinate.column(),
            Cell::BombCell {
                coordinate,
                symbol: _,
            } => coordinate.column(),
        }
    }

    pub fn add_bomb_adjacent(&mut self) -> Result<(), GameError> {
        match self {
            Cell::EmptyCell {
                coordinate: _,
                ref mut bombs,
            } => {
                *bombs = *bombs + 1;
                Ok(())
            }
            Cell::BombCell {
                coordinate: _,
                symbol: _,
            } => Err(GameError::BombCellCantHaceAdjacentBombs),
        }
    }
}
