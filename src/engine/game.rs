use crate::engine::grid::*;
use crate::engine::grid_coord::GridCoord;
use rand::prelude::*;

#[derive(Debug)]
pub struct Game {
    score: u32,
    grid: Grid,
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

    pub fn get_score(&self) -> u32 {
        self.score
    }

    pub fn get_grid(&self) -> &Grid {
        &self.grid
    }

    fn add_random_tile(&mut self) -> Result<(), &'static str> {
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

    pub fn perform_move(&mut self, direction: MoveDirection) -> Result<(), &'static str> {
        // combine tiles in the given direction
        match direction {
            MoveDirection::Down => self.perform_move_down(),
            _ => panic!(),
        };

        // if any tiles have combined, update the score

        // if there's been any movement, add a new random tile

        Ok(())
    }

    fn perform_move_down(&mut self) -> Result<Vec<Tile>, &'static str> {
        // for each column
        for y in 0..self.grid.get_size() {
            loop { // do-while
                let shunt_performed = self.perform_shunt_down(y)?;
                if !shunt_performed {
                    break;
                }
            }
            
            // need to handle moving something all the way down as far it can go
            // then combine anything that can be combined.
        }
        
        Ok(vec![])
    }

    fn perform_shunt_down(&mut self, column_index: u8) -> Result<bool, &'static str> {
        // Move everything as far down as it can go
        let mut lowest_empty_space: Option<GridCoord> = None;
        for x in 0..self.grid.get_size() {
            let current_position = GridCoord { x, y: column_index };
            let tile = self.grid.get_tile(current_position).unwrap();
            match tile {
                None => lowest_empty_space = Some(current_position),
                Some(_tile_to_move) => {
                    if let Some(new_position) = lowest_empty_space {
                        self.grid.move_tile(current_position, new_position)?;
                        return Ok(true);
                    }
                }
            }
        }

        Ok(false)
    }
}

pub enum MoveDirection {
    Up,
    Down,
    Left,
    Right,
}

#[cfg(test)]
mod test {
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

            assert_eq!(result.grid.get_size(), input);
        }

        #[rstest(input, case(0), case(1))]
        fn invalid_grid_size_requested_err_returned(input: u8) {
            let result = Game::new(input);

            assert_eq!(
                result.unwrap_err(),
                "cannot create a game with a grid smaller than 2x2."
            );
        }

        #[test]
        fn game_created_with_two_initial_tiles() {
            let game = Game::new(2).unwrap();

            let empty_positions = game.grid.get_empty_positions();

            assert_eq!(empty_positions.len(), 2);
        }

        #[test]
        fn game_created_with_two_tiles_that_are_of_initial_value() {
            let game = Game::new(2).unwrap();

            for x in 0..game.grid.get_size() {
                for y in 0..game.grid.get_size() {
                    if let Some(tile) = game.grid.get_tile(GridCoord { x, y }).unwrap() {
                        assert!(tile.value == 2 || tile.value == 4);
                    };
                }
            }
        }
    }
}
