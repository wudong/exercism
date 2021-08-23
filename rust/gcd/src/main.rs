use std::env;
use std::str::FromStr;

fn main() {
    let args = env::args();
    let mut vec: Vec<u64> = Vec::new();
    for arg in args.skip(1) {
        vec.push(u64::from_str(&arg).expect("error parse argument"));
    }
    if vec.len() == 0 {
        eprintln!("No argument provided");
        std::process::exit(1);
    }
    let mut d = vec[0];
    for number in &vec[1..] {
        d = gcd(d, *number)
    }
    println!("The greatest common divisor of {:?} is {}", vec, d);
}

fn gcd(mut m: u64, mut n: u64) -> u64 {
    assert!(m != 0 && n != 0);
    while m != 0 {
        if m < n {
            //swap
            let t = m;
            m = n;
            n = t;
        }
        m = m % n
    }
    return n;
}
