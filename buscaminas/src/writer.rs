use std::{fs::File, io::Write};

use crate::{cell::Cell, errors::GameError};

/// Receives a Cell matrix and prints the field with the * (bombs) and '.' (empty) characters.
///
///
/// # Examples
///
/// ```
/// use buscaminas::writer::show_field;
/// use buscaminas::cell::Cell;
/// let field = vec![
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
///         ]];
/// show_field(&field);
/// ```
///
/// The output will be:
///
/// .**.
///
/// ..*.
///
///
pub fn show_field(field: &Vec<Vec<Cell>>) {
    println!();
    println!("Result: ");
    for line in field {
        for cell in line {
            match cell {
                Cell::EmptyCell { bombs } => print!("{}", *bombs),
                Cell::BombCell => print!("*"),
            }
        }
        println!();
    }
}

/// This function creates a file named 'out.txt', or overwrites it if it does already exists and writes  the field with the * (bombs) and '.'(empty) characters.
///
/// # Errors
///
/// This function will return an error if the file could not be created (`GameError::CouldNotOpenFile`) or if the data could not be written on the file (`GameError::CouldNotOpenFile`)
///
/// # Examples
///
/// ```
/// use buscaminas::writer::write_file;
///
/// use buscaminas::cell::Cell;
/// let field = vec![
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
///         ]];
/// assert!(write_file(&field).is_ok());
///
/// ```
///
/// The output will be:
///
/// .**.
///
/// ..*.
///
pub fn write_file(data: &Vec<Vec<Cell>>) -> Result<(), GameError> {
    let path = "out.txt";

    let mut file = match File::create(path) {
        Ok(it) => it,
        Err(_) => return Err(GameError::CouldNotOpenFile),
    };

    write_data(data, &mut file)?;

    Ok(())
}

/// Receives a Cell matrix and an opened valid File. Translates the Cell field into chars, '*' for bombs and '.' for empty cells and writes it in the file.
///
/// # Errors
///
/// This function will return an error if it could not write the file correctly (`GameError::CoulNotWriteFile`).
fn write_data(data: &Vec<Vec<Cell>>, file: &mut File) -> Result<(), GameError> {
    for row in data {
        for cell in row {
            match cell {
                Cell::EmptyCell { bombs } => {
                    if write!(file, "{}", *bombs).is_err() {
                        return Err(GameError::CoulNotWriteFile);
                    }
                }
                Cell::BombCell => {
                    if write!(file, "*").is_err() {
                        return Err(GameError::CoulNotWriteFile);
                    }
                }
            }
        }
        if writeln!(file).is_err() {
            return Err(GameError::CoulNotWriteFile);
        }
    }
    Ok(())
}
