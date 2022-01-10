
pub fn nth(n: u32) -> u32 {
    // how to calc a prime number properly?
    let mut primes: Vec<u32> = vec![2,3,5,7,11,13];
    if (n as usize) < primes.len()-1 {
        return primes[n as usize];
    }

    for i in primes.len()..=n as usize {
        let vv = next_prime(&primes);
        println!("{}th prime: {}", i, vv);
        primes.push(vv);
    }

    return primes[n as usize];
}

pub fn next_prime(primes: &Vec<u32>)-> u32 { 
    let mut next = *primes.last().unwrap() + 1;
    loop {
        // checking from the last_prime.
        if is_prime(primes, next) {            
            return next;
        }
        next +=1;
    }
}

pub fn is_prime(primes: &Vec<u32>, v: u32) -> bool {     
    for i in primes {        
        if v % i ==0  {
            return false;
        }

        if *i > (v as f32).sqrt().round() as u32 {
            break;
        }
    }

    return true;
}


#[cfg(test)] 
mod test {

    use super::*;

    #[test]
    fn test_is_prime() {
        let primes: Vec<u32> = vec![2,3,5,7,11,13];
        assert_eq!(is_prime(&primes, 14), false);
        assert_eq!(is_prime(&primes, 15), false);
        assert_eq!(is_prime(&primes, 16), false);
        assert_eq!(is_prime(&primes, 17), true);
        assert_eq!(is_prime(&primes, 18), false);
        assert_eq!(is_prime(&primes, 19), true);
    }
}