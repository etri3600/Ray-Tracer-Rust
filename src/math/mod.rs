pub mod matrix;
pub mod vector;
pub mod quaternion;

pub fn clamp<T: PartialOrd>(v: T, min: T, max: T) -> T {
    if v < min {
        min
    } else if v > max {
        max
    } else {
        v
    }
}

pub trait Math {
    fn is_nearly_zero(&self) -> bool;
}

impl Math for f64 {
    fn is_nearly_zero(&self) -> bool {
        -std::f64::EPSILON < *self || *self < std::f64::EPSILON
    }
}

impl Math for f32 {
    fn is_nearly_zero(&self) -> bool {
        -std::f32::EPSILON < *self || *self < std::f32::EPSILON
    }
}