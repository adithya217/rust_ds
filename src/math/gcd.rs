use std::mem::swap;

pub fn compute_euclidean_gcd(a: &u32, b: &u32) -> u32 {
    let a = *a;
    let b = *b;

    if a == 0 || b == 0 {
        return 0;
    }

    let (mut bigger, mut smaller) = if a >= b { (a, b) } else { (b, a) };

    while smaller != 0 {
        bigger = bigger % smaller;
        swap(&mut bigger, &mut smaller);
    }

    return bigger;
}

#[cfg(test)]
mod test {
    use super::compute_euclidean_gcd as compute_gcd;

    #[test]
    fn gcd_of_zeros() {
        let a = 0;
        let b = 0;
        assert_eq!(compute_gcd(&a, &b), 0);
    }

    #[test]
    fn gcd_of_even_numbers() {
        let a = 48;
        let b = 18;
        assert_eq!(compute_gcd(&a, &b), 6);
    }

    #[test]
    fn gcd_of_odd_numbers() {
        let a = 9;
        let b = 7;
        assert_eq!(compute_gcd(&a, &b), 1);
    }

    #[test]
    fn gcd_of_odd_even_mixed_numbers() {
        let a = 9;
        let b = 16;
        assert_eq!(compute_gcd(&a, &b), 1);
    }
}