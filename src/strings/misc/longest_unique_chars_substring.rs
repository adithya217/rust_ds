
use std::vec::Vec;
use std::collections::HashMap;

pub fn compute(src: &str) -> Option<Vec<(usize, usize)>> {
    match src.len() {
        0 => { return None; },
        1 => { return Some(vec![(0, 0)]); }
        _ => {}
    }

    let mut results: Vec<(usize, usize)> = vec![];
    let mut visited_letters: HashMap<char, usize> = HashMap::new();

    let mut max_len = 0;
    let mut start = 0;

    let letters: Vec<char> = src.chars().collect();

    for index in 0..letters.len() {
        let letter = letters[index];

        if let Some(&prev_index) = visited_letters.get(&letter) {
            clear_visited_letters(&letters, &mut visited_letters, start, prev_index);

            let prev_char_index = index - 1;
            let len_so_far = prev_char_index - start + 1; // length upto preceeding character
            if len_so_far > max_len {
                max_len = len_so_far;
                results = vec![(start, prev_char_index)];
            } else if len_so_far == max_len {
                results.push((start, prev_char_index));
            }

            start = prev_index + 1;
        }

        visited_letters.insert(letter, index);
    }

    let last_char_index = letters.len() - 1;
    let len_so_far = last_char_index - start + 1;
    if len_so_far > max_len {
        max_len = len_so_far;
        results = vec![(start, last_char_index)];
    } else if len_so_far == max_len {
        results.push((start, last_char_index));
    }

    return Some(results);
}

fn clear_visited_letters(src: &Vec<char>, target: &mut HashMap<char, usize>, start: usize, end: usize) {
    for index in start..=end {
        let letter = src[index];
        target.remove(&letter);
    }
}

#[cfg(test)]
mod tests {
    use super::compute as compute;

    #[test]
    fn test_with_empty_string() {
        let input = "";
        let expected = None;
        assert_eq!(expected, compute(input));
    }

    #[test]
    fn test_with_one_char_string() {
        let input = "a";
        let expected = Some(vec![(0, 0)]);
        assert_eq!(expected, compute(input));
    }

    #[test]
    fn test_with_two_possibilities() {
        let input = "abca";
        let expected = Some(vec![(0, 2), (1, 3)]);
        assert_eq!(expected, compute(input));
    }

    #[test]
    fn test_with_multiple_possibilities() {
        let input = "abababcdefababcdab";
        let expected = Some(vec![(4, 9), (5, 10), (6, 11)]);
        assert_eq!(expected, compute(input));
    }

    #[test]
    fn test_with_string_with_no_repeated_letters() {
        let input = "abcdefg";
        let expected = Some(vec![(0, 6)]);
        assert_eq!(expected, compute(input));
    }

    #[test]
    fn test_with_sequence_at_the_end() {
        let input = "abcabcdefg";
        let expected = Some(vec![(3, 9)]);
        assert_eq!(expected, compute(input));
    }
}