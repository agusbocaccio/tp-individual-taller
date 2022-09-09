extern crate buscaminas;

use buscaminas::{errors::GameError, reader::read_file};

#[test]
fn open_non_existing_file() {
    let value = read_file();
    assert_eq!(value.err(), Some(GameError::CouldNotOpenFile));
}
