use std::{fs::File, io::Write};

use crate::{errors::GameError, cell::Cell};

pub fn show_field(field: &Vec<Vec<Cell>>) {
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
