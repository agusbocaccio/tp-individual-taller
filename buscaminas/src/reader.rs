use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use crate::{cell::Cell, errors::GameError};

/// This function receives a file path and opens and reads it, transforming the file information into a Cell matrix and returning it.
/// The valid characters are the ASCII characters '*' (bomb) and '.' (empty).
/// # Errors
///
/// This function will return an error if the file has an invalid character (`GameError::InvalidCharacter`), if the file could not be opened (`GameError::CouldNotOpenFile`) or if there ir an error while reading the file (`GameError::CouldNotReadFile`)
///
/// # Examples
///
/// ```
/// use buscaminas::reader::read_file;
/// use buscaminas::cell::Cell;
///
/// // If he file has the following structure:
/// // .**.
/// // ..*.
///
/// assert_eq!(read_file("files/small_field.txt".to_string()).unwrap(),
///   vec![
///       vec![
///           Cell::EmptyCell { bombs: (0) },
///           Cell::BombCell,
///           Cell::BombCell,
///           Cell::EmptyCell { bombs: (0) }
///       ],
///       vec![
///           Cell::EmptyCell { bombs: (0) },
///           Cell::EmptyCell { bombs: (0) },
///           Cell::BombCell,
///           Cell::EmptyCell { bombs: (0) },
///         ],
/// ]
/// )
/// ```
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
    use crate::{cell::Cell, reader::read_file};

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
        let result = read_file("files/empty_file.txt".to_string());
        assert!(result.is_ok());
    }

    #[test]
    pub(crate) fn read_valid_file() {
        assert_eq!(
            read_file("files/field.txt".to_string()).unwrap(),
            vec![
                vec![
                    Cell::EmptyCell { bombs: (0) },
                    Cell::BombCell,
                    Cell::EmptyCell { bombs: (0) },
                    Cell::BombCell,
                    Cell::EmptyCell { bombs: (0) }
                ],
                vec![
                    Cell::EmptyCell { bombs: (0) },
                    Cell::EmptyCell { bombs: (0) },
                    Cell::BombCell,
                    Cell::EmptyCell { bombs: (0) },
                    Cell::EmptyCell { bombs: (0) }
                ],
                vec![
                    Cell::EmptyCell { bombs: (0) },
                    Cell::EmptyCell { bombs: (0) },
                    Cell::BombCell,
                    Cell::EmptyCell { bombs: (0) },
                    Cell::EmptyCell { bombs: (0) }
                ],
                vec![
                    Cell::EmptyCell { bombs: (0) },
                    Cell::EmptyCell { bombs: (0) },
                    Cell::EmptyCell { bombs: (0) },
                    Cell::EmptyCell { bombs: (0) },
                    Cell::EmptyCell { bombs: (0) }
                ],
            ]
        )
    }
}
