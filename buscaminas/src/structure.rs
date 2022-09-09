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

    pub fn rows(&self) -> usize {
        self.rows
    }

    pub fn columns(&self) -> usize {
        self.columns
    }
}

#[derive(Debug)]
pub struct Coordenada {
    row: usize,
    column: usize,
}

impl Coordenada {
    pub fn new(row: usize, column: usize) -> Self {
        Coordenada { row, column }
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
        coordenada: Coordenada,
    },
    BombCell {
        coordenada: Coordenada,
    },
}

impl Cell {
    pub fn new(element: u8, row: usize, column: usize) -> Self {
        if element == '*' as u8 {
            Cell::BombCell {
                coordenada: Coordenada::new(row, column),
            }
        } else {
            Cell::EmptyCell {
                bombs: 0,
                coordenada: Coordenada::new(row, column),
            }
        }
    }
}
