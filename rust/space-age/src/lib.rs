#[derive(Debug)]
pub struct Duration {
    s: u64,
}
impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Duration { s }
    }
}
pub trait Planet {
    const ORBITAL_PERIOD: f64;
    const SECONDS_PER_EARTH_YEAR: u64 = 315360000;
    fn years_during(d: &Duration) -> f64 {
        d.s as f64 / Self::ORBITAL_PERIOD
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
    const ORBITAL_PERIOD: f64 = 7600543.81992;
}
impl Planet for Venus {
    const ORBITAL_PERIOD: f64 = 19414149.052176;
}
impl Planet for Earth {
    const ORBITAL_PERIOD: f64 = 31557600.0;
}
impl Planet for Mars {
    const ORBITAL_PERIOD: f64 = 59354032.690079994;
}
impl Planet for Jupiter {
    const ORBITAL_PERIOD: f64 = 374355659.124;
}
impl Planet for Saturn {
    const ORBITAL_PERIOD: f64 = 929292362.8848;
}
impl Planet for Uranus {
    const ORBITAL_PERIOD: f64 = 2651370019.3296;
}
impl Planet for Neptune {
    const ORBITAL_PERIOD: f64 = 5200418560.032001;
}
