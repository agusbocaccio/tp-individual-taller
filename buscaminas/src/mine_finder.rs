use crate::structure::Cell;

pub fn find_mines(field: &mut Vec<Vec<Cell>>) {
    for i in 0..field.len() {
        for j in 0..(*field[i]).len() {
            if let Cell::EmptyCell { bombs: _ } = (*field[i])[j] {
                let mut bombs = 0;
                get_range(i).for_each(|k| {
                    get_range(j).for_each(|l| {
                        if !(outside_field(k, l, field, i)) {
                            if let Cell::BombCell = (*field[k])[l] {
                                bombs += 1;
                            }
                        }
                    });
                });
                (*field[i])[j] = Cell::EmptyCell { bombs: (bombs) };
            }
        }
    }
}

fn get_range(i: usize) -> std::ops::RangeInclusive<usize> {
    if i == 0 {
        0..=i + 1
    } else {
        i - 1..=i + 1
    }
}

fn outside_field(k: usize, l: usize, field: &mut Vec<Vec<Cell>>, i: usize) -> bool {
    (k >= field.len()) || (l >= (*field[i]).len())
}
