
use std::vec::Vec;

#[derive(Clone, Copy, PartialEq)]
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

    if matrix.len() == 1 {
        return matrix[0].iter().fold(Vec::with_capacity(matrix[0].len()), |mut acc, number| {
            acc.push(*number);
            acc
        });
    }

    if matrix[0].len() == 1 {
        return matrix.iter().fold(Vec::with_capacity(matrix.len()), |mut acc, sub_matrix| {
            acc.push(sub_matrix[0]);
            acc
        });
    }

    let is_square_matrix = matrix.len() == matrix[0].len();
    let is_horizontal_matrix = matrix[0].len() > matrix.len(); // only if rectangle matrix

    let mut path = Vec::with_capacity(matrix.len() * matrix[0].len());
    let (mut x_lower_bound, mut x_upper_bound) = (0, matrix[0].len() - 1);
    let (mut y_lower_bound, mut y_upper_bound) = (0, matrix.len() - 1);
    let (mut x_offset, mut y_offset) = (0, 0);
    let mut current_direction = TraversalDirection::RIGHT;

    loop {
        path.push(matrix[y_offset][x_offset]);

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

        let is_x_bound_reached = x_lower_bound > x_upper_bound;
        let is_y_bound_reached = y_lower_bound > y_upper_bound;
        let is_one_bound_reached = is_x_bound_reached || is_y_bound_reached;
        let are_both_bounds_reached = is_x_bound_reached && is_y_bound_reached;
        if is_square_matrix {
            if are_both_bounds_reached && current_direction == TraversalDirection::UP {
                break;
            } else if are_both_bounds_reached && current_direction == TraversalDirection::DOWN {
                break;
            }
        } else if is_horizontal_matrix {
            if is_one_bound_reached && current_direction == TraversalDirection::DOWN {
                break;
            } else if is_one_bound_reached && current_direction == TraversalDirection::UP {
                break;
            }
        } else {
            if is_one_bound_reached && current_direction == TraversalDirection::LEFT {
                break;
            } else if is_one_bound_reached && current_direction == TraversalDirection::RIGHT {
                break;
            }
        }
    }

    return path;
}

#[cfg(test)]
mod tests {
    use super::traverse as compute;

    #[test]
    fn test_with_empty_matrix() {
        let input: &[&[i32]] = &[];
        let expected: Vec<i32> = vec![];
        assert_eq!(expected, compute(input));
    }

    #[test]
    fn test_with_nested_empty_matrix() {
        let input: &[&[i32]] = &[&[]];
        let expected: Vec<i32> = vec![];
        assert_eq!(expected, compute(&input));
    }

    #[test]
    fn test_with_single_element_matrix() {
        let input: &[&[i32]] = &[
            &[1]
        ];
        let expected = vec![1];
        assert_eq!(expected, compute(input));
    }

    #[test]
    fn test_with_square_matrix_ending_on_left() {
        let input: &[&[i32]] = &[
            &[1, 2],
            &[3, 4]
        ];
        let expected = vec![1, 2, 4, 3];
        assert_eq!(expected, compute(input));
    }

    #[test]
    fn test_with_square_matrix_ending_on_right() {
        let input: &[&[i32]] = &[
            &[1, 2, 3],
            &[4, 5, 6],
            &[7, 8, 9]
        ];
        let expected = vec![1, 2, 3, 6, 9, 8, 7, 4, 5];
        assert_eq!(expected, compute(input));
    }

    #[test]
    fn test_with_bigger_square_matrix_ending_on_left() {
        let input: &[&[i32]] = &[
            &[1, 2, 3, 4],
            &[5, 6, 7, 8],
            &[9, 10, 11, 12],
            &[13, 14, 15, 16]
        ];
        let expected = vec![1, 2, 3, 4, 8, 12, 16, 15, 14, 13, 9, 5, 6, 7, 11, 10];
        assert_eq!(expected, compute(input));
    }

    #[test]
    fn test_with_bigger_square_matrix_ending_on_right() {
        let input: &[&[i32]] = &[
            &[1, 2, 3, 4, 5],
            &[6, 7, 8, 9, 10],
            &[11, 12, 13, 14, 15],
            &[16, 17, 18, 19, 20],
            &[21, 22, 23, 24, 25]
        ];
        let expected = vec![1, 2, 3, 4, 5, 10, 15, 20, 25, 24, 23, 22, 21, 16, 11, 6, 7, 8, 9, 14, 19, 18, 17, 12, 13];
        assert_eq!(expected, compute(input));
    }

    #[test]
    fn test_with_horizontal_rectangle_matrix_ending_on_right() {
        let input: &[&[i32]] = &[
            &[1, 2, 3]
        ];
        let expected = vec![1, 2, 3];
        assert_eq!(expected, compute(input));
    }

    #[test]
    fn test_with_horizontal_rectangle_matrix_ending_on_left() {
        let input: &[&[i32]] = &[
            &[1, 2, 3],
            &[4, 5, 6]
        ];
        let expected = vec![1, 2, 3, 6, 5, 4];
        assert_eq!(expected, compute(input));
    }

    #[test]
    fn test_with_bigger_horizontal_rectangle_matrix_ending_on_right() {
        let input: &[&[i32]] = &[
            &[1, 2, 3, 4],
            &[5, 6, 7, 8],
            &[9, 10, 11, 12]
        ];
        let expected = vec![1, 2, 3, 4, 8, 12, 11, 10, 9, 5, 6, 7];
        assert_eq!(expected, compute(input));
    }

    #[test]
    fn test_with_bigger_horizontal_rectangle_matrix_ending_on_left() {
        let input: &[&[i32]] = &[
            &[1, 2, 3, 4, 5],
            &[6, 7, 8, 9, 10],
            &[11, 12, 13, 14, 15],
            &[16, 17, 18, 19, 20]
        ];
        let expected = vec![1, 2, 3, 4, 5, 10, 15, 20, 19, 18, 17, 16, 11, 6, 7, 8, 9, 14, 13, 12];
        assert_eq!(expected, compute(input));
    }

    #[test]
    fn test_with_vertical_rectangle_matrix_ending_on_down() {
        let input: &[&[i32]] = &[
            &[1],
            &[2],
            &[3]
        ];
        let expected = vec![1, 2, 3];
        assert_eq!(expected, compute(input));
    }

    #[test]
    fn test_with_vertical_rectangle_matrix_ending_on_up() {
        let input: &[&[i32]] = &[
            &[1, 2],
            &[3, 4],
            &[5, 6]
        ];
        let expected = vec![1, 2, 4, 6, 5, 3];
        assert_eq!(expected, compute(input));
    }

    #[test]
    fn test_with_bigger_vertical_rectangle_matrix_ending_on_down() {
        let input: &[&[i32]] = &[
            &[1, 2, 3],
            &[4, 5, 6],
            &[7, 8, 9],
            &[10, 11, 12]
        ];
        let expected = vec![1, 2, 3, 6, 9, 12, 11, 10, 7, 4, 5, 8];
        assert_eq!(expected, compute(input));
    }

    #[test]
    fn test_with_bigger_vertical_rectangle_matrix_ending_on_up() {
        let input: &[&[i32]] = &[
            &[1, 2, 3, 4],
            &[5, 6, 7, 8],
            &[9, 10, 11, 12],
            &[13, 14, 15, 16],
            &[17, 18, 19, 20]
        ];
        let expected = vec![1, 2, 3, 4, 8, 12, 16, 20, 19, 18, 17, 13, 9, 5, 6, 7, 11, 15, 14, 10];
        assert_eq!(expected, compute(input));
    }
}