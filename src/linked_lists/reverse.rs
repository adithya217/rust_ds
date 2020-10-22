

#[cfg(test)]
mod tests {
    use super::super::list::List;
    use std::vec::Vec;

    #[test]
    fn test_reversal_of_empty_list() {
        let mut list = List::new();

        list.reverse();

        let expected: Vec<u16> = Vec::with_capacity(0);
        let mut result: Vec<u16> = Vec::with_capacity(expected.len());
        (0..expected.len()).into_iter().for_each(|_| if let Some(&number) = list.pop_back() { result.push(number); });

        assert_eq!(expected, result);
        assert_eq!(0, list.len());
    }

    #[test]
    fn test_reversal_of_single_element_list() {
        let mut list = List::new();
        let input: Vec<u16> = vec![1];
        input.iter().for_each(|number| list.append(number));

        list.reverse();

        let expected: Vec<u16> = vec![1];
        let mut result: Vec<u16> = Vec::with_capacity(expected.len());
        (0..expected.len()).into_iter().for_each(|_| if let Some(&number) = list.pop_back() { result.push(number); });

        assert_eq!(expected, result);
        assert_eq!(0, list.len());
    }

    #[test]
    fn test_reversal_of_odd_sized_list() {
        let mut list = List::new();
        let input: Vec<u16> = vec![1, 2, 3, 4, 5];
        input.iter().for_each(|number| list.append(number));

        list.reverse();

        let expected: Vec<u16> = vec![5, 4, 3, 2, 1];
        let mut result: Vec<u16> = Vec::with_capacity(expected.len());
        (0..expected.len()).into_iter().for_each(|_| if let Some(&number) = list.pop_front() { result.push(number); });

        assert_eq!(expected, result);
        assert_eq!(0, list.len());
    }

    #[test]
    fn test_reversal_of_even_sized_list() {
        let mut list = List::new();
        let input: Vec<u16> = vec![1, 2, 3, 4, 5, 6, 7, 8];
        input.iter().for_each(|number| list.append(number));

        list.reverse();

        let expected: Vec<u16> = vec![8, 7, 6, 5, 4, 3, 2, 1];
        let mut result: Vec<u16> = Vec::with_capacity(expected.len());
        (0..expected.len()).into_iter().for_each(|_| if let Some(&number) = list.pop_front() { result.push(number); });

        assert_eq!(expected, result);
        assert_eq!(0, list.len());
    }
}