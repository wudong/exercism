pub fn is_armstrong_number(num: u32) -> bool {
    let nn = nums(num);
    let pow = nn.len();
    let sum: u32 = nn.iter().map(|&x|(x as u32).pow(pow as u32)).sum();
    sum == num
}

fn nums(num: u32)-> Vec<u8> {
    let mut vv =Vec::new();
    let mut divider = num;
    loop {
        let remainder = divider % 10;
        divider = divider / 10;
        vv.push(remainder as u8);

        if divider== 0 {
            break;
        }
    }
    vv.reverse();
    vv
}
