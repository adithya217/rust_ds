
use std::collections::VecDeque;

pub fn find_smallest_negative_number(arr: &[i32]) -> Option<(&i32, usize)> {
    let mut smallest: &i32 = &0;
    let mut found_index: usize = 0;

    for (index, number) in arr.iter().enumerate() {
        if number < smallest {
            smallest = number;
            found_index = index;
        }
    }

    if smallest == &0 {
        return None;
    } else {
        return Some((smallest, found_index));
    }
}

pub fn find_largest_number(arr: &[u32]) -> Option<(&u32, usize)> {
    if arr.len() < 1 {
        return None;
    }

    let (mut largest, mut found_index) = (&arr[0], 0);

    for (index, number) in arr.iter().enumerate() {
        if number > largest {
            largest = number;
            found_index = index;
        }
    }

    return Some((largest, found_index));
}

pub fn count_digits(num: u32) -> u32 {
    if num < 10 {
        return 1;
    }

    let mut count = 0;
    let mut num = num;

    while num != 0 {
        num = num / 10;
        count += 1;
    }

    return count;
}

pub fn get_individual_digits(num: u32) -> VecDeque<u32> {
    let mut digits = VecDeque::new();
    let mut num = num;

    loop {
        digits.push_front(num % 10);
        num = num / 10;
        if num == 0 {
            break;
        }
    }

    return digits;
}

pub fn pad_with_own_digits(num: u32, target_len: u32) -> u32 {
    if num == 0 {
        return num;
    }

    let digits = get_individual_digits(num);
    if digits.len() >= target_len as usize {
        return num;
    }

    let mut padded_number = num;
    let mut padding_offset = target_len - digits.len() as u32;
    let mut index = 0;
    while padding_offset != 0 {
        padded_number = (padded_number * 10) + digits[index];
        index = (index + 1) % digits.len();
        padding_offset -= 1;
    }

    return padded_number;
}

#[cfg(test)]
mod tests {
    use super::find_smallest_negative_number as compute;
    use super::find_largest_number as compute_largest;
    use super::count_digits as count_digits;
    use super::get_individual_digits as get_individual_digits;
    use std::collections::VecDeque;
    use super::pad_with_own_digits as pad_with_own_digits;

    #[test]
    fn find_smallest_negative_number_in_all_positive_numbers() {
        let test_data = [1, 2, 3, 4];
        let result = compute(&test_data);

        assert_eq!(None, result);
    }

    #[test]
    fn find_smallest_negative_number_when_only_one_negative_is_present() {
        let test_data = [1, 2, -1, 3, 4];
        let result = compute(&test_data);

        assert_eq!(Some((&-1, 2)), result);
    }

    #[test]
    fn find_smallest_negative_number_when_multiple_negatives_are_present() {
        let test_data = [-3, 1, 5, 2, -4, -5, 3, 4, -2];
        let result = compute(&test_data);

        assert_eq!(Some((&-5, 5)), result);
    }

    #[test]
    fn find_largest_number_in_empty_input() {
        let test_data = [];
        let result = compute_largest(&test_data);

        assert_eq!(None, result);
    }

    #[test]
    fn find_largest_number_from_all_positive_numbers() {
        let test_data = [1, 2, 3, 4, 5];
        let result = compute_largest(&test_data);

        assert_eq!(Some((&5, 4)), result);
    }

    #[test]
    fn find_number_of_digits_in_zero() {
        let test_data = 0;
        let result = count_digits(test_data);

        assert_eq!(1, result);
    }

    #[test]
    fn find_number_of_digits_in_some_number() {
        let test_data = 12345;
        let result = count_digits(test_data);

        assert_eq!(5, result);
    }

    #[test]
    fn get_individual_digits_of_zero() {
        let test_data = 0;
        let result = get_individual_digits(test_data);

        let mut expected = VecDeque::new();
        expected.push_front(0);

        assert_eq!(result, result);
    }

    #[test]
    fn get_individual_digits_of_some_number() {
        let test_data = 12345;
        let result = get_individual_digits(test_data);

        let mut expected = VecDeque::new();
        expected.push_back(1);
        expected.push_back(2);
        expected.push_back(3);
        expected.push_back(4);
        expected.push_back(5);

        assert_eq!(result, result);
    }

    #[test]
    fn test_padding_for_zero() {
        let result = pad_with_own_digits(0, 5);
        assert_eq!(0, result);
    }

    #[test]
    fn test_no_truncation_on_padding_for_number_bigger_than_target_padded_length() {
        let result = pad_with_own_digits(12121, 5);
        assert_eq!(12121, result);
    }

    #[test]
    fn test_padding_for_some_number() {
        let result = pad_with_own_digits(121, 6);
        assert_eq!(121121, result);
    }
}