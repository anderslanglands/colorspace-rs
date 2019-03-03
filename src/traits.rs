pub use num_traits::{Bounded, Float, One, Zero};
pub(crate) use std::ops::{Add, Div, Mul, Neg, Sub};

pub trait Scalar:
    Copy
    + Zero
    + One
    + Bounded
    + Neg<Output = Self>
    + PartialOrd
    + PartialEq
    + Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
{
}

/// Further constrains what we need from a Scalar to cover just the real numbers
/// in order to be generic for functions like sqrt() that are not defined for
/// integers
pub trait Real: Scalar + Float {}

impl<T> Real for T where T: Scalar + Float {}

impl Scalar for f32 {}
