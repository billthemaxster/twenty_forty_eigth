use crate::engine::grid::Grid;
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
    }
}
