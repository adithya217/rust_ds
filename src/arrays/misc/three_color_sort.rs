
pub fn sort_three_color_array(arr: &mut [u8], mid_val: u8) {
    let mut low = 0;
    let mut mid = 0;
    let mut high = arr.len();

    while mid < high {
        let curr_mid_element = arr[mid];
        if curr_mid_element < mid_val {
            arr.swap(low, mid);
            low += 1;
            mid += 1;
        } else if curr_mid_element > mid_val {
            high -= 1;
            arr.swap(mid, high);
        } else {
            mid += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::sort_three_color_array as compute;

    #[test]
    fn test_with_positive_set1_is_successful() {
        let mut arr = [0, 2, 1, 1, 2, 0, 0, 2, 1];
        compute(&mut arr, 1);

        assert_eq!([0, 0, 0, 1, 1, 1, 2, 2, 2], arr);
    }

    #[test]
    fn test_with_positive_set2_is_successful() {
        let mut arr = [0, 1, 2, 0, 1, 2];
        compute(&mut arr, 1);

        assert_eq!([0, 0, 1, 1, 2, 2], arr);
    }

    #[test]
    fn test_with_positive_set3_is_successful() {
        let mut arr = [0, 1, 1, 0, 1, 2, 1, 2, 0, 0, 0, 1];
        compute(&mut arr, 1);

        assert_eq!([0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 2, 2], arr);
    }

    #[test]
    fn test_with_positive_set4_is_successful() {
        let mut arr = [0, 2, 1, 2, 0];
        compute(&mut arr, 1);

        assert_eq!([0, 0, 1, 2, 2], arr);
    }

    #[test]
    fn test_with_positive_set5_is_successful() {
        let mut arr = [0, 1, 0];
        compute(&mut arr, 1);

        assert_eq!([0, 0, 1], arr);
    }
}