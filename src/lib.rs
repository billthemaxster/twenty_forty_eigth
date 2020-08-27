pub enum MoveDirection {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone)]
pub struct Tile {
    pub value: u16,
}

pub struct Game {
    pub score: u32,
    pub tiles: Vec<Vec<Tile>>,
}

impl Game {
    pub fn new(size: u8) -> Game {
        let size: usize = size.into();
        let score = 0;
        let tiles: Vec<Vec<Tile>> = vec![vec![Tile { value: 2 }; size]; size];

        Game { score, tiles }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(test)]
    mod game {
        use super::*;

        mod new {
            use super::*;
            use rstest::rstest;

            #[test]
            fn set_score_to_zero() {
                let result = Game::new(2);

                assert_eq!(result.score, 0)
            }

            #[rstest(input, case(2), case(4), case(5))]
            fn set_tiles_to_grid_of_given_size(input: u8) {
                let result = Game::new(input);

                let expected_length: usize = input.into();
                assert_eq!(result.tiles.len(), expected_length);

                for row in result.tiles.iter() {
                    assert_eq!(row.len(), expected_length);
                }
            }
        }
    }
}
