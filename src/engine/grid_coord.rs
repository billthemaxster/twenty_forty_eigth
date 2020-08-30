#[derive(Clone, Copy, Debug)]
pub struct GridCoord {
    pub x: u8,
    pub y: u8,
}

impl GridCoord {
    pub fn is_equal(&self, other: &GridCoord) -> bool {
        self.x == other.x && self.y == other.y
    }
}

#[cfg(test)]
mod test {
    use super::*;

    mod is_equal {
        use super::*;

        #[test]
        fn returns_true_when_both_x_and_y_are_equal() {
            let coord1 = GridCoord { x: 0, y: 0 };
            let coord2 = GridCoord { x: 0, y: 0 };

            let result = coord1.is_equal(&coord2);

            assert_eq!(result, true);
        }
        #[test]
        fn returns_false_when_both_x_and_y_are_not_equal() {
            let coord1 = GridCoord { x: 0, y: 0 };
            let coord2 = GridCoord { x: 1, y: 1 };

            let result = coord1.is_equal(&coord2);

            assert_eq!(result, false);
        }
        #[test]
        fn returns_false_when_xs_are_not_equal() {
            let coord1 = GridCoord { x: 0, y: 0 };
            let coord2 = GridCoord { x: 1, y: 0 };

            let result = coord1.is_equal(&coord2);

            assert_eq!(result, false);
        }
        #[test]
        fn returns_false_when_ys_are_not_equal() {
            let coord1 = GridCoord { x: 0, y: 0 };
            let coord2 = GridCoord { x: 0, y: 1 };

            let result = coord1.is_equal(&coord2);

            assert_eq!(result, false);
        }
    }
}
