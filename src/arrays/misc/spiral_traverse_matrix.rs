
use std::vec::Vec;

#[derive(Clone, Copy)]
pub enum TraversalDirection {
    RIGHT,
    DOWN,
    LEFT,
    UP
}

pub fn traverse(matrix: &[&[i32]]) -> Vec<i32> {
    if matrix.len() < 1 || matrix[0].len() < 1 {
        return vec![];
    }

    let mut path = Vec::with_capacity(matrix.len() * matrix[0].len());
    let (mut x_lower_bound, mut x_upper_bound) = (0, matrix[0].len() - 1);
    let (mut y_lower_bound, mut y_upper_bound) = (0, matrix.len() - 1);
    let (mut x_offset, mut y_offset) = (0, 0);
    let mut current_direction = TraversalDirection::RIGHT;

    while (x_lower_bound != x_upper_bound)
        && (y_lower_bound != y_upper_bound) {

            path.push(matrix[x_offset][y_offset]);

        match current_direction {
            TraversalDirection::RIGHT => {
                if x_offset < x_upper_bound {
                    x_offset += 1;
                } else {
                    current_direction = TraversalDirection::DOWN;
                    x_upper_bound -= 1;
                    y_lower_bound += 1;
                    y_offset += 1;
                }
            },
            TraversalDirection::DOWN => {
                if y_offset < y_upper_bound {
                    y_offset += 1;
                } else {
                    current_direction = TraversalDirection::LEFT;
                    y_upper_bound -= 1;
                    x_offset -= 1;
                }
            },
            TraversalDirection::LEFT => {
                if x_offset > x_lower_bound {
                    x_offset -= 1;
                } else {
                    current_direction = TraversalDirection::UP;
                    x_lower_bound += 1;
                    y_offset -= 1;
                }
            },
            TraversalDirection::UP => {
                if y_offset > y_lower_bound {
                    y_offset -= 1;
                } else {
                    current_direction = TraversalDirection::RIGHT;
                    x_offset += 1;
                }
            }
        }
    }

    return path;
}

#[cfg(tests)]
mod tests {
    use super::traverse as compute;

    #[test]
    fn test_with_empty_matrix() {
        let input = [];
        assert_eq!(vec![], compute(&input));
    }

    #[test]
    fn test_with_nested_empty_matrix() {
        let input = [[]];
        assert_eq!(vec![], compute(&input));
    }

    #[test]
    fn test_with_square_matrix() {
        let input = [
            [1, 2, 3, 4],
            [5, 6, 7, 8],
            [9, 10, 11, 12],
            [13, 14, 15, 16]
        ];
        let output = vec![1, 2, 3, 4, 8, 12, 16, 15, 14, 13, 9, 5, 6, 7, 11, 10];
        assert_eq!(vec![], compute(&input));
    }

    #[test]
    fn test_with_rectangle_matrix() {
        let input = [
            [1, 2, 3, 4],
            [5, 6, 7, 8],
            [9, 10, 11, 12]
        ];
        let output = vec![1, 2, 3, 4, 8, 12, 11, 10, 9, 5, 6, 7];
        assert_eq!(vec![], compute(&input));
    }
}