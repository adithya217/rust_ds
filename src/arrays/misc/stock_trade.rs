
use std::vec::Vec;

pub fn compute_profitable_trades(arr: &[u16]) -> Vec<(usize, usize)> {
    let mut trades = vec![];

    if arr.len() < 2 {
        return trades;
    }

    let mut start_index = 0;
    let mut end_index = 0;
    let mut prev_diff = 0;

    for index in 1..arr.len() {
        let buy_value = arr[start_index] as i16;
        let sell_value = arr[index] as i16;
        let current_diff = sell_value - buy_value;

        if current_diff > prev_diff {
            end_index = index;
            prev_diff = current_diff;
        } else {
            if prev_diff > 0 {
                trades.push((start_index, end_index));
            }

            start_index = index;
            end_index = index;
            prev_diff = 0;
        }
    }

    if prev_diff > 0 {
        trades.push((start_index, end_index));
    }

    return trades;
}

#[cfg(test)]
mod tests {
    use super::compute_profitable_trades as compute;

    #[test]
    fn test_with_multiple_immediate_profits() {
        let input = [100, 180, 260, 310, 40, 535, 695];
        
        assert_eq!(vec![(0, 3), (4, 6)], compute(&input));
    }

    #[test]
    fn test_with_no_profits() {
        let input = [100, 50, 30, 20];
        let empty_profits: Vec<(usize, usize)> = vec![];
        
        assert_eq!(empty_profits, compute(&input));
    }

    #[test]
    fn test_with_multiple_non_immediate_profits() {
        let input = [23, 13, 25, 29, 33, 19, 34, 45, 65, 67];
        
        assert_eq!(vec![(1, 4), (5, 9)], compute(&input));
    }

    #[test]
    fn test_with_no_trades() {
        let input = [];
        let empty_profits: Vec<(usize, usize)> = vec![];
        
        assert_eq!(empty_profits, compute(&input));
    }

    #[test]
    fn test_with_one_trades() {
        let input = [100];
        let empty_profits: Vec<(usize, usize)> = vec![];
        
        assert_eq!(empty_profits, compute(&input));
    }

    #[test]
    fn test_with_no_profit_in_two_trades() {
        let input = [100, 50];
        let empty_profits: Vec<(usize, usize)> = vec![];
        
        assert_eq!(empty_profits, compute(&input));
    }

    #[test]
    fn test_with_profit_in_two_trades() {
        let input = [100, 200];
        let profits: Vec<(usize, usize)> = vec![(0, 1)];
        
        assert_eq!(profits, compute(&input));
    }

    #[test]
    fn test_with_one_profit_in_multiple_trades() {
        let input = [100, 200, 90, 80, 70, 60, 50];
        let profits: Vec<(usize, usize)> = vec![(0, 1)];
        
        assert_eq!(profits, compute(&input));
    }
}