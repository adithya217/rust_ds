
pub fn compute_factorial(number: u8) -> u128 {
    return match number {
        0 | 1 => 1,
        2 => 2,
        _ => (2..=number as u128).fold(1, |acc, element| acc * element)
    };
}

#[cfg(test)]
mod tests {
    use super::compute_factorial as compute;

    #[test]
    fn test_positive_is_successful() {
        let test_data = [1, 2, 3, 4, 5, 6];
        let results = [1, 2, 6, 24, 120, 720];

        test_data.iter().zip(results.iter()).for_each(|(number, result)| {
            let expected = *result;
            let actual = compute(*number);
            assert_eq!(expected, actual);
        });
    }
}