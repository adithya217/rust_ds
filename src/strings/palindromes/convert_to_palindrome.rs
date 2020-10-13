
use super::longest::find_longest;
use std::vec::Vec;

pub fn compute_min_insert_count(src: &str) -> Option<usize> {
    match src.len() {
        0 => { return None; },
        1 => { return Some(0); }
        _ => {}
    }

    let max_palindrome_size = match find_longest(src) {
        None => 0,
        Some((left, right)) => right - left + 1
    };

    let letters: Vec<char> = src.chars().collect();

    if max_palindrome_size > 0 {
        return Some(letters.len() - max_palindrome_size);
    }

    let mut left = 0;
    let mut right = letters.len() - 1;
    while left < right && letters[left] == letters[right] {
        left += 1;
        right -= 1;
    }

    // Else case is size of non-palindromic part of string - 1
    let min_insert_count = if left > right { 0 } else { right - left };
    return Some(min_insert_count);
}

#[cfg(tests)]
mod tests {
    use super::compute_min_insert_count as compute;

    #[test]
    fn test_with_empty_string() {
        let input = "";
        let expected = None;
        assert_eq!(expected, compute(input));
    }

    #[test]
    fn test_with_single_char_string() {
        let input = "a";
        let expected = Some(0);
        assert_eq!(expected, compute(input));
    }

    #[test]
    fn test_with_odd_sized_palindrome_at_start_of_odd_sized_string() {
        let input = "babef";
        let expected = Some(2);
        assert_eq!(expected, compute(input));
    }

    #[test]
    fn test_with_odd_sized_palindrome_in_middle_of_odd_sized_string() {
        let input = "fbabe";
        let expected = Some(2);
        assert_eq!(expected, compute(input));
    }

    #[test]
    fn test_with_odd_sized_palindrome_at_end_of_odd_sized_string() {
        let input = "efbab";
        let expected = Some(2);
        assert_eq!(expected, compute(input));
    }

    #[test]
    fn test_with_odd_sized_palindrome_at_start_of_even_sized_string() {
        let input = "babe";
        let expected = Some(1);
        assert_eq!(expected, compute(input));
    }

    #[test]
    fn test_with_odd_sized_palindrome_in_middle_of_even_sized_string() {
        let input = "xybabz";
        let expected = Some(1);
        assert_eq!(expected, compute(input));
    }

    #[test]
    fn test_with_odd_sized_palindrome_at_end_of_even_sized_string() {
        let input = "xyzbab";
        let expected = Some(2);
        assert_eq!(expected, compute(input));
    }

    #[test]
    fn test_with_even_sized_palindrome_at_start_of_odd_sized_string() {
        let input = "baabxyz";
        let expected = Some(3);
        assert_eq!(expected, compute(input));
    }

    #[test]
    fn test_with_even_sized_palindrome_in_middle_of_odd_sized_string() {
        let input = "xybaabz";
        let expected = Some(3);
        assert_eq!(expected, compute(input));
    }

    #[test]
    fn test_with_even_sized_palindrome_at_end_of_odd_sized_string() {
        let input = "xxa";
        let expected = Some(1);
        assert_eq!(expected, compute(input));
    }

    #[test]
    fn test_with_even_sized_palindrome_at_start_of_even_sized_string() {
        let input = "baabdc";
        let expected = Some(2);
        assert_eq!(expected, compute(input));
    }

    #[test]
    fn test_with_even_sized_palindrome_in_middle_of_even_sized_string() {
        let input = "dbaabc";
        let expected = Some(2);
        assert_eq!(expected, compute(input));
    }

    #[test]
    fn test_with_even_sized_palindrome_at_end_of_even_sized_string() {
        let input = "dcbaab";
        let expected = Some(2);
        assert_eq!(expected, compute(input));
    }

    #[test]
    fn test_with_even_sized_palindrome_in_a_slightly_palindromic_string() {
        let input = "abxxca";
        let expected = Some(2);
        assert_eq!(expected, compute(input));
    }

    #[test]
    fn test_with_no_palindrome_in_a_slightly_odd_palindromic_string() {
        let input = "abcda";
        let expected = Some(2);
        assert_eq!(expected, compute(input));
    }

    #[test]
    fn test_with_no_palindrome_in_a_slightly_even_palindromic_string() {
        let input = "abcdea";
        let expected = Some(3);
        assert_eq!(expected, compute(input));
    }
}