
use std::vec::Vec;

pub fn to_number(src: &str) -> Option<i128> {
    if src.len() == 0 {
        return None;
    }

    let letters: Vec<char> = src.chars().collect();
    let (negative_offset, mut number): (i8, u64) = match letters[0] {
        '-' => (-1, 0),
        letter if letter.is_ascii_digit() => (1, letter.to_digit(10).unwrap() as u64),
        _ => {
            return None;
        }
    };

    if letters.len() == 1 && negative_offset == -1 {
        return None;
    }

    for index in 1..letters.len() {
        let letter = letters[index];
        if !letter.is_ascii_digit() {
            return None;
        }

        number = (number * 10) + letter.to_digit(10).unwrap() as u64;
    }

    return Some(number as i128 * negative_offset as i128);
}

#[cfg(test)]
mod tests {
    use super::to_number as compute;

    #[test]
    fn test_with_empty_string() {
        let input = "";
        let expected = None;
        assert_eq!(expected, compute(input));
    }

    #[test]
    fn test_with_non_ascii_single_letter_string() {
        let input = "a";
        let expected = None;
        assert_eq!(expected, compute(input));
    }

    #[test]
    fn test_with_minus_single_letter_string() {
        let input = "-";
        let expected = None;
        assert_eq!(expected, compute(input));
    }

    #[test]
    fn test_with_single_digit_negative_number_string() {
        let input = "-9";
        let expected = Some(-9);
        assert_eq!(expected, compute(input));
    }

    #[test]
    fn test_with_single_digit_positive_number_string() {
        let input = "5";
        let expected = Some(5);
        assert_eq!(expected, compute(input));
    }

    #[test]
    fn test_with_multi_digit_positive_number_string() {
        let input = "9223372036854775819";
        let expected = Some(9223372036854775819);
        assert_eq!(expected, compute(input));
    }

    #[test]
    fn test_with_multi_digit_negative_number_string() {
        let input = "-9223372036854775819";
        let expected = Some(-9223372036854775819);
        assert_eq!(expected, compute(input));
    }
}