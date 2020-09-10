use super::factorial::compute_factorial as factorial;

pub fn n_c_r(n: u128, r: u8) -> u128 {
    let r = r as u128;

    if r > n {
        return 0;
    }

    if r == 0 || n == r {
        return 1;
    }

    if r == 1 || r == n - 1 {
        return n;
    }

    // ncr = ncn-r; doesn't matter if we optimize denominator here because numerator will increase accordingly.

    let numerator = ((n-r+1)..=n).fold(1, |acc, number| acc * number);
    let denominator = factorial(r as u8);

    return numerator / denominator;
}

#[cfg(test)]
mod tests {
    use super::n_c_r as compute;

    #[test]
    fn test_nc0_is_1() {
        assert_eq!(1, compute(10, 0));
    }

    #[test]
    fn test_ncn_is_1() {
        assert_eq!(1, compute(10, 10));
    }

    #[test]
    fn test_0c0_is_1() {
        assert_eq!(1, compute(0, 0));
    }

    #[test]
    fn test_nc1_is_n() {
        assert_eq!(10, compute(10, 1));
    }

    #[test]
    fn test_ncnminus1_is_n() {
        assert_eq!(10, compute(10, 9));
    }

    #[test]
    fn test_ncr_is_correct() {
        assert_eq!(210, compute(10, 4));
    }

    #[test]
    fn test_1c2_is_0() {
        assert_eq!(0, compute(1, 2));
    }
}