
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
        return *rotation_count - *arr_size;
    }

    *rotation_count
}