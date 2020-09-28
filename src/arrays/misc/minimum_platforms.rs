
use std::vec::Vec;

pub fn find_count(arrivals: &[u32], departures: &[u32]) -> Result<usize, String> {
    if arrivals.len() != departures.len() {
        return Err(String::from("Arrivals and Departures count must be the same!"));
    }

    if arrivals.is_empty() {
        return Ok(0);
    }

    let mut platforms: Vec<u32> = vec![departures[0]];

    for arrival_index in 1..arrivals.len() {
        let arrival = arrivals[arrival_index];
        let departure = departures[arrival_index];
        let mut is_platform_assigned = false;

        for platform_index in 0..platforms.len() {
            if platforms[platform_index] < arrival {
                platforms[platform_index] = departure;
                is_platform_assigned = true;
                break;
            }
        }

        if !is_platform_assigned {
            platforms.push(departure);
        }
    }

    return Ok(platforms.len());
}

#[cfg(test)]
mod tests {
    use super::find_count as compute;

    #[test]
    fn test_with_multiple_different_arrivals_results_in_multiple_platforms() {
        let arrivals = [0900, 0940, 0950, 1100, 1500, 1800];
        let departures = [0910, 1200, 1120, 1130, 1900, 2000];

        assert_eq!(Ok(3), compute(&arrivals, &departures));
    }

    #[test]
    fn test_with_multiple_different_arrivals_results_in_single_platform() {
        let arrivals = [0900, 1100, 1235];
        let departures = [1000, 1200, 1240];

        assert_eq!(Ok(1), compute(&arrivals, &departures));
    }

    #[test]
    fn test_with_multiple_same_arrivals_results_in_different_platforms() {
        let arrivals = [0900, 0900, 0900];
        let departures = [1000, 1000, 1000];

        assert_eq!(Ok(3), compute(&arrivals, &departures));
    }

    #[test]
    fn test_with_no_arrivals_results_in_no_platforms() {
        let arrivals = [];
        let departures = [];

        assert_eq!(Ok(0), compute(&arrivals, &departures));
    }

    #[test]
    fn test_with_different_count_of_arrivals_departures_results_in_error() {
        let arrivals = [0900, 1000];
        let departures = [0930];
        let expected = Err(String::from("Arrivals and Departures count must be the same!"));

        assert_eq!(expected, compute(&arrivals, &departures));
    }
}