use std::cmp::max;

pub fn rearrange_alternatively(arr: &mut [i32]) {
    let offset = arr.iter().fold(&i32::MIN, |acc, number| max(acc, number)) + 1;
    let mut min_index = 0;
    let mut max_index = arr.len() - 1;

    for index in 0..arr.len() {
        if index % 2 == 0 {
            arr[index] += (arr[max_index] % offset) * offset;
            max_index -= 1;
        } else {
            arr[index] += (arr[min_index] % offset) * offset;
            min_index += 1;
        }
    }

    for index in 0..arr.len() {
        arr[index] = arr[index] / offset;
    }
}

#[cfg(test)]
mod tests {
    use super::rearrange_alternatively as compute;

    #[test]
    fn test_with_positive_set1_is_successful() {
        let mut test_data = [1, 2, 3, 4, 5, 6];
        compute(&mut test_data);

        assert_eq!([6, 1, 5, 2, 4, 3], test_data);
    }

    #[test]
    fn test_with_positive_set2_is_successful() {
        let mut test_data = [10, 20, 30, 40, 50, 60, 70, 80, 90, 100, 110];
        compute(&mut test_data);

        assert_eq!([110, 10, 100, 20, 90, 30, 80, 40, 70, 50, 60], test_data);
    }
}