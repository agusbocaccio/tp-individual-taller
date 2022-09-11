use std::io;

use buscaminas::{errors::GameError, mine_finder::find_mines, reader::read_file, writer::{write_file, show_field}};

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
    find_mines(&mut field)?;

    show_field(&field);
    write_file(&field)?;

    Ok(())
}
