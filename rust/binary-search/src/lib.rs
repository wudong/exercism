use std::cmp::Ordering;

pub fn find(array: &[i32], key: i32) -> Option<usize> {
    find_with_start_index(array, key, 0)
}

fn find_with_start_index(array: &[i32], key: i32, start_index: usize) -> Option<usize> {
    let len = array.len();

    match len {
        0 => Option::None,
        1 => if array[0]==key {
            Option::Some(start_index)
        } else {
            Option::None
        },
        _ => {
            let middle_index = len/2;
            let middle_value = array[middle_index];
            let ordering = key.cmp(&middle_value);
            match ordering {
                Ordering::Less =>
                    find_with_start_index(&array[0..middle_index],
                                          key, start_index),
                Ordering::Equal =>
                    Option::Some(middle_index + start_index),
                Ordering::Greater =>
                    find_with_start_index(&array[middle_index+1..],
                                          key, start_index+ middle_index+1),
            }
        }
    }

}
