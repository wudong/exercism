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
    const YEAR_IN_EARTH_YEAR: f64;

    fn years_during(d: &Duration) -> f64 {
        d.in_earth_year() / Self::YEAR_IN_EARTH_YEAR
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

macro_rules! impl_planet {
   ($t:ident, $e:expr) => {
    impl Planet for $t {
        const YEAR_IN_EARTH_YEAR:f64 = $e;
    }
   }
}

impl_planet!(Mercury, 0.2408467);
impl_planet!(Venus, 0.61519726);
impl_planet!(Earth, 1_f64);
impl_planet!(Mars, 1.8808158_f64);
impl_planet!(Jupiter, 11.862615_f64);
impl_planet!(Saturn, 29.447498_f64);
impl_planet!(Uranus, 84.016846_f64);
impl_planet!(Neptune, 164.79132_f64);
