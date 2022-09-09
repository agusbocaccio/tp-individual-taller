use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use crate::{
    errors::GameError,
    structure::{Cell, Field},
};

pub fn read_file() -> Result<Field, GameError> {
    let file = match File::open("src/campo.txt") {
        Ok(it) => it,
        Err(_) => return Err(GameError::CouldNotOpenFile),
    };

    let buffered_reader = BufReader::new(file);
    let mut cells = vec![];
    let mut rows = 0;
    let mut columns = 0;

    for (row, line) in buffered_reader.lines().enumerate() {
        if let Some(value) = transform_line(line, &mut columns, &mut cells, row, &mut rows) {
            return value;
        }
    }


    Ok(Field::new(cells, rows, columns))
}

fn transform_line(line: Result<String, std::io::Error>, columns: &mut usize, cells: &mut Vec<Cell>, row: usize, rows: &mut usize) -> Option<Result<Field, GameError>> {
    if let Ok(it) = line {
        *columns = it.len();
        for (column, element) in it.as_bytes().iter().enumerate() {
            cells.push(Cell::new(*element, row, column));
        }
    } else {
        return Some(Err(GameError::CouldNotReadFile));
    };
    *rows = *rows + 1;
    None
}
