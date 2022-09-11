use buscaminas::{
    ask_path::ask_path,
    errors::GameError,
    mine_finder::find_mines,
    reader::read_file,
    writer::{show_field, write_file},
};

fn main() -> Result<(), GameError> {
    let path = ask_path()?;
    let mut field = read_file(path)?;
    find_mines(&mut field)?;

    show_field(&field);
    write_file(&field)?;

    Ok(())
}
