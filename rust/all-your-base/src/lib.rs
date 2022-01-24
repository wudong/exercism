use core::num;


#[derive(Debug, PartialEq)]
pub enum Error {
    InvalidInputBase,
    InvalidOutputBase,
    InvalidDigit(u32),
}

///
/// Convert a number between two bases.
///
/// A number is any slice of digits.
/// A digit is any unsigned integer (e.g. u8, u16, u32, u64, or usize).
/// Bases are specified as unsigned integers.
///
/// Return an `Err(.)` if the conversion is impossible.
/// The tests do not test for specific values inside the `Err(.)`.
///
///
/// You are allowed to change the function signature as long as all test still pass.
///
///
/// Example:
/// Input
///   number: &[4, 2]
///   from_base: 10
///   to_base: 2
/// Result
///   Ok(vec![1, 0, 1, 0, 1, 0])
///
/// The example corresponds to converting the number 42 from decimal
/// which is equivalent to 101010 in binary.
///
///
/// Notes:
///  * The empty slice ( "[]" ) is equal to the number 0.
///  * Never output leading 0 digits, unless the input number is 0, in which the output must be `[0]`.
///    However, your function must be able to process input with leading 0 digits.
///
pub fn convert(number: &[u32], from_base: u32, to_base: u32) -> Result<Vec<u32>, Error> {
    if from_base<=1 { 
        return Result::Err(Error::InvalidInputBase);
    }
    if to_base<=1 { 
        return Result::Err(Error::InvalidOutputBase);
    }

    let opt = number.into_iter().find(|&&x|x >=from_base);
    if opt.is_some() {
        return Result::Err(Error::InvalidDigit(*opt.unwrap()));
    }
    
    // it might be 
    let v: Option<u64> = convert_to_10_base(number, from_base);
    let vv = v.map(|f|convert_from_10_base(f, to_base));

    match vv {
        Some(vvv) =>  Result::Ok(vvv),
        None=> Result::Err(Error::InvalidDigit(to_base))
    }
    
}

fn convert_from_10_base(number: u64, to_base: u32) -> Vec<u32> {
    let mut result = Vec::new();
    let mut div = number;
    loop {
        let remainder = div % to_base as u64;
        div = div / to_base as u64;
        result.push(remainder as u32);
        if div==0 {
            break;
        }
    }
    result.reverse();
    result
}

// how to deal with overflow 
fn convert_to_10_base(number: &[u32], from_base: u32)-> Option<u64> {
    let (succ, fail): (Vec<Option<u64>>, Vec<Option<u64>>) = number.iter().rev().enumerate().
        map(|x| number_to_base_10(*x.1, from_base, x.0))
        .partition(|x|x.is_some());

    if !fail.is_empty() {
        return Option::None
    }else {
        let sum = succ.iter().map(|x|x.unwrap_or(0)).sum();
        Option::Some(sum)
    }
}

fn number_to_base_10(num: u32, from_base: u32, exp: usize) -> Option<u64>{
    (num as u64).checked_mul(from_base.pow(exp as u32) as u64)
}

