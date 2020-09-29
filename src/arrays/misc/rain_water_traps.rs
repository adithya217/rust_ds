
pub fn calculate_amount(arr: &[u8]) -> u8 {
    if arr.len() < 3 {
        return 0;
    }

    let mut prev_high = arr[0];
    let mut prev_high_index = 0;

    let mut water_in_section = 0;
    let mut total_water = 0;

    for index in 1..arr.len() {
        let current = arr[index];
        if current >= prev_high {
            prev_high = current;
            prev_high_index = index;
            water_in_section = 0;
        } else {
            let water_here = prev_high - current;
            water_in_section += water_here;
            total_water += water_here;
        }
    }

    if prev_high_index == arr.len() - 1 {
        return total_water;
    }

    total_water -= water_in_section;
    prev_high = arr[arr.len() - 1];

    for index in (prev_high_index..arr.len()).rev() {
        let current = arr[index];
        if current >= prev_high {
            prev_high = current;
        } else {
            total_water += prev_high - current;
        }
    }

    return total_water;
}

#[cfg(test)]
mod tests {
    use super::calculate_amount as compute;

    #[test]
    fn test_with_boundaries_at_both_ends_where_left_smaller_than_right_boundary() {
        let input = [3, 0, 0, 2, 0, 4];
        assert_eq!(10, compute(&input));
    }

    #[test]
    fn test_with_boundaries_at_both_ends_where_right_smaller_than_left_boundary() {
        let input = [9, 0, 4, 7];
        assert_eq!(10, compute(&input));
    }

    #[test]
    fn test_with_no_empty_gaps_where_left_smaller_than_right_boundary() {
        let input = [6, 9, 9];
        assert_eq!(0, compute(&input));
    }

    #[test]
    fn test_with_equal_sized_boundaries() {
        let input = [2, 0, 2];
        assert_eq!(2, compute(&input));
    }

    #[test]
    fn test_with_random_combination() {
        let input = [0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1];
        assert_eq!(6, compute(&input));
    }

    #[test]
    fn test_with_no_elements() {
        let input = [];
        assert_eq!(0, compute(&input));
    }

    #[test]
    fn test_with_one_element() {
        let input = [1];
        assert_eq!(0, compute(&input));
    }

    #[test]
    fn test_with_two_element() {
        let input = [1, 2];
        assert_eq!(0, compute(&input));
    }
}