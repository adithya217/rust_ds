
use std::cmp::min;
use std::collections::LinkedList;

pub fn find_prefix(text_arr: &[&str]) -> Option<String> {
    match text_arr.len() {
        0 => { return None; },
        1 => { return Some(String::from(text_arr[0])); },
        _ => {}
    }

    let (min_text_len, text_vec): (usize, Vec<Vec<char>>) = text_arr.iter()
        .fold((usize::MAX, Vec::with_capacity(text_arr.len())), |(min_len, mut sink), text| {
            let letters: Vec<char> = text.chars().collect();
            let len = letters.len();

            sink.push(letters);
            (min(min_len, len), sink)
        });
    let mut longest_prefix_chars: LinkedList<char> = LinkedList::new();

    'outer: for char_index in 0..min_text_len {
        let common_letter = text_vec[0][char_index];

        for text_index in 1..text_vec.len() {
            let current_letter = text_vec[text_index][char_index];
            if common_letter != current_letter {
                break 'outer;
            }
        }

        longest_prefix_chars.push_back(common_letter);
    }

    return Some(longest_prefix_chars.iter().collect());
}

#[cfg(test)]
mod tests {
    use super::find_prefix as compute;

    #[test]
    fn test_with_empty_array() {
        let input = [];
        let expected = None;
        assert_eq!(expected, compute(&input));
    }

    #[test]
    fn test_with_single_sentence() {
        let input = [
            "Hey there!"
        ];
        let expected = Some(String::from("Hey there!"));
        assert_eq!(expected, compute(&input));
    }

    #[test]
    fn test_with_single_empty_sentence() {
        let input = [
            ""
        ];
        let expected = Some(String::from(""));
        assert_eq!(expected, compute(&input));
    }

    #[test]
    fn test_with_smallest_sentence_empty() {
        let input = [
            "a",
            "ab",
            "abc",
            ""
        ];
        let expected = Some(String::from(""));
        assert_eq!(expected, compute(&input));
    }

    #[test]
    fn test_with_medium_sized_common_prefix() {
        let input = [
            "geeksforgeeks",
            "geezer",
            "geek",
            "geeks"
        ];
        let expected = Some(String::from("gee"));
        assert_eq!(expected, compute(&input));
    }

    #[test]
    fn test_with_small_sized_common_prefix() {
        let input = [
            "apple",
            "ape",
            "april"
        ];
        let expected = Some(String::from("ap"));
        assert_eq!(expected, compute(&input));
    }

    #[test]
    fn test_with_no_common_prefix_in_all_strings() {
        let input = [
            "ab",
            "cde",
            "fg"
        ];
        let expected = Some(String::from(""));
        assert_eq!(expected, compute(&input));
    }

    #[test]
    fn test_with_no_common_prefix_in_one_strings() {
        let input = [
            "abc",
            "abcde",
            "defg"
        ];
        let expected = Some(String::from(""));
        assert_eq!(expected, compute(&input));
    }
}