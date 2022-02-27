/// Return the Hamming distance between the strings,
/// or None if the lengths are mismatched.
pub fn hamming_distance(s1: &str, s2: &str) -> Option<usize> {
    let len = s1.len();
    let same_length = len == s2.len();
    match same_length {
        false => Option::None,
        true => {
            let diff = s1.bytes().zip(s2.bytes()).filter(|(x, y)| x != y).count();
            Some(diff)
        }
    }
}
