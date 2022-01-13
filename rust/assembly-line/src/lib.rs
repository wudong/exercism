// This stub file contains items which aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

use std::ops::Div;

const RATE: u32 = 221;

pub fn production_rate_per_hour(speed: u8) -> f64 {    
    match speed {
        0 => 0_f64,
        x@ 1..=4 =>  (x as u32* RATE) as f64,
        x @ 5..=8 =>  (x as u32* RATE) as f64 * 0.9,
        x @ 9..=10 =>  (x as u32* RATE) as f64 * 0.77,
        _ => panic!("invalid input")
    }
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    let hourly_rate = production_rate_per_hour(speed);
    let minute_rate = (hourly_rate / 60_f64);
    minute_rate.trunc() as u32    
}
