use std::cmp::max;
use std::mem::swap;

pub fn sort_separate_arrays(arr1: &mut [i32], arr2: &mut [i32]) {
    let arr1_len = arr1.len();
    let combined_arr_len = arr1.len() + arr2.len();
    let mut gap = max(combined_arr_len / 2, (combined_arr_len + 1) / 2);

    while gap != 0 {
        let mut start_index = 0;

        while start_index + gap < combined_arr_len {
            let end_index = start_index + gap;

            match (start_index, end_index) {
                (i, j) if i < arr1.len() && j < arr1.len() => {
                    if arr1[i] > arr1[j] {
                        arr1.swap(i, j);
                    }
                },
                (i, j) if i < arr1.len() && j < combined_arr_len => {
                    let j = j - arr1_len;

                    if arr1[i] > arr2[j] {
                        swap(&mut arr1[i], &mut arr2[j]);
                    }
                },
                (i, j) if i < combined_arr_len && j < combined_arr_len => {
                    let i = i - arr1_len;
                    let j = j - arr1_len;

                    if arr2[i] > arr2[j] {
                        arr2.swap(i, j);
                    }
                },
                (_, _) => {}
            }

            start_index += 1;
        }

        gap = if gap <=1 { 0 } else { max(gap / 2, (gap + 1) / 2) };
    }
}

#[cfg(test)]
mod tests {
    use super::sort_separate_arrays as compute;

    #[test]
    fn test_with_positive_set1_is_successful() {
        let mut arr1 = [10];
        let mut arr2 = [2, 3];

        compute(&mut arr1, &mut arr2);

        assert_eq!([2], arr1);
        assert_eq!([3, 10], arr2);
    }

    #[test]
    fn test_with_positive_set2_is_successful() {
        let mut arr1 = [1, 5, 9, 10, 15, 20];
        let mut arr2 = [2, 3, 8, 13];

        compute(&mut arr1, &mut arr2);

        assert_eq!([1, 2, 3, 5, 8, 9], arr1);
        assert_eq!([10, 13, 15, 20], arr2);
    }

    #[test]
    fn test_with_positive_set3_is_successful() {
        let mut arr1 = [1, 3, 5, 7];
        let mut arr2 = [0, 2, 6, 8, 9];

        compute(&mut arr1, &mut arr2);

        assert_eq!([0, 1, 2, 3], arr1);
        assert_eq!([5, 6, 7, 8, 9], arr2);
    }

    #[test]
    fn test_with_positive_set4_is_successful() {
        let mut arr1 = [10, 12];
        let mut arr2 = [5, 18, 20];

        compute(&mut arr1, &mut arr2);

        assert_eq!([5, 10], arr1);
        assert_eq!([12, 18, 20], arr2);
    }

    #[test]
    fn test_with_positive_set5_is_successful() {
        let mut arr1 = [3, 27, 38, 43];
        let mut arr2 = [9, 10, 82];

        compute(&mut arr1, &mut arr2);

        assert_eq!([3, 9, 10, 27], arr1);
        assert_eq!([38, 43, 82], arr2);
    }

    #[test]
    fn test_with_positive_set6_is_successful() {
        let mut arr1 = [10, 27, 38, 43, 82];
        let mut arr2 = [3, 9];

        compute(&mut arr1, &mut arr2);

        assert_eq!([3, 9, 10, 27, 38], arr1);
        assert_eq!([43, 82], arr2);
    }
}