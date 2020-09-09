
pub fn is_rotation_required(arr_size: &u32, rotation_count: &u32) -> bool {
    !((*arr_size == *rotation_count)
        || *arr_size == 0
        || *rotation_count == 0)
}

pub fn compute_min_rotation_count(arr_size: &u32, rotation_count: &u32) -> u32 {
    if *arr_size <= 1 || *rotation_count == 0 || *arr_size == *rotation_count {
        return 0;
    }

    if *rotation_count > *arr_size {
        return *rotation_count % *arr_size;
    }

    *rotation_count
}

pub fn reverse<T>(arr: &mut [T], start: usize, end: usize) {
    let mid_index = (end - start) / 2;
    for offset in 0..mid_index {
        let first = start + offset;
        let last = (end - 1) - offset;
        arr.swap(first, last);
    }
}

pub fn swap_blocks<T>(arr: &mut [T], a_offset: usize, b_offset: usize, count: usize) {
    for index in 0..count {
        let a_index = a_offset + index;
        let b_index = b_offset + index;
        arr.swap(a_index, b_index);
    }
}

pub fn find_pivot<T: Ord>(arr: &[T], arr_size: &usize, is_ascending: bool) {
 // TODO
}