use std::io;

use buscaminas::{errors::GameError, mine_finder::find_mines, reader::read_file, structure::Cell};

fn ask_path() -> String {
    let mut path = String::new();
    println!("Pasame el path del archivo: ");
    io::stdin()
        .read_line(&mut path)
        .expect("Failed to read line");

    path.trim().to_string()
}

fn main() -> Result<(), GameError> {
    let path = ask_path();
    let mut field = read_file(path)?;
    find_mines(&mut field);

    for line in field {
        for cell in line {
            match cell {
                Cell::EmptyCell { bombs } => print!("{}", bombs),
                Cell::BombCell => print!("*"),
            }
        }
        println!();
    }

    Ok(())
}
