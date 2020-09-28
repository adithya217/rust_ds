
use super::super::utils::reverse_between_bounds_inclusive as reverse;
use std::cmp::min;

#[derive(Debug)]
#[derive(PartialEq)]
pub enum ErrorMsg {
    ArraySmallerThanGroup,
    GroupSizeIsZero
}

pub fn reverse_in_groups(arr: &mut [i32], group_size: usize) -> Result<(), ErrorMsg> {
    let arr_len = arr.len();
    if arr_len < group_size {
        return Err(ErrorMsg::ArraySmallerThanGroup);
    }

    if group_size == 0 {
        return Err(ErrorMsg::GroupSizeIsZero);
    }

    for start_index in (0..arr_len).step_by(group_size) {
        let end_index = min(start_index + group_size - 1, arr_len - 1);
        reverse(arr, start_index, end_index);
    }

    return Ok(());
}

#[cfg(test)]
mod tests {
    use super::reverse_in_groups as compute;
    use super::ErrorMsg as ErrorMsg;

    #[test]
    fn test_with_combo_of_complete_and_non_complete_groups_is_successful() {
        let mut input = [1, 2, 3, 4, 5];
        
        assert_eq!(Ok(()), compute(&mut input, 3));
        assert_eq!([3, 2, 1, 5, 4], input);
    }

    #[test]
    fn test_with_combo_of_complete_groups_is_successful() {
        let mut input = [10, 20, 30, 40, 50, 60];
        
        assert_eq!(Ok(()), compute(&mut input, 3));
        assert_eq!([30, 20, 10, 60, 50, 40], input);
    }

    #[test]
    fn test_with_array_smaller_than_group_input_not_modified() {
        let mut input = [1, 2, 3, 4, 5];
        let group_size = input.len() + 1;
        
        assert_eq!(Err(ErrorMsg::ArraySmallerThanGroup), compute(&mut input, group_size));
        assert_eq!([1, 2, 3, 4, 5], input);
    }

    #[test]
    fn test_for_complete_reversal_when_arr_size_same_as_group_size() {
        let mut input = [1, 2, 3, 4, 5];
        let group_size = input.len();
        
        assert_eq!(Ok(()), compute(&mut input, group_size));
        assert_eq!([5, 4, 3, 2, 1], input);
    }

    #[test]
    fn test_for_no_modification_when_group_size_is_0() {
        let mut input = [1, 2, 3, 4, 5];
        
        assert_eq!(Err(ErrorMsg::GroupSizeIsZero), compute(&mut input, 0));
        assert_eq!([1, 2, 3, 4, 5], input);
    }

    #[test]
    fn test_for_no_modification_when_group_size_is_1() {
        let mut input = [1, 2, 3, 4, 5];
        
        assert_eq!(Ok(()), compute(&mut input, 1));
        assert_eq!([1, 2, 3, 4, 5], input);
    }
}