
pub fn find_last_index(arr: &[i32], target: i32) -> Option<usize> {
    if arr.len() < 1 {
        return None;
    }

    for index in (0..arr.len()).rev() {
        if arr[index] == target {
            return Some(index);
        }
    }

    return None;
}

#[cfg(test)]
mod tests {
    use super::find_last_index as compute;

    #[test]
    fn test_with_no_elements() {
        let input = [];
        assert_eq!(None, compute(&input, 0));
    }

    #[test]
    fn test_with_target_present() {
        let input = [1, 2, 3, 4, 5];
        assert_eq!(Some(2), compute(&input, 3));
    }

    #[test]
    fn test_with_target_absent() {
        let input = [1, 2, 3, 4, 5];
        assert_eq!(None, compute(&input, -1));
    }

    #[test]
    fn test_with_target_duplicates() {
        let input = [1, 2, 3, 3, 4, 5];
        assert_eq!(Some(3), compute(&input, 3));
    }
}