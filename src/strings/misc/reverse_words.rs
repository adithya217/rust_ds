
use std::collections::VecDeque;
use std::vec::Vec;

pub fn reverse(sentence: &str, separator: char) -> String {
    let mut word_order_rev: VecDeque<char> = VecDeque::with_capacity(sentence.len());
    let mut word_stack: Vec<char> = vec![];

    for letter in sentence.chars().rev() {
        if letter == separator {
            while !word_stack.is_empty() {
                word_order_rev.push_back(word_stack.pop().unwrap());
            }
            word_order_rev.push_back(letter);
        } else {
            word_stack.push(letter);
        }
    }

    while !word_stack.is_empty() {
        word_order_rev.push_back(word_stack.pop().unwrap());
    }

    return word_order_rev.into_iter().collect();
}

#[cfg(test)]
mod tests {
    use super::reverse as compute;

    #[test]
    fn test_with_simple_sentence() {
        let input = "i.like.this.program.very.much";
        let expected = "much.very.program.this.like.i";
        assert_eq!(expected, compute(input, '.'));
    }

    #[test]
    fn test_with_empty_sentence() {
        let input = "";
        let expected = "";
        assert_eq!(expected, compute(input, '.'));
    }

    #[test]
    fn test_with_empty_words_in_between_sentence() {
        let input = "There  are  extra  gaps  here";
        let expected = "here  gaps  extra  are  There";
        assert_eq!(expected, compute(input, ' '));
    }
}