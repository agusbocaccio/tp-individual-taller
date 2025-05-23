use crate::{cell::Cell, errors::GameError};

/// Receives a Cell matrix and counts the adjacents bombs, changing the original matrix so it shows the bombs and the number of adjacent bombs of the empty cells.
///
/// # Errors
///
/// This function will return an error if the field is not rectangular or squared (`GameError::InvalidField`) .
///
/// # Examples
///
/// ```
/// use buscaminas::mine_finder::find_mines;
/// use buscaminas::cell::Cell;
///
/// let line1 = vec![Cell::new(b'*').unwrap(), Cell::new(b'.').unwrap()];
/// let line2 = vec![Cell::new(b'*').unwrap(), Cell::new(b'.').unwrap()];
/// let mut field = vec![line1, line2];
///
/// let result = find_mines(&mut field);
///
/// assert!(result.is_ok());
/// assert_eq!(
///     field,
///     vec![
///         vec![Cell::BombCell, Cell::EmptyCell { bombs: (2) }],
///         vec![Cell::BombCell, Cell::EmptyCell { bombs: (2) }]
///     ]
/// )
/// ```
///

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

/// Returns true if any row of the received field is longer than any other.
fn invalid_field(field: &[Vec<Cell>]) -> bool {
    return field.iter().any(|line| line.len() != field[0].len());
}

/// Returns a valid usize range.
/// If i is zero or less than zero it will return a range (0..i).
/// If i is more than zero it will return a range (i-1..i+1).
fn get_range(i: usize) -> std::ops::RangeInclusive<usize> {
    if i == 0 {
        0..=i + 1
    } else {
        i - 1..=i + 1
    }
}

/// Returns true if row or column are out of the range of the field
fn outside_field(row: usize, column: usize, field: &Vec<Vec<Cell>>, i: usize) -> bool {
    (row >= field.len()) || (column >= (*field[i]).len())
}

#[cfg(test)]
mod mine_finder_test {
    use crate::cell::Cell;
    use crate::mine_finder::{find_mines, invalid_field, outside_field};

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

    #[test]
    pub(crate) fn row_and_column_inside_field() {
        let line1 = vec![
            Cell::new(b'.').unwrap(),
            Cell::new(b'*').unwrap(),
            Cell::new(b'*').unwrap(),
        ];
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

        let field = vec![line1, line2, line3];
        assert!(!outside_field(2, 2, &field, 0));
    }

    #[test]
    pub(crate) fn row_outside_field() {
        let line1 = vec![
            Cell::new(b'.').unwrap(),
            Cell::new(b'*').unwrap(),
            Cell::new(b'*').unwrap(),
        ];
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

        let field = vec![line1, line2, line3];
        assert!(outside_field(4, 2, &field, 0));
    }

    #[test]
    pub(crate) fn column_outside_field() {
        let line1 = vec![
            Cell::new(b'.').unwrap(),
            Cell::new(b'*').unwrap(),
            Cell::new(b'*').unwrap(),
        ];
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

        let field = vec![line1, line2, line3];
        assert!(outside_field(2, 3, &field, 0));
    }

    #[test]
    pub(crate) fn field_with_different_length_row_is_invalid() {
        let line1 = vec![
            Cell::new(b'.').unwrap(),
            Cell::new(b'*').unwrap(),
            Cell::new(b'*').unwrap(),
        ];
        let line2 = vec![Cell::new(b'.').unwrap(), Cell::new(b'*').unwrap()];
        let line3 = vec![
            Cell::new(b'.').unwrap(),
            Cell::new(b'*').unwrap(),
            Cell::new(b'*').unwrap(),
        ];

        let field = vec![line1, line2, line3];
        assert!(invalid_field(&field));
    }
}
