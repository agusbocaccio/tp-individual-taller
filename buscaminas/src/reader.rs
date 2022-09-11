use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use crate::{errors::GameError, cell::Cell};

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
                match Cell::new(*element) {
                    Some(it) => row.push(it),
                    None => return Err(GameError::InvalidCharacter),
                };
            }
        } else {
            return Err(GameError::CouldNotReadFile);
        };
        field.push(row);
    }

    Ok(field)
}



#[cfg(test)]
mod reader_test {
    use crate::reader::read_file;

    #[test]
    pub(crate) fn open_non_existing_file() {
        let result = read_file("no_existe.txt".to_string());
        assert!(result.is_err());
    }

    #[test]
    pub(crate) fn open_existing_file() {
        let result = read_file("files/field.txt".to_string());
        assert!(result.is_ok());
    }

    #[test]
    pub(crate) fn open_invalid_file() {
        let result = read_file("files/invalid_field.txt".to_string());
        assert!(result.is_err());
    }

    #[test]
    pub(crate) fn open_empty_file() {
        let result = read_file("files/empty_field.txt".to_string());
        assert!(result.is_ok());
    }
}
