// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(Debug)]
pub struct Duration(u64);

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Self(s)
    }
}

impl Duration {
    fn in_earth_year(&self)-> f64 {
        self.0 as f64 / 31557600_f64
    }
}

pub trait Planet {
    fn year_in_earth_year ()-> f64;

    fn years_during(d: &Duration) -> f64 {
        d.in_earth_year() / Self::year_in_earth_year()
    }
}

pub struct Mercury;
pub struct Venus;
pub struct Earth;
pub struct Mars;
pub struct Jupiter;
pub struct Saturn;
pub struct Uranus;
pub struct Neptune;

impl Planet for Mercury {
    fn year_in_earth_year() -> f64 {
        0.2408467
    }
}
impl Planet for Venus {
    fn year_in_earth_year() -> f64 {
        0.61519726
    }
}
impl Planet for Earth {
    fn year_in_earth_year() -> f64 {
        1_f64
    }
}
impl Planet for Mars {
    fn year_in_earth_year() -> f64 {
        1.8808158_f64
    }
}
impl Planet for Jupiter {
    fn year_in_earth_year() -> f64 {
        11.862615_f64
    }
}
impl Planet for Saturn {
    fn year_in_earth_year() -> f64 {
        29.447498_f64
    }
}
impl Planet for Uranus {
    fn year_in_earth_year() -> f64 {
        84.016846_f64
    }
}
impl Planet for Neptune {
    fn year_in_earth_year() -> f64 {
        164.79132_f64
    }
}
