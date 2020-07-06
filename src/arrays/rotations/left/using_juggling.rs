use super::super::utils::is_rotation_required;
use super::super::utils::compute_min_rotation_count;
use crate::math::gcd::compute_euclidean_gcd;

pub fn rotate_using_juggling<T>(arr: &mut [T], arr_size: usize, rotation_count: u32) {
    let arr_size = arr_size as u32;
    if arr.len() == 0 || !is_rotation_required(&arr_size, &rotation_count) {
        return;
    }

    let rotation_count = compute_min_rotation_count(&arr_size, &rotation_count);
    let set_size = compute_euclidean_gcd(&arr_size, &rotation_count);
    let sets_count = arr_size / set_size;
    let juggle_count = if set_size == 1 && set_size < rotation_count { rotation_count } else { 1 };

    for _ in 0..juggle_count {
        for index in 0..set_size {
            for set_index in 0..sets_count-1 {
                let index_a = ((set_index * set_size) + index) as usize;
                let index_b = (((set_index + 1) * set_size) + index) as usize;
                arr.swap(index_a, index_b);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::rotate_using_juggling as rotate;

    #[test]
    fn rotate_using_juggling_for_numbers() {
        let mut data = [1,2,3,4,5,6,7,8];
        rotate(&mut data, 8, 2);

        let expected = [3,4,5,6,7,8,1,2];
        assert_eq!(data, expected);
    }

    #[test]
    fn rotate_using_juggling_partially() {
        let mut data = [1,2,3,4,5,6,7];
        rotate(&mut data, 5, 2);

        let expected = [3,4,5,1,2,6,7];
        assert_eq!(data, expected);
    }

    #[test]
    fn rotate_using_juggling_for_characters() {
        let mut data = ['a','b','c','d','e','f','g'];
        rotate(&mut data, 7, 2);

        let expected = ['c','d','e','f','g','a','b'];
        assert_eq!(data, expected);
    }

    #[test]
    fn rotate_using_juggling_no_change_on_same_rotations_as_arr_size() {
        let mut data = ['a','b','c','d'];
        rotate(&mut data, 3, 3);

        let expected = ['a','b','c','d'];
        assert_eq!(data, expected);
    }

    #[test]
    fn rotate_using_juggling_no_change_on_zero_arr_size() {
        let mut data = ['a','b','c','d'];
        rotate(&mut data, 0, 2);

        let expected = ['a','b','c','d'];
        assert_eq!(data, expected);
    }

    #[test]
    fn rotate_using_juggling_no_change_on_zero_rotations() {
        let mut data = ['a','b','c','d'];
        rotate(&mut data, 4, 0);

        let expected = ['a','b','c','d'];
        assert_eq!(data, expected);
    }

    #[test]
    fn rotate_using_juggling_no_errors_on_empty_array() {
        let mut data: [u8; 0] = [];
        rotate(&mut data, 3, 2);

        let expected: [u8; 0] = [];
        assert_eq!(data, expected);
    }

    #[test]
    fn rotate_using_juggling_correct_change_when_rotations_more_than_arr_size() {
        let mut data = [1,2,3,4,5,6,7];
        rotate(&mut data, 7, 9);

        let expected = [3,4,5,6,7,1,2];
        assert_eq!(data, expected);
    }

    #[test]
    fn rotate_using_juggling_once() {
        let mut data = [1,2,3,4,5,6,7];
        rotate(&mut data, 5, 1);

        let expected = [2,3,4,5,1,6,7];
        assert_eq!(data, expected);
    }

    #[test]
    fn rotate_using_juggling_thrice() {
        let mut data = [1,2,3,4,5,6,7];
        rotate(&mut data, 5, 3);

        let expected = [4,5,1,2,3,6,7];
        assert_eq!(data, expected);
    }
}