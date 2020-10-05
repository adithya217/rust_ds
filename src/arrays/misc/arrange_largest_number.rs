
use super::utils::find_largest_number;
use super::utils::count_digits;
use super::utils::pad_with_own_digits;
use std::collections::HashMap;

pub fn arrange(arr: &mut [u32]) {
    if arr.len() < 1 {
        return;
    }

    let (largest, _) = find_largest_number(arr).unwrap();
    let largest_num_digits_count = count_digits(*largest);

    let mut cache: HashMap<u32, u32> = HashMap::new();

    let cache_insert_lambda = |key: &u32| pad_with_own_digits(*key, largest_num_digits_count);

    arr.sort_by(|a, b| {
        let computed_a = get_or_insert_into_cache(&mut cache, a, cache_insert_lambda);
        let computed_b = get_or_insert_into_cache(&mut cache, b, cache_insert_lambda);
        return computed_a.cmp(&computed_b);
    });
}

fn get_or_insert_into_cache<F: FnOnce(&u32) -> u32>(cache: &mut HashMap<u32, u32>, key: &u32, compute_fn: F) -> u32 {
    match cache.get(key) {
        None => {
            let value = compute_fn(key);
            cache.insert(*key, value);
            value
        },
        Some(&value) => value
    }
}

#[cfg(tests)]
mod tests {
    use super::arrange as compute;

    #[test]
    fn test_with_empty_array() {
        let input = [];
        assert_eq!([], compute(&input));
    }

    #[test]
    fn test_with_input_having_multiple_same_digit_starts_with_conflicts() {
        let input = [3, 30, 34, 5, 9];
        let expected = [9, 5, 34, 3, 30];
        assert_eq!(expected, compute(&input));
    }

    #[test]
    fn test_with_input_having_multiple_same_digit_starts_without_conflicts() {
        let input = [54, 546, 548, 60];
        let expected = [60, 548, 546, 54];
        assert_eq!(expected, compute(&input));
    }

    #[test]
    fn test_with_input_having_multiple_same_digit_starts_with_conflicts_v2() {
        let input = [1, 34, 3, 98, 9, 76, 45, 4];
        let expected = [9, 98, 76, 45, 4, 34, 3, 1];
        assert_eq!(expected, compute(&input));
    }

    #[test]
    fn test_with_input_having_multiple_same_digit_starts_with_conflicts_v3() {
        let input = [12, 121];
        let expected = [12, 121];
        assert_eq!(expected, compute(&input));
    }
}