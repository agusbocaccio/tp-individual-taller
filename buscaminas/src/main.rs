use std::{io, slice::range, vec};

use buscaminas::{
    errors::GameError,
    reader::read_file,
    structure::{Cell, Coordinate, Field},
};

fn ask_path() -> String {
    let mut path = String::new();
    println!("Pasame el path del archivo: ");
    io::stdin()
        .read_line(&mut path)
        .expect("Failed to read line");

    path
}

fn main() -> Result<(), GameError> {
    let mut field = read_file()?;

    let mut new_cells = vec![];
    println!("{:?}", field);
    for element in field.cells().iter() {
        let mut adjacent_bombs = 0;
        match element {
            Cell::EmptyCell {
                bombs: _,
                coordinate,
            } => {
                let row = coordinate.row();
                let column = coordinate.column();
                println!("{:?}", range(column - 1, column + 1));
                for i in (column - 1..column + 1) {
                    for j in ((row - 1)..(row + 1)) {
                        if let Some(cell_adjacent) = field.get_cell(j, i) {
                            match cell_adjacent {
                                Cell::EmptyCell { bombs, coordinate } => (),
                                Cell::BombCell { coordinate, symbol } => {
                                    adjacent_bombs = adjacent_bombs + 1
                                }
                            }
                        }
                    }
                }
                new_cells.push((element, adjacent_bombs))
            }
            Cell::BombCell { coordinate, symbol } => new_cells.push((element, 0)),
        }
    }

    println!("{:?}", new_cells);

    Ok(())
}

fn add_adjacent(
    coordinate: &Coordinate,
    element: &mut Cell,
    field: &Field,
) -> Result<(), GameError> {
    let row = coordinate.row();
    let column = coordinate.column();

    for i in (column - 1..column + 1) {
        for j in (row - 1..row + 1) {
            if let Some(_) = field.get_cell(j, i) {
                element.add_bomb_adjacent()?;
            }
        }
    }
    Ok(())
}
