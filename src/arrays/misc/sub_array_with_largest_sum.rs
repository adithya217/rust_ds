
pub fn sub_array_with_largest_sum(arr: &[i32]) -> i32 {
    let mut max_sum = arr[0];
    let mut current_sum = arr[0];

    for index in 1..arr.len() {
        current_sum += arr[index];

        if max_sum < current_sum {
            max_sum = current_sum;
        }

        if current_sum < 0 {
            current_sum = 0;
        }
    }

    return max_sum;
}

#[cfg(test)]
mod tests {
    use super::sub_array_with_largest_sum as compute;

    #[test]
    fn test_with_positive_set1_is_successful() {
        let test_data = [-2, -3, 4, -1, -2, 1, 5, -3];
        assert_eq!(7, compute(&test_data));
    }

    #[test]
    fn test_with_positive_set2_is_successful() {
        let test_data = [1, 2, 3, -2, 5];
        assert_eq!(9, compute(&test_data));
    }

    #[test]
    fn test_with_positive_set3_is_successful() {
        let test_data = [-1, -2, -3, -4];
        assert_eq!(-1, compute(&test_data));
    }

    #[test]
    fn test_with_positive_set4_is_successful() {
        let test_data = [3];
        assert_eq!(3, compute(&test_data));
    }
}