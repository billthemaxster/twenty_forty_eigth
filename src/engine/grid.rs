use crate::engine::grid_coord::GridCoord;

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

    pub fn add_new_tile(
        &mut self,
        value: u16,
        position: GridCoord,
    ) -> Result<(), &'static str> {
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

    pub fn get_size(&self) -> u8{
        self.size
    }
}


#[cfg(test)]
mod test {
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
            let result = Grid::new(invalid_size);

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

        #[rstest(
            square0, 
            square1,
            case(GridCoord{x:0, y:0}, GridCoord{x:0, y:1}),
            case(GridCoord{x:0, y:0}, GridCoord{x:1, y:0}),
            case(GridCoord{x:0, y:0}, GridCoord{x:1, y:1}),
            case(GridCoord{x:0, y:1}, GridCoord{x:1, y:0}),
            case(GridCoord{x:0, y:1}, GridCoord{x:1, y:1}),
            case(GridCoord{x:1, y:0}, GridCoord{x:1, y:1})
        )]
        fn returns_correct_squares(square0: GridCoord, square1: GridCoord) {
            let mut grid = Grid::new(2).unwrap();

            let _ = grid.add_new_tile(2, square0).unwrap();
            let _ = grid.add_new_tile(2, square1).unwrap();

            let a = grid.get_empty_positions();

            assert_eq!(a.len(), 2);

            let square2 = a[0];
            assert!(!square2.is_equal(&square0));
            assert!(!square2.is_equal(&square1));

            let square3 = a[1];
            assert!(!square3.is_equal(&square0));
            assert!(!square3.is_equal(&square1));
        }
    }
}
