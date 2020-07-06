use super::super::utils::is_rotation_required;
use super::super::utils::compute_min_rotation_count;
use super::super::utils::swap_blocks;

pub fn rotate_using_block_swap<T>(arr: &mut [T], arr_size: usize, rotation_count: u32) {
    let arr_size = arr_size as u32;
    if arr.len() == 0 || !is_rotation_required(&arr_size, &rotation_count) {
        return;
    }

    let rotation_count = compute_min_rotation_count(&arr_size, &rotation_count) as usize;
    let arr_size = arr_size as usize;

    let mut rem_arr_start_index = 0 as usize;
    let mut rem_arr_size = arr_size;
    let mut rem_rotations = rotation_count;

    while rem_arr_size - rem_rotations != rem_rotations {
        let diff = rem_arr_size - rem_rotations;
        if rem_rotations < diff {
            let a_offset = rem_arr_start_index;
            let b_offset = a_offset + diff;
            swap_blocks(arr, a_offset, b_offset, rem_rotations);

            rem_arr_start_index += rem_rotations;
            rem_arr_size = diff;
        } else {
            let a_offset = rem_arr_start_index;
            let b_offset = a_offset + rem_rotations;
            swap_blocks(arr, a_offset, b_offset, diff);

            let temp = rem_arr_size;
            rem_arr_size = rem_rotations;
            rem_rotations = (2 * rem_rotations) - temp;
        }
    }

    swap_blocks(arr, rem_arr_start_index, rem_arr_start_index + rem_rotations, rem_rotations);
}

#[cfg(test)]
mod tests {
    use super::rotate_using_block_swap as rotate;

    #[test]
    fn rotate_using_block_swap_partially() {
        let mut data = [1,2,3,4,5,6,7];
        rotate(&mut data, 5, 2);

        let expected = [4,5,1,2,3,6,7];
        assert_eq!(data, expected);
    }

    #[test]
    fn rotate_using_block_swap_basic_for_characters() {
        let mut data = ['a','b','c','d','e','f','g'];
        rotate(&mut data, 7, 2);

        let expected = ['f','g','a','b','c','d','e'];
        assert_eq!(data, expected);
    }

    #[test]
    fn rotate_using_block_swap_no_change_on_same_rotations_as_arr_size() {
        let mut data = ['a','b','c','d'];
        rotate(&mut data, 3, 3);

        let expected = ['a','b','c','d'];
        assert_eq!(data, expected);
    }

    #[test]
    fn rotate_using_block_swap_no_change_on_zero_arr_size() {
        let mut data = ['a','b','c','d'];
        rotate(&mut data, 0, 2);

        let expected = ['a','b','c','d'];
        assert_eq!(data, expected);
    }

    #[test]
    fn rotate_using_block_swap_no_change_on_zero_rotations() {
        let mut data = ['a','b','c','d'];
        rotate(&mut data, 4, 0);

        let expected = ['a','b','c','d'];
        assert_eq!(data, expected);
    }

    #[test]
    fn rotate_using_block_swap_no_errors_on_empty_array() {
        let mut data: [u8; 0] = [];
        rotate(&mut data, 3, 2);

        let expected: [u8; 0] = [];
        assert_eq!(data, expected);
    }

    #[test]
    fn rotate_using_block_swap_correct_change_when_rotations_more_than_arr_size() {
        let mut data = [1,2,3,4,5,6,7];
        rotate(&mut data, 7, 9);

        let expected = [6,7,1,2,3,4,5];
        assert_eq!(data, expected);
    }

    #[test]
    fn rotate_using_block_swap_once() {
        let mut data = [1,2,3,4,5,6,7];
        rotate(&mut data, 5, 1);

        let expected = [5,1,2,3,4,6,7];
        assert_eq!(data, expected);
    }

    #[test]
    fn rotate_using_block_swap_thrice() {
        let mut data = [1,2,3,4,5,6,7];
        rotate(&mut data, 5, 3);

        let expected = [3,4,5,1,2,6,7];
        assert_eq!(data, expected);
    }

    #[test]
    fn rotate_using_block_swap_when_arr_size_is_even_and_rotation_count_is_even() {
        let mut data = [1,2,3,4,5,6,7,8];
        rotate(&mut data, 8, 2);

        let expected = [7,8,1,2,3,4,5,6];
        assert_eq!(data, expected);
    }

    #[test]
    fn rotate_using_block_swap_when_arr_size_is_even_and_rotation_count_is_odd() {
        let mut data = [1,2,3,4,5,6,7,8];
        rotate(&mut data, 8, 3);

        let expected = [6,7,8,1,2,3,4,5];
        assert_eq!(data, expected);
    }

    #[test]
    fn rotate_using_block_swap_when_arr_size_is_odd_and_rotation_count_is_even() {
        let mut data = [1,2,3,4,5,6,7];
        rotate(&mut data, 7, 2);

        let expected = [6,7,1,2,3,4,5];
        assert_eq!(data, expected);
    }

    #[test]
    fn rotate_using_block_swap_when_arr_size_is_odd_and_rotation_count_is_odd() {
        let mut data = [1,2,3,4,5,6,7];
        rotate(&mut data, 7, 3);

        let expected = [5,6,7,1,2,3,4];
        assert_eq!(data, expected);
    }
}