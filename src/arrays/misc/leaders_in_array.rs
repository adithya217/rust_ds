
use std::vec::Vec;
use std::cmp::max;

pub fn find_leaders(arr: &[i32]) -> Vec<i32> {
    let mut leaders:Vec<i32> = Vec::new();
    leaders.push(arr[arr.len() - 1]);

    if arr.len() == 1 {
        return leaders;
    }

    let mut greatest = arr[arr.len() - 1];
    for index in (0..arr.len() - 1).rev() {
        if arr[index] >  greatest {
            leaders.push(arr[index]);
        }

        greatest = max(arr[index], greatest);
    }

    return leaders;
}

#[cfg(test)]
mod tests {
    use super::find_leaders as compute;

    #[test]
    fn test_with_positive_set1_is_successful() {
        let test_data = [16, 17, 4, 3, 5, 2];
        assert_eq!(vec![2, 5, 17], compute(&test_data));
    }

    #[test]
    fn test_with_positive_set2_is_successful() {
        let test_data = [1, 2, 3, 4, 0];
        assert_eq!(vec![0, 4], compute(&test_data));
    }

    #[test]
    fn test_with_positive_set3_is_successful() {
        let test_data = [4, 3, 2, -1];
        assert_eq!(vec![-1, 2, 3, 4], compute(&test_data));
    }
}