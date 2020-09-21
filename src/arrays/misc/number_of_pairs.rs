
use std::collections::HashMap;
use std::vec::Vec;
use std::cmp::min;
use std::cmp::max;
use std::mem::replace;

/**
 * Find the number of pairs (x, y) such that x^y > y^x.
 * Constraints: arr[i] <= 1000
 */
pub fn find_number_of_pairs(arr_x: &[u32], arr_y: &[u32]) -> u32 {

    let (frequency_map, _, biggest): (HashMap<&u32, u32>, &u32, &u32)
        = compute_frequency_map_smallest_biggest(arr_y);

    let suffix_sum_vec: Vec<u32> = compute_suffix_sum_vector(&frequency_map, biggest);

    return arr_x.iter().fold(0, |acc, number| {
        match number {
            0 => acc,
            1 => acc
                + get_element_frequency_from_map(&frequency_map, &0),
            2 => acc
                + get_element_frequency_from_map(&frequency_map, &0)
                + get_element_frequency_from_map(&frequency_map, &1)
                + get_element_frequency_from_vec(&suffix_sum_vec, 5),
            3 => acc
                + get_element_frequency_from_vec(&suffix_sum_vec, 0)
                - get_element_frequency_from_map(&frequency_map, &3),
            x => acc
                + get_element_frequency_from_map(&frequency_map, &0)
                + get_element_frequency_from_map(&frequency_map, &1)
                + get_element_frequency_from_vec(&suffix_sum_vec, (x + 1) as usize)
        }
    });
}

fn compute_frequency_map_smallest_biggest(arr: &[u32]) -> (HashMap<&u32, u32>, &u32, &u32) {
    return arr.iter().fold((HashMap::new(), &arr[0], &arr[0]), |(mut acc, x, y), number| {
        let new_frequency = get_element_frequency_from_map(&acc, &number) + 1;
        acc.insert(number, new_frequency);
        (acc, min(x, number), max(y, number))
    });
}

fn compute_suffix_sum_vector(frequency_map: &HashMap<&u32, u32>, end: &u32) -> Vec<u32> {
    return (0..=*end).rev().fold(vec![0; (end + 1) as usize], |mut acc, number| {
        let prev_suffix_sum = get_element_frequency_from_vec(&acc, (number + 1) as usize);
        let curr_element_count = get_element_frequency_from_map(&frequency_map, &number);
        replace(&mut acc[number as usize], prev_suffix_sum + curr_element_count);
        acc
    });
}

fn get_element_frequency_from_map(frequency_map: &HashMap<&u32, u32>, element: &u32) -> u32 {
    return extract_frequency(frequency_map.get(element));
}

fn get_element_frequency_from_vec(frequency_vec: &Vec<u32>, element: usize) -> u32 {
    return extract_frequency(frequency_vec.get(element));
}

fn extract_frequency(optional: Option<&u32>) -> u32 {
    return match optional {
        Some(&frequency) => frequency,
        None => 0
    };
}

#[cfg(test)]
mod tests {
    use super::find_number_of_pairs as compute;

    #[test]
    fn test_with_positive_set1_is_successful() {
        let result = compute(&[2, 1, 6], &[1, 5]);
        assert_eq!(3, result);
    }

    #[test]
    fn test_with_positive_set2_is_successful() {
        let result = compute(&[10, 19, 18], &[11, 15, 9]);
        assert_eq!(2, result);
    }

    #[test]
    fn test_with_positive_set3_is_successful() {
        let result = compute(&[1, 1, 1], &[1, 1, 1]);
        assert_eq!(0, result);
    }

    #[test]
    fn test_with_positive_set4_is_successful() {
        let result = compute(&[0, 2], &[5, 0, 1]);
        assert_eq!(3, result);
    }
}