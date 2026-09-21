pub use std::f64::consts::PI;

#[allow(dead_code)]
#[inline]
pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}

#[inline]
pub fn random_double() -> f64 {
    // gives random real in [0, 1)
    fastrand::f64()
}

#[inline]
pub fn random_double_range(min: f64, max: f64) -> f64 {
    min + (max - min) * fastrand::f64()
}

#[inline]
pub fn random_usize_range(min: usize, max: usize) -> usize {
    fastrand::usize(min..=max)
}

#[inline]
pub fn random_byte_range(min: u8, max: u8) -> u8 {
    fastrand::u8(min..=max)
}
