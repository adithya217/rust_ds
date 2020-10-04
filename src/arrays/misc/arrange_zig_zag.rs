
#[derive(Clone, Copy)]
pub enum State {
    Lesser,
    Greater
}

pub fn arrange(arr: &mut [i32]) {
    if arr.len() < 2 {
        return;
    }

    let mut state = State::Lesser;

    for index in 0..arr.len() - 1 {
        let curr_less_than_next = arr[index] < arr[index + 1];

        let (should_swap, next_state) = match state {
            State::Lesser => (!curr_less_than_next, State::Greater),
            State::Greater => (curr_less_than_next, State::Lesser)
        };

        if should_swap {
            arr.swap(index, index + 1);
        }

        state = next_state;
    }
}

#[cfg(tests)]
mod tests {
    use super::arrange as compute;

    #[test]
    fn test_with_moderate_sized_all_positive_input() {
        let mut input = [4, 3, 7, 8, 6, 2, 1];
        let output = [3, 7, 4, 8, 2, 6, 1];

        assert_eq!(output, compute(&mut input));
    }

    #[test]
    fn test_with_small_one_change_positive_input() {
        let mut input = [1, 4, 3, 2];
        let output = [1, 4, 2, 3];

        assert_eq!(output, compute(&mut input));
    }

    #[test]
    fn test_with_moderate_sized_mixed_input() {
        let mut input = [4, 3, -7, -8, 6, 2, -1];
        let output = [3, 4, -7, 6, -8, 2, -1];

        assert_eq!(output, compute(&mut input));
    }

    #[test]
    fn test_with_all_same_elements_input() {
        let mut input = [0, 0, 0, 0];
        let output = [0, 0, 0, 0];

        assert_eq!(output, compute(&mut input));
    }

    #[test]
    fn test_with_no_input() {
        let mut input = [];
        let output = [];

        assert_eq!(output, compute(&mut input));
    }
}