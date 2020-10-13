
use std::vec::Vec;
use std::collections::HashSet;

pub fn remove(src: &str) -> String {
    if src.len() < 2 {
        return String::from(src);
    }

    let mut visited_chars: HashSet<char> = HashSet::new();
    let mut unique_chars: Vec<char> = vec![];

    for letter in src.chars() {
        if !visited_chars.contains(&letter) {
            visited_chars.insert(letter);
            unique_chars.push(letter);
        }
    }

    return unique_chars.iter().collect();
}

#[cfg(tests)]
mod tests {
    use super::remove as compute;

    #[test]
    fn test_with_empty_string() {
        let input = "";
        let expected = "";
        assert_eq!(expected, compute(&input));
    }

    #[test]
    fn test_with_single_char_string() {
        let input = "a";
        let expected = "a";
        assert_eq!(expected, compute(&input));
    }

    #[test]
    fn test_with_no_repetitions() {
        let input = "cwm fjord veg balks nth pyx quiz";
        let expected = "cwm fjord veg balks nth pyx quiz";
        assert_eq!(expected, compute(&input));
    }

    #[test]
    fn test_with_repetitions() {
        let input = "hello there";
        let expected = "helo thr";
        assert_eq!(expected, compute(&input));
    }
}