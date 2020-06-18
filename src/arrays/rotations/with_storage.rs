use super::utils::is_rotation_required;
use super::utils::compute_min_rotation_count;
use std::ptr;

fn rotate_using_storage<T: Clone>(arr: &mut [T], arr_size: usize, rotation_count: u32) {
    let arr_size = arr_size as u32;
    if arr.len() == 0 || !is_rotation_required(&arr_size, &rotation_count) {
        return;
    }

    let rotation_count = compute_min_rotation_count(&arr_size, &rotation_count) as usize;
    let last_items = &mut arr[0..rotation_count].to_vec();

    let start = rotation_count as usize;
    let end = arr_size as usize;
    for index in start..end {
        let ai = index - start;
        let bi = index;
        arr.swap(ai, bi);
    }

    let start = arr_size as usize - last_items.len();
    let end = arr_size as usize;
    for index in (start..end).rev() {
        unsafe {
            let a: *mut T = &mut arr[index];
            let b: *mut T = last_items.get_mut(index - start).expect("Element not found at index!");
            ptr::swap(a, b);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::rotate_using_storage;

    #[test]
    fn rotate_using_storage_for_numbers() {
        let mut data = [1,2,3,4,5,6,7,8];
        rotate_using_storage(&mut data, 8, 2);

        let expected = [3,4,5,6,7,8,1,2];
        assert!(data == expected)
    }

    #[test]
    fn rotate_partially() {
        let mut data = [1,2,3,4,5,6,7];
        rotate_using_storage(&mut data, 5, 2);

        let expected = [3,4,5,1,2,6,7];
        assert!(data == expected)
    }

    #[test]
    fn rotate_for_characters() {
        let mut data = ['a','b','c','d','e','f','g'];
        rotate_using_storage(&mut data, 7, 2);

        let expected = ['c','d','e','f','g','a','b'];
        assert!(data == expected)
    }

    #[test]
    fn rotate_no_change_on_same_rotations_as_arr_size() {
        let mut data = ['a','b','c','d'];
        rotate_using_storage(&mut data, 3, 3);

        let expected = ['a','b','c','d'];
        assert!(data == expected)
    }

    #[test]
    fn rotate_no_change_on_zero_arr_size() {
        let mut data = ['a','b','c','d'];
        rotate_using_storage(&mut data, 0, 2);

        let expected = ['a','b','c','d'];
        assert!(data == expected)
    }

    #[test]
    fn rotate_no_change_on_zero_rotations() {
        let mut data = ['a','b','c','d'];
        rotate_using_storage(&mut data, 4, 0);

        let expected = ['a','b','c','d'];
        assert!(data == expected)
    }

    #[test]
    fn rotate_no_errors_on_empty_array() {
        let mut data: [u8; 0] = [];
        rotate_using_storage(&mut data, 3, 2);

        let expected: [u8; 0] = [];
        assert!(data == expected)
    }

    #[test]
    fn rotate_correct_change_when_rotations_more_than_arr_size() {
        let mut data = [1,2,3,4,5,6,7];
        rotate_using_storage(&mut data, 7, 9);

        let expected = [3,4,5,6,7,1,2];
        assert!(data == expected)
    }
}