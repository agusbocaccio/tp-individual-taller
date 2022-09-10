extern crate buscaminas;

mod mine_finder_test {
    use buscaminas::{mine_finder::find_mines, cell::Cell};

    #[test]
    pub(crate) fn count_mines_squared_field() {
        let line1 = vec![Cell::new(b'*').unwrap(), Cell::new(b'.').unwrap()];
        let line2 = vec![Cell::new(b'*').unwrap(), Cell::new(b'.').unwrap()];

        let mut field = vec![line1, line2];
        find_mines(&mut field);

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
        find_mines(&mut field);

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
        find_mines(&mut field);

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
}
