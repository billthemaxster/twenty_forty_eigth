use rand::prelude::*;

pub enum MoveDirection {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Copy, Debug)]
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

    pub fn get_empty_positions(&self) -> Vec<GridCoord> {
        let mut tiles: Vec<GridCoord> = vec![];
        for x in 0..self.size {
            for y in 0..self.size {
                let tile_square = self.get_tile(GridCoord { x, y }).unwrap();

                if tile_square.is_none() {
                    tiles.push(GridCoord { x, y });
                }
            }
        }

        tiles
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

        let mut game = Game { score, grid };

        // start with two tiles
        game.add_random_tile()?;
        game.add_random_tile()?;

        Ok(game)
    }

    pub fn add_random_tile(&mut self) -> Result<(), &'static str> {
        let tile_value = Game::generate_tile_value();
        let tile_position = self.generate_random_empty_position()?;

        self.grid.add_new_tile(tile_value, tile_position)?;

        Ok(())
    }

    fn generate_random_empty_position(&self) -> Result<GridCoord, &'static str> {
        let empty_tiles = self.grid.get_empty_positions();

        if empty_tiles.len() == 0 {
            return Err("No empty positions remain.");
        }

        let mut rng = thread_rng();
        let index = rng.gen_range(0, empty_tiles.len());

        Ok(empty_tiles[index])
    }

    fn generate_tile_value() -> u16 {
        let mut rng = thread_rng();

        let seed = rng.gen_range(0, 2);

        match seed {
            0 => 2,
            1 => 4,
            _ => panic!("Should not be possible"),
        }
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

        mod get_empty_positions {
            use super::*;
            use rstest::rstest;

            #[rstest(size, case(2), case(4), case(5))]
            fn return_all_squares_when_grid_is_empty(size: u8) {
                let grid = Grid::new(size).unwrap();

                let empty_positions = grid.get_empty_positions();

                let expected_size: usize = (size * size).into();

                assert_eq!(empty_positions.len(), expected_size);
            }

            #[rstest(
                size,
                populated_squares,
                case(2, 2),
                case(2, 4),
                case(3, 2),
                case(3, 4),
                case(5, 5),
                case(5, 20)
            )]
            fn return_correct_number_of_squares_when_given_number_are_populated(
                size: u8,
                populated_squares: u8,
            ) {
                let mut grid = Grid::new(size).unwrap();

                for i in 0..populated_squares {
                    let position = GridCoord {
                        x: i / size,
                        y: i % size,
                    };
                    let _ = grid.add_new_tile(2, position).unwrap();
                }

                let expected_squares_count: usize = (size * size - populated_squares).into();

                let empty_positions = grid.get_empty_positions();
                assert_eq!(empty_positions.len(), expected_squares_count);
            }
        }
    }
}
