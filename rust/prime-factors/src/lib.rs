pub fn factors(n: u64) -> Vec<u64> {

    let primes = nth(80000);    // how can i cache this? 
    
    if n==1 {
        return vec![];
    } else if is_prime(&primes, n) {
        return vec![n];
    }

    let mut result = vec![];
    let mut ss = n;
    for prime in primes {
        if prime > ss {
            break;
        }

        let (ss1, mut factors) = factor_with(ss, prime);
        ss = ss1;

        if factors.len() > 0 {
            result.append(&mut factors);
        }
    }

    result
}

pub fn factor_with(n: u64, prime: u64)-> (u64, Vec<u64>) {
    let mut ss =  n;    
    let mut result =  vec![];
    while ss % prime == 0 {
        ss = ss / prime;
        result.push(prime);
    }    
    (ss, result)
}

pub fn nth(n: u64) -> Vec<u64> {
    // how to calc a prime number properly?
    let mut primes: Vec<u64> = vec![2,3,5,7,11,13];
    if (n as usize) < primes.len()-1 {
        return primes;
    }

    for _ in primes.len()..=n as usize {
        let vv = next_prime(&primes);        
        primes.push(vv);
    }

    return primes;
}

pub fn next_prime(primes: &Vec<u64>)-> u64 { 
    let mut next = *primes.last().unwrap() + 1;
    loop {
        // checking from the last_prime.
        if is_prime(primes, next) {            
            return next;
        }
        next +=1;
    }
}

pub fn is_prime(primes: &Vec<u64>, v: u64) -> bool {     
    for i in primes {        
        if v % i ==0  {
            return false;
        }

        if *i > (v as f32).sqrt().round() as u64 {
            break;
        }
    }

    return true;
}
