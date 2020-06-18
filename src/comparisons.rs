use std::cmp::Ordering;

pub fn compare_slices<T: Ord>(a: &[T], b: &[T]) -> Ordering {
    a.iter()
        .zip(b)
        .map(|(x, y)| x.cmp(y))
        .find(|&ord| ord != Ordering::Equal)
        .unwrap_or(a.len().cmp(&b.len()))
}

#[cfg(test)]
mod tests {
    use std::cmp::Ordering;
    use super::compare_slices;

    #[test]
    fn compare_slices_of_numbers() {
        let first_slice = [1,2,3];
        let second_slice = [1,2,3];
        assert_eq!(Ordering::Equal, compare_slices(&first_slice, &second_slice))
    }

    #[test]
    fn compare_slices_of_characters() {
        let first_slice = ['a','b','c'];
        let second_slice = ['a','b','c'];
        assert_eq!(Ordering::Equal, compare_slices(&first_slice, &second_slice))
    }
}