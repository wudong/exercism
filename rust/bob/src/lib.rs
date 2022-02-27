pub fn reply(message: &str) -> &str {
    let trimmed = message.trim();
    let mut revs = trimmed.chars().rev();

    let first = revs.next();
    match first {
        Some('?') => {
            let is_all_upper = is_all_upper(revs);
            if is_all_upper {
                "Calm down, I know what I'm doing!"
            } else {
                "Sure."
            }
        }
        Some(_) => {
            let is_all_upper = is_all_upper(revs);
            if is_all_upper {
                "Whoa, chill out!"
            } else {
                "Whatever."
            }
        }
        None => "Fine. Be that way!",
    }
}

fn is_all_upper(rev: impl Iterator<Item = char>) -> bool {
    let mut has_upper_case = false;
    let mut has_lower_case = false;
    rev.for_each(|x| {
        if x.is_lowercase() {
            has_lower_case = true;
        } else if x.is_uppercase() {
            has_upper_case = true;
        }
    });
    has_upper_case && !has_lower_case
}
