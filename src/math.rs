use super::traits::*;

pub fn clamp(x: f32, a: f32, b: f32) -> f32 {
    if x < a {
        a
    } else if x > b {
        b
    } else {
        x
    }
}

pub fn lerp<T>(a: T, b: T, t: T) -> T where T: Float {
    (T::one() - t) * a + t*b
}
/// Returns true if x and y are equal with an absolute error of e
pub fn equal_with_abs_error<T>(x: T, y: T, e: T) -> bool
where
    T: Scalar,
{
    let a = if x > y { x - y } else { y - x };
    a <= e
}

/// Returns true if x and y are equal with a relative error of e
pub fn equal_with_rel_error<T>(x: T, y: T, e: T) -> bool
where
    T: Real,
{
    let a = if x > y { x - y } else { y - x };
    let b = if x > T::zero() { x } else { -x };
    a <= e * b
}
