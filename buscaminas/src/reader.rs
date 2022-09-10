use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use crate::{errors::GameError, structure::Cell};

pub fn read_file(path: String) -> Result<Vec<Vec<Cell>>, GameError> {
    let file = match File::open(path) {
        Ok(it) => it,
        Err(_) => return Err(GameError::CouldNotOpenFile),
    };

    let buffered_reader = BufReader::new(file);
    let mut field = vec![];

    for line in buffered_reader.lines() {
        let mut row = vec![];
        if let Ok(it) = line {
            for element in it.as_bytes().iter() {
                row.push(Cell::new(*element));
            }
        } else {
            return Err(GameError::CouldNotReadFile);
        };
        field.push(row);
    }

    Ok(field)
}
