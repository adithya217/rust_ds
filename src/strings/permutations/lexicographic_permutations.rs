
use std::vec::Vec;

pub fn compute(data: &str) -> Vec<String> {
    let mut letters: Vec<char> = data.chars().collect();

    let data_len = letters.len();
    let mut results = permute(&mut letters, 0, data_len);
    results.sort();

    return results;
}

fn permute(data: &mut Vec<char>, left: usize, right:usize) -> Vec<String> {
    if left >= right {
        return vec![data.iter().collect()];
    }

    return (left..right).flat_map(|index| {
        let rotation_count = index - left;

        data.swap(left, index);
        let permutation = permute(data, left+1, right);
        data.swap(left, index);

        return permutation;
    })
    .collect();
}

#[cfg(test)]
mod tests {
    use super::compute as compute;

    #[test]
    fn test_for_simple_permutations() {
        let input = "ABC";
        let expected = vec![
            String::from("ABC"), String::from("ACB"),
            String::from("BAC"), String::from("BCA"),
            String::from("CAB"), String::from("CBA")
        ];
        assert_eq!(expected, compute(input));
    }

    #[test]
    fn test_with_multi_letter_word() {
        let input = "SBAG";
        let expected = vec![
            String::from("ABGS"), String::from("ABSG"), String::from("AGBS"), String::from("AGSB"),
            String::from("ASBG"), String::from("ASGB"), String::from("BAGS"), String::from("BASG"),
            String::from("BGAS"), String::from("BGSA"), String::from("BSAG"), String::from("BSGA"),
            String::from("GABS"), String::from("GASB"), String::from("GBAS"), String::from("GBSA"),
            String::from("GSAB"), String::from("GSBA"), String::from("SABG"), String::from("SAGB"),
            String::from("SBAG"), String::from("SBGA"), String::from("SGAB"), String::from("SGBA"),
        ];
        assert_eq!(expected, compute(input));
    }
}