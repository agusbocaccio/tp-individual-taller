#[cfg(test)]
mod integration_test {
    use std::{
        fs::File,
        io::{BufRead, BufReader},
    };

    use buscaminas::{cell::Cell, mine_finder::find_mines, reader::read_file, writer::write_file};

    // field: 
    // ..*.*.
    // *.*...
    // *.*.*.
    //
    //expected output: 
    //13*3*1
    //*5*522
    //*4*3*1
    //
    #[test]
    pub(crate) fn transform_correctly_valid_file() {
        let result_reading = read_file("files/other_field.txt".to_string());
        assert!(result_reading.is_ok());

        let mut field = result_reading.unwrap();

        let line1 = vec![
            Cell::EmptyCell { bombs: (0) },
            Cell::EmptyCell { bombs: (0) },
            Cell::BombCell,
            Cell::EmptyCell { bombs: (0) },
            Cell::BombCell,
            Cell::EmptyCell { bombs: (0) },
        ];
        let line2 = vec![
            Cell::BombCell,
            Cell::EmptyCell { bombs: (0) },
            Cell::BombCell,
            Cell::EmptyCell { bombs: (0) },
            Cell::EmptyCell { bombs: (0) },
            Cell::EmptyCell { bombs: (0) },
        ];
        let line3 = vec![
            Cell::BombCell,
            Cell::EmptyCell { bombs: (0) },
            Cell::BombCell,
            Cell::EmptyCell { bombs: (0) },
            Cell::BombCell,
            Cell::EmptyCell { bombs: (0) },
        ];

        let expected_field = vec![line1, line2, line3];
        assert_eq!(field, expected_field);

        let transformation_result = find_mines(&mut field);
        assert!(transformation_result.is_ok());

        assert_eq!(
            field,
            vec![
                vec![
                    Cell::EmptyCell { bombs: (1) },
                    Cell::EmptyCell { bombs: (3) },
                    Cell::BombCell,
                    Cell::EmptyCell { bombs: (3) },
                    Cell::BombCell,
                    Cell::EmptyCell { bombs: (1) },
                ],
                vec![
                    Cell::BombCell,
                    Cell::EmptyCell { bombs: (5) },
                    Cell::BombCell,
                    Cell::EmptyCell { bombs: (5) },
                    Cell::EmptyCell { bombs: (2) },
                    Cell::EmptyCell { bombs: (2) },
                ],
                vec![
                    Cell::BombCell,
                    Cell::EmptyCell { bombs: (4) },
                    Cell::BombCell,
                    Cell::EmptyCell { bombs: (3) },
                    Cell::BombCell,
                    Cell::EmptyCell { bombs: (1) },
                ]
            ]
        );

        let result_writing = write_file(&field);
        assert!(result_writing.is_ok());

        let file = File::open("out.txt").unwrap();

        let buffered_reader = BufReader::new(file);
        let mut lines_written = vec![];

        for line in buffered_reader.lines() {
            if let Ok(it) = line {
                lines_written.push(it)
            }
        }

        assert_eq!(lines_written, vec!["13*3*1", "*5*522", "*4*3*1"])
    }
}
