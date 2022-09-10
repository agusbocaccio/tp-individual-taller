extern crate buscaminas;

use buscaminas::{reader::read_file};

#[test]
fn open_non_existing_file() {
    let result = read_file("no_existe.txt".to_string());
    assert!(result.is_err());
}
