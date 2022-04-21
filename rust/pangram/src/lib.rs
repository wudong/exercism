/// Determine whether a sentence is a pangram.
pub fn is_pangram(sentence: &str) -> bool {
    let mut letters = [false; 26];
    for c in sentence.to_lowercase().bytes() {
        if 'a' <= c as char && c as char <= 'z' {
            letters[c as usize - 'a' as usize] = true;
        }
    }
    letters.iter().all(|&x| x)
}
