use std::collections::HashMap;

pub fn find_from_mixed_numbers_v1(arr: &[i32], target: i32) -> (i32, i32) {
    if arr.len() == 1 {
        if arr[0] == target { return (0, 0); } else { return(-1, -1); }
    }

    let mut data_map: HashMap<i32, i32> = HashMap::new();
    let mut sum: i32 = 0;

    for (index, number) in arr.iter().enumerate() {
        let index = index as i32;
        sum += number;

        if sum == target {
            return (0, index);
        }

        let sum_diff = sum - target;
        match data_map.get(&sum_diff) {
            Some(&prev_index) => return (prev_index + 1, index),
            None => {}
        }

        data_map.insert(sum, index);
    }

    return (-1, -1);
}

#[cfg(test)]
mod tests {
    use super::find_from_mixed_numbers_v1 as compute;

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