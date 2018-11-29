pub mod matrix;
pub mod vector;
pub mod quaternion;

pub fn clamp<T: PartialOrd>(v: T, min: T, max: T) -> T {
    if v < min {
        return min;
    } else if v > max {
        return max;
    } else {
        return v;
    }
}