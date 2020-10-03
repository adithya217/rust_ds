
use std::collections::VecDeque;
use std::cmp::min;
use std::cmp::max;

pub fn find_index(arr: &[i32]) -> Option<usize> {
    if arr.len() < 3 {
        return None;
    }

    let mut suffix_mins = VecDeque::with_capacity(arr.len());
    suffix_mins.push_front(i32::MAX);

    let curr_min = i32::MAX;
    for index in (1..arr.len()).rev() {
        let new_min = min(curr_min, arr[index]);
        suffix_mins.push_front(new_min);
    }

    let suffix_mins = suffix_mins;

    let mut curr_max = arr[0];
    for index in 1..arr.len() - 1 {
        let current_element = arr[index];
        let min_element_after = suffix_mins[index];

        if curr_max <= current_element && current_element <= min_element_after {
            return Some(index);
        }

        curr_max = max(curr_max, current_element);
    }

    return None;
}

#[cfg(test)]
mod tests {
    use super::find_index as compute;

    #[test]
    fn test_with_simple_middle_element_positive_case() {
        let input = [4, 2, 5, 7];
        assert_eq!(Some(2), compute(&input));
    }

    #[test]
    fn test_with_simple_middle_element_negative_case() {
        let input = [11, 9, 12];
        assert_eq!(None, compute(&input));
    }

    #[test]
    fn test_with_multiple_middle_elements_positive_case() {
        let input = [4, 3, 2, 7, 8, 9];
        assert_eq!(Some(3), compute(&input));
    }

    #[test]
    fn test_with_multiple_middle_elements_positive_case_v2() {
        let input = [5, 1, 4, 3, 6, 8, 10, 7, 9];
        assert_eq!(Some(4), compute(&input));
    }

    #[test]
    fn test_with_repeated_middle_elements_negative_case() {
        let input = [5, 1, 4, 4];
        assert_eq!(None, compute(&input));
    }

    #[test]
    fn test_with_repeated_middle_elements_positive_case() {
        let input = [5, 1, 4, 6, 6, 7];
        assert_eq!(Some(3), compute(&input));
    }

    #[test]
    fn test_with_negative_and_positive_elements_mixture_positive_case() {
        let input = [-5, -1, 4, 6, 6, 7];
        assert_eq!(Some(1), compute(&input));
    }
}