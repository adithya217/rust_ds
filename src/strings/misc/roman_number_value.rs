
use lazy_static::lazy_static;
use regex::Regex;
use std::vec::Vec;

enum RomanNumeral {
    I, V, X, L, C, D, M
}

impl RomanNumeral {
    fn get_char(&self) -> char {
        match self {
            RomanNumeral::I => 'I',
            RomanNumeral::V => 'V',
            RomanNumeral::X => 'X',
            RomanNumeral::L => 'L',
            RomanNumeral::C => 'C',
            RomanNumeral::D => 'D',
            RomanNumeral::M => 'M'
        }
    }

    fn get_value(&self) -> u16 {
        match self {
            RomanNumeral::I => 1,
            RomanNumeral::V => 5,
            RomanNumeral::X => 10,
            RomanNumeral::L => 50,
            RomanNumeral::C => 100,
            RomanNumeral::D => 500,
            RomanNumeral::M => 1000
        }
    }

    fn get_from_char(letter: char) -> Option<RomanNumeral> {
        match letter {
            'i' | 'I' => Some(RomanNumeral::I),
            'v' | 'V' => Some(RomanNumeral::V),
            'x' | 'X' => Some(RomanNumeral::X),
            'l' | 'L' => Some(RomanNumeral::L),
            'c' | 'C' => Some(RomanNumeral::C),
            'd' | 'D' => Some(RomanNumeral::D),
            'm' | 'M' => Some(RomanNumeral::M),
            _ => None
        }
    }
}

lazy_static! {
    static ref ROMAN_NUMBER_DISALLOWED_CHARS: Regex
        = Regex::new(r"(?i)[^IVXLCDM]").unwrap();
    
    static ref ROMAN_NUMBER_DISALLOWED_COMBINATIONS: Regex
        = Regex::new(r"(?i)IL|IC|ID|IM|VX|VL|VC|VD|VM|XD|XM|LC|LD|LM|DM").unwrap();
}

pub fn compute_value(roman_number: &str) -> Option<u32> {
    if roman_number.len() == 0 {
        return None;
    }

    if ROMAN_NUMBER_DISALLOWED_CHARS.find(roman_number) != None {
        return None;
    }

    if ROMAN_NUMBER_DISALLOWED_COMBINATIONS.find(roman_number) != None {
        return None;
    }

    let letters: Vec<char> = roman_number.chars().rev().collect();
    let mut prev_value = 0;
    let mut total_value = prev_value as u32;

    for index in 0..letters.len() {
        let curr_value = RomanNumeral::get_from_char(letters[index]).unwrap().get_value();
        if curr_value >= prev_value {
            total_value += curr_value as u32;
        } else {
            total_value -= curr_value as u32;
        }

        prev_value = curr_value;
    }

    return Some(total_value);
}

#[cfg(test)]
mod tests {
    use super::compute_value as compute;

    #[test]
    fn test_with_empty_string() {
        let input = "";
        let expected = None;
        assert_eq!(expected, compute(input));
    }

    #[test]
    fn test_with_non_roman_numeral_string() {
        let input = "abcdefg";
        let expected = None;
        assert_eq!(expected, compute(input));
    }

    #[test]
    fn test_with_mixed_string() {
        let input = "XVabcIII";
        let expected = None;
        assert_eq!(expected, compute(input));
    }

    #[test]
    fn test_with_increasing_roman_numeral_string() {
        let input = "XVIII";
        let expected = Some(18);
        assert_eq!(expected, compute(input));
    }

    #[test]
    fn test_with_increasing_roman_numeral_string_in_lowercase() {
        let input = "xviii";
        let expected = Some(18);
        assert_eq!(expected, compute(input));
    }

    #[test]
    fn test_with_decreasing_roman_numeral_string() {
        let input = "XLIX";
        let expected = Some(49);
        assert_eq!(expected, compute(input));
    }

    #[test]
    fn test_with_decreasing_then_increasing_roman_numeral_string() {
        let input = "XLXIX";
        let expected = Some(59);
        assert_eq!(expected, compute(input));
    }

    #[test]
    fn test_with_invalid_roman_numeral_string() {
        let input = "xcdm";
        let expected = None;
        assert_eq!(expected, compute(input));
    }
}