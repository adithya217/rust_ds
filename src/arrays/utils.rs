
pub fn reverse_between_bounds_inclusive<T>(arr: &mut [T], start: usize, end: usize) {
    let mut first = start;
    let mut last = end;
    while first < last {
        arr.swap(first, last);
        first += 1;
        last -= 1;
    }
}