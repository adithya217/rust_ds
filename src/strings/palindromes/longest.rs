
use std::vec::Vec;

pub fn find_longest(word: &str) -> Option<(usize, usize)> {
    match word.len() {
        0 => { return None; },
        1 => { return Some((0, 0)); }
        _ => {}
    }

    let letters: Vec<char> = word.chars().collect();

    let mut max_len = 0;
    let mut result = None;

    for index in 0..letters.len() {
        let (max_len_odd, left_odd, right_odd) = find_palindrome(&letters, index, index);
        let (max_len_even, left_even, right_even) = find_palindrome(&letters, index, index + 1);
        let (temp_max_len, left, right) = if max_len_odd > max_len_even {
            (max_len_odd, left_odd, right_odd)
        } else {
            (max_len_even, left_even, right_even)
        };

        if temp_max_len > max_len {
            max_len = temp_max_len;
            result = Some((left, right));
        }
    }

    return result;
}

fn find_palindrome(letters: &Vec<char>, start: usize, end: usize) -> (usize, usize, usize) {
    let left_bound = 0;
    let right_bound = (letters.len() - 1) as i32;
    let mut left = start as i32;
    let mut right = end as i32;

    while left >= left_bound && right <= right_bound {
        if letters.get(left as usize).as_deref() != letters.get(right as usize).as_deref() {
            break;
        }

        left -= 1;
        right += 1;
    }

    left += 1;
    right -= 1;

    return ((right - left + 1) as usize, left as usize, right as usize);
}

#[cfg(test)]
mod tests {
    use super::find_longest as compute;

    #[test]
    fn test_with_empty_string() {
        let input = "";
        let expected = None;
        assert_eq!(expected, compute(&input));
    }

    #[test]
    fn test_with_even_sized_string_with_even_sized_palindrome() {
        let input = "aaaabbaa";
        let expected = Some((2, 7));
        assert_eq!(expected, compute(&input));
    }

    #[test]
    fn test_with_odd_sized_string_with_odd_sized_palindrome() {
        let input = "babad";
        let expected = Some((0, 2));
        assert_eq!(expected, compute(&input));
    }

    #[test]
    fn test_with_single_letter_string() {
        let input = "a";
        let expected = Some((0, 0));
        assert_eq!(expected, compute(&input));
    }

    #[test]
    fn test_with_dual_letter_string_with_single_letter_palindrome() {
        let input = "ac";
        let expected = Some((0, 0));
        assert_eq!(expected, compute(&input));
    }

    #[test]
    fn test_with_random_string() {
        let input = "forgeeksskeegfor";
        let expected = Some((3, 12));
        assert_eq!(expected, compute(&input));
    }
}