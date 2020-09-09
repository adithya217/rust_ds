
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

#[cfg(test)]
mod tests {
    use super::find_smallest_negative_number as compute;

    #[test]
    fn findSmallestNegativeNumber_NoNegativeNumbers_Successful() {
        let test_data = [1, 2, 3, 4];
        let result = compute(&test_data);

        assert_eq!(None, result);
    }

    #[test]
    fn findSmallestNegativeNumber_OneNegativeNumber_Successful() {
        let test_data = [1, 2, -1, 3, 4];
        let result = compute(&test_data);

        assert_eq!(Some((&-1, 2)), result);
    }

    #[test]
    fn findSmallestNegativeNumber_MultipleNegativeNumbers_Successful() {
        let test_data = [-3, 1, 5, 2, -4, -5, 3, 4, -2];
        let result = compute(&test_data);

        assert_eq!(Some((&-5, 5)), result);
    }
}