pub enum MoveDirection {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Debug)]
pub struct GridCoord {
    pub x: u8,
    pub y: u8,
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

    pub fn add_new_tile(&mut self, value: u16, position: GridCoord) -> Result<(), &'static str> {
        let x: usize = position.x.into();
        let y: usize = position.y.into();

        if let Some(_tile) = &self.get_tile(position)? {
            return Err("cannot add a new tile where a tile already exists");
        }

        self.data[x][y] = Some(Tile { value });

        Ok(())
    }

    pub fn get_tile(&self, position: GridCoord) -> Result<Option<&Tile>, &'static str> {
        if self.size < position.x || self.size < position.y {
            return Err("can't access a position greater than grid size");
        }

        let x: usize = position.x.into();
        let y: usize = position.y.into();

        Ok(self.data[x][y].as_ref())
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

        mod add_new_tile {
            use super::*;

            #[test]
            fn adds_a_tile_of_given_value_to_the_given_position() {
                let mut grid = Grid::new(2).unwrap();

                let expected_value: u16 = 2;
                let x = 0;
                let y = 0;
                grid.add_new_tile(expected_value, GridCoord { x, y })
                    .unwrap();

                let x: usize = x.into();
                let y: usize = y.into();
                let result = &grid.data[x][y];
                match result {
                    None => assert!(false),
                    Some(tile) => assert_eq!(tile.value, expected_value),
                };
            }

            #[test]
            fn does_not_add_a_tile_if_one_is_present_in_given_location() {
                let mut grid = Grid::new(2).unwrap();

                grid.data[0][0] = Some(Tile { value: 16 });
                let result = grid.add_new_tile(2, GridCoord { x: 0, y: 0 });

                assert_eq!(
                    result.unwrap_err(),
                    "cannot add a new tile where a tile already exists"
                );
            }
        }

        mod get_tile {
            use super::*;
            use rstest::rstest;

            #[test]
            fn returns_tile_for_valid_location() {
                let mut grid = Grid::new(2).unwrap();
                let expected_value = 16;
                let expected_tile = Some(Tile {
                    value: expected_value,
                });
                grid.data[0][0] = expected_tile;

                let result = grid.get_tile(GridCoord { x: 0, y: 0 }).unwrap();

                match result {
                    Some(tile) => assert_eq!(expected_value, tile.value),
                    None => assert!(false),
                };
            }

            #[rstest(x, y, size, case(0, 3, 2), case(3, 0, 2))]
            fn errors_if_position_outside_of_grid(x: u8, y: u8, size: u8) {
                let grid = Grid::new(size).unwrap();

                let result = grid.get_tile(GridCoord { x, y });

                assert_eq!(
                    result.unwrap_err(),
                    "can't access a position greater than grid size"
                );
            }
        }
    }
}
