pub fn square(s: u32) -> u64 {
    if s == 0 || s > 64 {
        panic!("Square must be between 1 and 64")
    }
    (0..s - 1).fold(1, |init, _| init as u64 * 2)
}

pub fn total() -> u64 {
    (1..=64).fold(0, |sum, x| sum + square(x))
}
