
pub fn find_equilibrium_point(arr: &[i32]) -> Option<usize> {
    if arr.len() == 1 {
        return Some(0);
    }

    let total_sum = arr.iter().fold(0, |acc, number| acc + number);
    let mut suffix_sum = total_sum;
    let mut prefix_sum = arr[0];

    for index in 1..arr.len() {
        suffix_sum -= arr[index - 1];
        prefix_sum += arr[index];

        if suffix_sum == prefix_sum {
            return Some(index);
        }
    }

    return None;
}

#[cfg(test)]
mod tests {
    use super::find_equilibrium_point as compute;

    #[test]
    fn test_with_positive_set1_is_successful() {
        let arr = [1];
        assert_eq!(Some(0), compute(&arr));
    }

    #[test]
    fn test_with_positive_set2_is_successful() {
        let arr = [1, 3, 5, 2, 2];
        assert_eq!(Some(2), compute(&arr));
    }

    #[test]
    fn test_with_positive_set3_is_successful() {
        let arr = [-7, 1, 5, 2, -4, 3, 0];
        assert_eq!(Some(3), compute(&arr));
    }

    #[test]
    fn test_with_positive_set4_is_successful() {
        let arr = [1, 2, 3];
        assert_eq!(None, compute(&arr));
    }
}