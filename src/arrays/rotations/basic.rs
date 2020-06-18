use super::utils::is_rotation_required;
use super::utils::compute_min_rotation_count;

fn rotate_basic<T>(arr: &mut [T], arr_size: usize, rotation_count: u32) {
    let arr_size = arr_size as u32;
    if arr.len() == 0 || !is_rotation_required(&arr_size, &rotation_count) {
        return;
    }

    let rotation_count = compute_min_rotation_count(&arr_size, &rotation_count);

    let arr_size = arr_size as usize;
    for _ in 0..rotation_count {
        for index in 0..arr_size-1 {
            arr.swap(index, index+1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::rotate_basic;

    #[test]
    fn rotate_for_numbers() {
        let mut data = [1,2,3,4,5,6,7,8];
        rotate_basic(&mut data, 8, 2);

        let expected = [3,4,5,6,7,8,1,2];
        assert!(data == expected)
    }

    #[test]
    fn rotate_partially() {
        let mut data = [1,2,3,4,5,6,7];
        rotate_basic(&mut data, 5, 2);

        let expected = [3,4,5,1,2,6,7];
        assert!(data == expected)
    }

    #[test]
    fn rotate_for_characters() {
        let mut data = ['a','b','c','d','e','f','g'];
        rotate_basic(&mut data, 7, 2);

        let expected = ['c','d','e','f','g','a','b'];
        assert!(data == expected)
    }

    #[test]
    fn rotate_no_change_on_same_rotations_as_arr_size() {
        let mut data = ['a','b','c','d'];
        rotate_basic(&mut data, 3, 3);

        let expected = ['a','b','c','d'];
        assert!(data == expected)
    }

    #[test]
    fn rotate_no_change_on_zero_arr_size() {
        let mut data = ['a','b','c','d'];
        rotate_basic(&mut data, 0, 2);

        let expected = ['a','b','c','d'];
        assert!(data == expected)
    }

    #[test]
    fn rotate_no_change_on_zero_rotations() {
        let mut data = ['a','b','c','d'];
        rotate_basic(&mut data, 4, 0);

        let expected = ['a','b','c','d'];
        assert!(data == expected)
    }

    #[test]
    fn rotate_no_errors_on_empty_array() {
        let mut data: [u8; 0] = [];
        rotate_basic(&mut data, 3, 2);

        let expected: [u8; 0] = [];
        assert!(data == expected)
    }

    #[test]
    fn rotate_correct_change_when_rotations_more_than_arr_size() {
        let mut data = [1,2,3,4,5,6,7];
        rotate_basic(&mut data, 7, 9);

        let expected = [3,4,5,6,7,1,2];
        assert!(data == expected)
    }
}
