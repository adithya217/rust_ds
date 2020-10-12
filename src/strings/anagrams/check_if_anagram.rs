
use std::vec::Vec;

pub fn is_anagram(target: &str, src: &str) -> Option<usize> {
    if target.len() > src.len() {
        return None;
    }

    if target.len() == 0 {
        return None;
    }

    let target_letters: Vec<char> = target.chars().collect();
    let src_letters: Vec<char> = src.chars().collect();

    let src_len = src_letters.len();
    let target_len = target_letters.len();

    for src_index in 0..src_len {
        let mut start = src_index;
        let mut found = true;

        for target_index in 0..target_len {
            if src_letters[start] != target_letters[target_index] {
                found = false;
                break;
            }

            start = (start + 1) % src_len;
        }

        if found {
            return Some(src_index);
        }
    }

    return None;
}

#[cfg(test)]
mod tests {
    use super::is_anagram as compute;

    #[test]
    fn test_with_different_length_strings() {
        let src = "xyz";
        let target = "abcdefg";
        let expected = None;
        assert_eq!(expected, compute(target, src));
    }

    #[test]
    fn test_with_empty_strings() {
        let src = "";
        let target = "";
        let expected = None;
        assert_eq!(expected, compute(target, src));
    }

    #[test]
    fn test_with_anagram_at_start() {
        let src = "abcdefg";
        let target = "abcdefg";
        let expected = Some(0);
        assert_eq!(expected, compute(target, src));
    }

    #[test]
    fn test_with_anagram_at_middle() {
        let src = "abcdefg";
        let target = "cdefgab";
        let expected = Some(2);
        assert_eq!(expected, compute(target, src));
    }

    #[test]
    fn test_with_anagram_at_end() {
        let src = "abcdefg";
        let target = "gabcdef";
        let expected = Some(6);
        assert_eq!(expected, compute(target, src));
    }

    #[test]
    fn test_with_anagram_when_target_smaller_than_src() {
        let src = "abcdefg";
        let target = "cde";
        let expected = Some(2);
        assert_eq!(expected, compute(target, src));
    }
}