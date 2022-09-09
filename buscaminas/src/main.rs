use std::io;

use buscaminas::{errors::GameError, reader::read_file, structure::Cell};

fn ask_path() -> String {
    let mut path = String::new();
    println!("Pasame el path del archivo: ");
    io::stdin()
        .read_line(&mut path)
        .expect("Failed to read line");

    path
}

fn main() -> Result<(), GameError> {
    let field = read_file()?;
    println!("{:?}", field);
    for element in field.cells() {
        check_adjacent(element);
    }

    Ok(())
}

fn check_adjacent(element: &Cell) {
    return;
}
