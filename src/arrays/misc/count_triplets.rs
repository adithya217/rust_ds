use std::collections::HashMap;
use std::cmp::max;
use crate::math::combinations::n_c_r;

pub fn count_triplets(arr: &[i32]) -> u128 {
    let mut counts: HashMap<&i32, u128> = HashMap::new();

    let max_value = arr.iter().fold(arr[0], |acc, number| {
        let new_count = match counts.get(number) {
            Some(count) => count + 1,
            None => 1
        };

        counts.insert(number, new_count);

        return max(acc, *number);
    });

    let mut triplet_count: u128 = 0;

    let count_of_zeroes = match counts.get(&0) {
        Some(&count) => count,
        None => 0
    };

    // Case 1: (0, 0, 0)
    triplet_count += if count_of_zeroes == 0 { 0 } else { n_c_r(count_of_zeroes, 3)};

    (1..=max_value).for_each(|x| {
        let count_x = match counts.get(&x) {
            Some(&count) => count,
            None => 0
        };

        let pair_count = if count_x == 0 { 0 } else { n_c_r(count_x as u128, 2) };

        // Case 2: (0, x, x) where x > 0
        triplet_count += count_of_zeroes * pair_count;

        // Case 3: (x, x, 2*x)
        let count_2x = match counts.get(&(2 * x)) {
            Some(&count) => count,
            None => 0
        };
        triplet_count += pair_count * count_2x;

        // Case 4: (x, y, x+y) where x < y and x > 0 and y > 0
        (x+1..=max_value).for_each(|y| {
            let count_y = match counts.get(&y) {
                Some(&count) => count,
                None => 0
            };

            let count_x_y = match counts.get(&(x + y)) {
                Some(&count) => count,
                None => 0
            };

            triplet_count += count_x * count_y * count_x_y;
        });
    });

    return triplet_count;
}

#[cfg(test)]
mod tests {
    use super::count_triplets as compute;

    #[test]
    fn test_positive_set1_has_results() {
        let test_data = [1, 2, 3, 4, 5];
        assert_eq!(4, compute(&test_data));
    }

    #[test]
    fn test_positive_set2_has_results() {
        let test_data = [1, 1, 1, 2, 2];
        assert_eq!(6, compute(&test_data));
    }

    #[test]
    fn test_positive_set3_has_results() {
        let test_data = [1, 5, 3, 2];
        assert_eq!(2, compute(&test_data));
    }

    #[test]
    fn test_negative_set1_has_no_results() {
        let test_data = [3, 2, 7];
        assert_eq!(0, compute(&test_data));
    }
}