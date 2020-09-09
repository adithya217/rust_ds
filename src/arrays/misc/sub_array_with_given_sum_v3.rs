use super::utils::find_smallest_negative_number;

// This algorithm doesn't work, refer the test cases.
pub fn find_from_mixed_numbers_v2(arr: &[i32], target: i32) -> (i32, i32) {
    if arr[0] == target { return (0, 0); }

    let offset = match find_smallest_negative_number(arr) {
        None => 0,
        Some((&number, _)) => number.abs()
    };

    let mut start_index: i32 = 0;
    let mut sum: i32 = arr[0] + offset;
    let mut modified_target: i32 = target + offset;

    for index in 1..arr.len() {
        sum += arr[index] + offset;
        modified_target += offset;

        // Problem 1: If we compare sum > target, it will always be true when target is negative
        while (sum > modified_target) && ((start_index as usize) < index) {
            sum -= arr[start_index as usize] + offset;
            start_index += 1;
            modified_target -= offset;
        }

        if sum == modified_target {
            return (start_index, index as i32);
        }
    }

    return (-1, -1);
}

#[cfg(test)]
mod tests {
    use super::find_from_mixed_numbers_v2 as compute;

    #[test]
    fn test_with_positive_set1_will_find() {
        let test_data = [1, 2, 3, 7, 5];
        let (start_index, end_index) = compute(&test_data, 12);

        assert_eq!(1, start_index);
        assert_eq!(3, end_index);
    }

    #[test]
    fn test_with_positive_set2_will_find() {
        let test_data = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let (start_index, end_index) = compute(&test_data, 15);

        assert_eq!(0, start_index);
        assert_eq!(4, end_index);
    }

    #[test]
    fn test_with_positive_set3_will_find() {
        let test_data = [1, 4, 20, 3, 10, 5];
        let (start_index, end_index) = compute(&test_data, 33);

        assert_eq!(2, start_index);
        assert_eq!(4, end_index);
    }

    #[test]
    fn test_with_positive_set4_will_find() {
        let test_data = [1, 4, 0, 0, 3, 10, 5];
        let (start_index, end_index) = compute(&test_data, 7);

        assert_eq!(1, start_index);
        assert_eq!(4, end_index);
    }

    // This test will fail because the algorithm is not correct.
    #[test]
    fn test_with_positive_set5_will_find() {
        let test_data = [10, 2, -2, -20, 10];
        let (start_index, end_index) = compute(&test_data, -10);

        assert_eq!(0, start_index);
        assert_eq!(3, end_index);
    }

    #[test]
    fn test_with_positive_set6_will_find() {
        let test_data = [1, 4];
        let (start_index, end_index) = compute(&test_data, 1);

        assert_eq!(0, start_index);
        assert_eq!(0, end_index);
    }

    #[test]
    fn test_with_positive_set7_will_find() {
        let test_data = [1, 4];
        let (start_index, end_index) = compute(&test_data, 4);

        assert_eq!(1, start_index);
        assert_eq!(1, end_index);
    }

    #[test]
    fn test_with_negative_set1_will_find() {
        let test_data = [1, 4];
        let (start_index, end_index) = compute(&test_data, 0);

        assert_eq!(-1, start_index);
        assert_eq!(-1, end_index);
    }

    #[test]
    fn test_with_negative_set2_will_find() {
        let test_data = [-10, 0, 2, -2, -20, 10];
        let (start_index, end_index) = compute(&test_data, 20);

        assert_eq!(-1, start_index);
        assert_eq!(-1, end_index);
    }
}