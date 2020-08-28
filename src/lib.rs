pub enum MoveDirection {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Debug)]
pub struct Tile {
    pub value: u16,
}

#[derive(Debug)]
pub struct Grid {
    data: Vec<Vec<Option<Tile>>>,
    size: u8,
}

impl Grid {
    pub fn new(size: u8) -> Result<Grid, &'static str> {
        if size < 2 {
            return Err("cannot create a game with a grid smaller than 2x2.");
        }

        let array_size: usize = size.into();
        let data: Vec<Vec<Option<Tile>>> = vec![vec![None; array_size]; array_size];

        Ok(Grid { size, data })
    }

    pub fn add_new_tile(value: u16) -> Result<(), &'static str> {
        Err("not implemented")
    }
}

#[derive(Debug)]
pub struct Game {
    pub score: u32,
    pub grid: Grid,
}

impl Game {
    pub fn new(size: u8) -> Result<Game, &'static str> {
        let score = 0;
        let grid: Grid = Grid::new(size)?;

        Ok(Game { score, grid })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod game {
        use super::*;

        mod new {
            use super::*;
            use rstest::rstest;

            #[test]
            fn set_score_to_zero() {
                let result = Game::new(2).unwrap();

                assert_eq!(result.score, 0)
            }

            #[rstest(input, case(2), case(4), case(5))]
            fn set_tiles_to_grid_of_given_size(input: u8) {
                let result = Game::new(input).unwrap();

                assert_eq!(result.grid.size, input);
            }

            #[rstest(input, case(0), case(1))]
            fn invalid_grid_size_requested_err_returned(input: u8) {
                let result = Game::new(input);

                assert_eq!(
                    result.unwrap_err(),
                    "cannot create a game with a grid smaller than 2x2."
                );
            }
        }
    }

    mod grid {
        use super::*;

        mod new {
            use super::*;
            use rstest::rstest;

            #[rstest(expected_size, case(2), case(4), case(5))]
            fn set_size_to_given_size(expected_size: u8) {
                let result = Grid::new(expected_size).unwrap();

                assert_eq!(result.size, expected_size);
            }

            #[rstest(invalid_size, case(0), case(1))]
            fn error_on_an_invalid_size(invalid_size: u8) {
                let result = Game::new(invalid_size);

                assert_eq!(
                    result.unwrap_err(),
                    "cannot create a game with a grid smaller than 2x2."
                );
            }

            #[rstest(expected_size, case(2), case(4), case(5))]
            fn set_dimensions_of_grid_data_to_given_size(expected_size: u8) {
                let result = Grid::new(expected_size).unwrap();

                let expected_length: usize = expected_size.into();
                assert_eq!(result.data.len(), expected_length);

                for row in result.data.iter() {
                    assert_eq!(row.len(), expected_length);
                }
            }
        }
    }
}
