/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    let total = isbn
        .to_ascii_lowercase()
        .chars()
        .filter(|x| *x != '-')
        .enumerate()
        .fold(0, |t, (idx, cc)| {
            let num = get_num(cc);
            num * (idx as i32 + 1) + t
        });
    total % 11 == 0
}

fn get_num(cc: char) -> i32 {
    match cc {
        'x' => 10,
        y if y.is_digit(10) => y.to_digit(10).unwrap() as i32,
        _ => 0,
    }
}
