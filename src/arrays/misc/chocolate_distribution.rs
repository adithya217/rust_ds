
use std::cmp::min;

pub fn find_min_difference(arr: &mut [u8], group_size: usize) -> Option<u8> {
    if group_size < 1 {
        return None;
    }

    if arr.len() < group_size {
        return None;
    }

    arr.sort();

    let mut min_difference = u8::MAX;
    for start_index in 0..=arr.len()-group_size {
        let end_index = start_index + group_size - 1;
        min_difference = min(min_difference, arr[end_index] - arr[start_index]);
    }

    return Some(min_difference);
}

#[cfg(test)]
mod tests {
    use super::find_min_difference as compute;

    #[test]
    fn test_with_group_size_bigger_than_input() {
        let mut input = [1, 2, 3];
        let group_size = input.len() + 2;

        assert_eq!(None, compute(&mut input, group_size));
    }

    #[test]
    fn test_with_group_size_zero() {
        let mut input = [1, 2, 3];
        let group_size = 0;

        assert_eq!(None, compute(&mut input, group_size));
    }

    #[test]
    fn test_with_empty_input() {
        let mut input = [];
        let group_size = 5;

        assert_eq!(None, compute(&mut input, group_size));
    }

    #[test]
    fn test_with_random_input_v1() {
        let mut input = [3, 4, 1, 9, 56, 7, 9, 12];
        let group_size = 5;

        assert_eq!(Some(6), compute(&mut input, group_size));
    }

    #[test]
    fn test_with_random_input_v2() {
        let mut input = [7, 3, 2, 4, 9, 12, 56];
        let group_size = 3;

        assert_eq!(Some(2), compute(&mut input, group_size));
    }

    #[test]
    fn test_with_random_input_v3() {
        let mut input = [12, 4, 7, 9, 2, 23, 25, 41, 30, 40, 28, 42, 30, 44, 48, 43, 50];
        let group_size = 7;

        assert_eq!(Some(10), compute(&mut input, group_size));
    }
}