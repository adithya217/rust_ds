
pub fn find_missing_number(arr: &[u32]) -> u32 {
    let expected_size: u32 = (arr.len() + 1) as u32;
    let expected_sum: u32 = expected_size * (expected_size + 1) / 2;
    let actual_sum: u32 = arr.iter().fold(0, |acc, number| acc + number);
    return expected_sum - actual_sum;
}

#[cfg(test)]
mod tests {
    use super::find_missing_number as compute;

    #[test]
    fn test_with_positive_set1_is_successful() {
        let test_data = [1, 2, 3, 5];
        assert_eq!(4, compute(&test_data));
    }

    #[test]
    fn test_with_positive_set2_is_successful() {
        let test_data = [1, 2, 3, 4, 5, 6, 7, 8, 10];
        assert_eq!(9, compute(&test_data));
    }

    #[test]
    fn test_with_positive_set3_is_successful() {
        let test_data = [1, 2, 4, 6, 3, 7, 8];
        assert_eq!(5, compute(&test_data));
    }
}