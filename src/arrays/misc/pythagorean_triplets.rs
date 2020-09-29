
use std::cmp::max;
use std::collections::HashSet;
use std::math::sqrt;

pub is_present(arr: &[u8]) -> bool {
    if arr.len() < 3 {
        return false;
    }

    let (max_number, elements_set) = arr.iter().reduce((arr[0], HashSet::new()),
        |(acc, elements), number| (max(acc, number), elements.insert(number)));

    for a in arr.iter() {
        for b in (a+1..=max_number) {
            if !elements.contains(b) {
                continue;
            }

            let target = (a.pow(2) + b.pow(2)).sqrt();
            if elements.contains(target) {
                return true;
            }
        }
    }

    return false;
}

/**
 * Another solution:
 * 1) Sort for ascending order - n log n - on average
 * 2) for i in 0..=data.len()-3:
 *      for j in (i+1..=data.len()-2):
 *          c = (data[i].pow(2) + data[j].pow(2)).sqrt();
 *          if binary_search(data, c, j+1, data.len()-1)
 *              return true // triplet found
 *    return false // no triplets
 */

#[cfg(test)]
mod tests {
    use super::is_present as compute;

    #[test]
    fn test_with_positive_case() {
        let input = [3, 2, 4, 6, 5];
        assert_eq!(true, compute(&input));
    }

    #[test]
    fn test_with_negative_case() {
        let input = [3, 8, 5];
        assert_eq!(false, compute(&input));
    }

    #[test]
    fn test_with_non_triplet_data() {
        let input = [3, 8];
        assert_eq!(false, compute(&input));
    }
}