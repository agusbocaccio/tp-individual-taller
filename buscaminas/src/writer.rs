use std::{fs::File, io::Write};

use crate::{cell::Cell, errors::GameError};

/// Receives a Cell matrix and prints the field with the * (bombs) and '.'(empty) characters.
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

pub fn write_file(data: &Vec<Vec<Cell>>) -> Result<(), GameError> {
    let path = "out.txt";

    let mut file = match File::create(path) {
        Ok(it) => it,
        Err(_) => return Err(GameError::CouldNotOpenFile),
    };

    if let Some(value) = write_data(data, &mut file) {
        return value;
    }

    Ok(())
}

fn write_data(data: &Vec<Vec<Cell>>, file: &mut File) -> Option<Result<(), GameError>> {
    for row in data {
        for cell in row {
            match cell {
                Cell::EmptyCell { bombs } => {
                    if write!(file, "{}", *bombs).is_err() {
                        return Some(Err(GameError::CoulNotWriteFile));
                    }
                }
                Cell::BombCell => {
                    if write!(file, "*").is_err() {
                        return Some(Err(GameError::CoulNotWriteFile));
                    }
                }
            }
        }
        if writeln!(file).is_err() {
            return Some(Err(GameError::CoulNotWriteFile));
        }
    }
    None
}
