fn bottle_str(n: u32, cap: bool) -> String {
    let c = if cap { 'N' } else { 'n' };
    match n {
        0 => format!("{}o more bottles", c),
        1 => "1 bottle".to_string(),
        _ => format!("{} bottles", n),
    }
}

pub fn verse(n: u32) -> String {
    let str1 = bottle_str(n, true);
    let str2 = bottle_str(n, false);
    let part1 = format!("{} of beer on the wall, {} of beer.\n", str1, str2);

    let part2 = match n {
        0 => "Go to the store and buy some more, 99 bottles of beer on the wall.\n".to_string(),
        _ => format!(
            "Take {} down and pass it around, {} of beer on the wall.\n",
            if n == 1 { "it" } else { "one" },
            bottle_str(n - 1, false)
        ),
    };
    return part1 + &part2;
}

pub fn sing(start: u32, end: u32) -> String {
    let cc: Vec<String> = (end..start + 1).rev().map(|n| verse(n)).collect();
    return cc.join("\n");
}
