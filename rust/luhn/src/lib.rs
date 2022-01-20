/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let strs = remove_space(code);
    if let Some(str) = strs {
        let sum: u8 = str
            .iter()
            .rev()
            .enumerate()
            .map(|x| {
                if x.0 % 2 == 1 {
                    let double = x.1 * 2;
                    if double > 9 {
                        double - 9
                    } else {
                        double
                    }
                } else {
                    *x.1
                }
            })
            .sum();

        sum % 10 == 0
    } else {
        false
    }
}

pub fn remove_space(code: &str) -> Option<Vec<u8>> {
    let valid = code.as_bytes().iter().all(|x| *x==b' '|| (*x >= b'0' && *x <= b'9'));

    fn convert(code: &str) -> Vec<u8> {
        code
            .as_bytes()
            .into_iter()
            .filter(|&x| *x != b' ')
            .map(|b| *b - b'0')
            .collect()
    }
    
    match valid {
        true => {
            let u8s =convert(code);
            let len = u8s.len();
            match len {
                x if x<=1=> Option::None,
                _ => Option::Some(u8s),
            }
        }
        false => Option::None,
    }
}
