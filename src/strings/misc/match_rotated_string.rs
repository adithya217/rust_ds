
use std::vec::Vec;

pub fn check_if_matches(target: &str, src: &str, offset: usize) -> Option<bool> {
    let target_len = target.len();
    let src_len = src.len();

    if target_len != src_len {
        return None;
    }

    if target_len == 0 {
        return Some(true);
    }

    let offset = offset % src_len;
    let target_chars: Vec<char> = target.chars().collect();
    let src_chars: Vec<char> = src.chars().collect();

    let mut left_index = offset;
    let mut right_index = src_len - offset;

    let mut does_left_match = true;
    let mut does_right_match = true;

    for letter in target_chars {
        if letter != src_chars[left_index] {
            does_left_match = false;
        }

        if letter != src_chars[right_index] {
            does_right_match = false;
        }

        left_index = (left_index + 1) % src_len;
        right_index = (right_index + 1) % src_len;
    }

    return Some(does_left_match || does_right_match);
}

#[cfg(test)]
mod tests {
    use super::check_if_matches as compute;

    #[test]
    pub fn test_with_different_length_strings() {
        let src = "abcdefg";
        let target = "xyz";
        let rotation_count = target.len();
        let expected = None;
        assert_eq!(expected, compute(&target, &src, rotation_count));
    }

    #[test]
    pub fn test_with_empty_strings() {
        let src = "";
        let target = "";
        let rotation_count = target.len();
        let expected = Some(true);
        assert_eq!(expected, compute(&target, &src, rotation_count));
    }

    #[test]
    pub fn test_with_rotation_count_greater_than_string_length_for_match() {
        let src = "abcdefg";
        let target = "cdefgab";
        let rotation_count = target.len() + 2;
        let expected = Some(true);
        assert_eq!(expected, compute(&target, &src, rotation_count));
    }

    #[test]
    pub fn test_with_rotation_count_greater_than_string_length_for_non_match() {
        let src = "xyzzzyx";
        let target = "cdefgab";
        let rotation_count = target.len() + 2;
        let expected = Some(false);
        assert_eq!(expected, compute(&target, &src, rotation_count));
    }

    #[test]
    pub fn test_with_left_rotated_string_match() {
        let src = "xyzzyx";
        let target = "zyxxyz";
        let rotation_count = 3;
        let expected = Some(true);
        assert_eq!(expected, compute(&target, &src, rotation_count));
    }

    #[test]
    pub fn test_with_left_rotated_string_non_match() {
        let src = "xyzzyx";
        let target = "yzzyxx";
        let rotation_count = 3;
        let expected = Some(false);
        assert_eq!(expected, compute(&target, &src, rotation_count));
    }

    #[test]
    pub fn test_with_right_rotated_string_match() {
        let src = "abcdefg";
        let target = "fgabcde";
        let rotation_count = 2;
        let expected = Some(true);
        assert_eq!(expected, compute(&target, &src, rotation_count));
    }

    #[test]
    pub fn test_with_right_rotated_string_non_match() {
        let src = "abcdefg";
        let target = "efgabcd";
        let rotation_count = 3;
        let expected = Some(true);
        assert_eq!(expected, compute(&target, &src, rotation_count));
    }
}