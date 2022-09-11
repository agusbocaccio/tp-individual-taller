use crate::{cell::Cell, errors::GameError};

pub fn find_mines(field: &mut Vec<Vec<Cell>>) -> Result<(), GameError> {
    if invalid_field(field) {
        return Err(GameError::InvalidField);
    }
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
    Ok(())
}

fn invalid_field(field: &mut Vec<Vec<Cell>>) -> bool {
    return field.iter().any(|line| line.len() != field[0].len());
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

#[cfg(test)]
mod mine_finder_test {
    use crate::cell::Cell;
    use crate::mine_finder::find_mines;

    #[test]
    pub(crate) fn count_mines_squared_field() {
        let line1 = vec![Cell::new(b'*').unwrap(), Cell::new(b'.').unwrap()];
        let line2 = vec![Cell::new(b'*').unwrap(), Cell::new(b'.').unwrap()];

        let mut field = vec![line1, line2];
        let result = find_mines(&mut field);

        assert!(result.is_ok());
        assert_eq!(
            field,
            vec![
                vec![Cell::BombCell, Cell::EmptyCell { bombs: (2) }],
                vec![Cell::BombCell, Cell::EmptyCell { bombs: (2) }]
            ]
        )
    }

    #[test]
    pub(crate) fn count_mines_line_field() {
        let line1 = vec![
            Cell::new(b'*').unwrap(),
            Cell::new(b'.').unwrap(),
            Cell::new(b'*').unwrap(),
            Cell::new(b'.').unwrap(),
            Cell::new(b'*').unwrap(),
        ];
        let mut field = vec![line1];
        let result = find_mines(&mut field);

        assert!(result.is_ok());
        assert_eq!(
            field,
            vec![vec![
                Cell::BombCell,
                Cell::EmptyCell { bombs: (2) },
                Cell::BombCell,
                Cell::EmptyCell { bombs: (2) },
                Cell::BombCell
            ]]
        )
    }

    #[test]
    pub(crate) fn count_mines_empty_field() {
        let line1 = vec![
            Cell::new(b'.').unwrap(),
            Cell::new(b'.').unwrap(),
            Cell::new(b'.').unwrap(),
        ];
        let line2 = vec![
            Cell::new(b'.').unwrap(),
            Cell::new(b'.').unwrap(),
            Cell::new(b'.').unwrap(),
        ];
        let line3 = vec![
            Cell::new(b'.').unwrap(),
            Cell::new(b'.').unwrap(),
            Cell::new(b'.').unwrap(),
        ];
        let line4 = vec![
            Cell::new(b'.').unwrap(),
            Cell::new(b'.').unwrap(),
            Cell::new(b'.').unwrap(),
        ];

        let mut field = vec![line1, line2, line3, line4];
        let result = find_mines(&mut field);

        assert!(result.is_ok());
        assert_eq!(
            field,
            vec![
                vec![
                    Cell::EmptyCell { bombs: (0) },
                    Cell::EmptyCell { bombs: (0) },
                    Cell::EmptyCell { bombs: (0) },
                ],
                vec![
                    Cell::EmptyCell { bombs: (0) },
                    Cell::EmptyCell { bombs: (0) },
                    Cell::EmptyCell { bombs: (0) },
                ],
                vec![
                    Cell::EmptyCell { bombs: (0) },
                    Cell::EmptyCell { bombs: (0) },
                    Cell::EmptyCell { bombs: (0) },
                ],
                vec![
                    Cell::EmptyCell { bombs: (0) },
                    Cell::EmptyCell { bombs: (0) },
                    Cell::EmptyCell { bombs: (0) },
                ]
            ]
        )
    }

    #[test]
    pub(crate) fn count_mines_columns_field() {
        let line1 = vec![Cell::new(b'.').unwrap(), Cell::new(b'*').unwrap()];
        let line2 = vec![
            Cell::new(b'.').unwrap(),
            Cell::new(b'*').unwrap(),
            Cell::new(b'.').unwrap(),
        ];
        let line3 = vec![
            Cell::new(b'.').unwrap(),
            Cell::new(b'*').unwrap(),
            Cell::new(b'*').unwrap(),
        ];

        let mut field = vec![line1, line2, line3];
        let result = find_mines(&mut field);

        assert!(result.is_err())
    }
}
