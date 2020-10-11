
use std::vec::Vec;

pub fn remove_adjacent_duplicates(data: &str) -> Option<String> {
    match data.len() {
        0 => return None,
        1 => return Some(String::from(data)),
        _ => {}
    }

    let mut result: Vec<char> = Vec::new();
    let mut prev_char: Option<char> = None;
    let mut is_prev_char_popped = false;

    for letter in data.chars() {
        let current_letter = Some(letter);

        if prev_char == current_letter {
            if !is_prev_char_popped {
                result.pop();
                is_prev_char_popped = true;
            }
            continue;
        }

        prev_char = result.pop();
        if prev_char == current_letter {
            is_prev_char_popped = true;
            continue;
        }

        if prev_char != None {
            result.push(prev_char.unwrap());
        }

        prev_char = current_letter;
        result.push(letter);
        is_prev_char_popped = false;
    }

    return Some(result.iter().collect());
}

#[cfg(test)]
mod tests {
    use super::remove_adjacent_duplicates as compute;

    #[test]
    fn test_with_adjacent_duplicates_in_the_middle() {
        let input = "geeksforgeek";
        let expected = Some(String::from("gksforgk"));
        assert_eq!(expected, compute(&input));
    }

    #[test]
    fn test_with_multiple_repeating_adjacent_duplicates() {
        let input = "acaaabbbacdddd";
        let expected = Some(String::from("acac"));
        assert_eq!(expected, compute(&input));
    }

    #[test]
    fn test_with_repeating_adjacent_duplicates_after_a_gap() {
        let input = "azxxzy";
        let expected = Some(String::from("ay"));
        assert_eq!(expected, compute(&input));
    }

    #[test]
    fn test_with_a_palindrome() {
        let input = "azxxza";
        let expected = Some(String::from(""));
        assert_eq!(expected, compute(&input));
    }

    #[test]
    fn test_with_repeating_adjacent_duplicates_in_the_middle_and_at_the_end() {
        let input = "geeksforgeeg";
        let expected = Some(String::from("gksfor"));
        assert_eq!(expected, compute(&input));
    }

    #[test]
    fn test_with_no_uniques() {
        let input = "caaabbbaacdddd";
        let expected = Some(String::from(""));
        assert_eq!(expected, compute(&input));
    }
}