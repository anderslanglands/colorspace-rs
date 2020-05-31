//! xyY coordinates and conversion to and from [XYZ]
use super::xyz::XYZ;
use std::convert::From;

use crate::math::Real;

pub type XYYf32 = XYY<f32>;
pub type XYYf64 = XYY<f64>;

/// Defines a pair of `xy` chromaticity coordinates
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct XYY<T>
where
    T: Real,
{
    pub x: T,
    pub y: T,
    pub Y: T,
}

pub fn xy<T>(x: T, y: T) -> XYY<T>
where
    T: Real,
{
    XYY::new(x, y, T::one())
}

impl<T> XYY<T>
where
    T: Real,
{
    #[allow(non_snake_case)]
    pub fn new(x: T, y: T, Y: T) -> XYY<T> {
        XYY::<T> { x, y, Y }
    }

    /// Convert the given XYZ tristimulus value to a chromaticity XYY
    /// value
    pub fn from_xyz(c: XYZ<T>) -> XYY<T> {
        let c = c / T::from(100.0).unwrap();
        XYY::<T> {
            x: (c.x / (c.x + c.y + c.z)),
            y: (c.y / (c.x + c.y + c.z)),
            Y: c.y,
        }
    }
}

impl<T> From<XYZ<T>> for XYY<T>
where
    T: Real,
{
    fn from(c: XYZ<T>) -> XYY<T> {
        XYY::<T>::from_xyz(c)
    }
}
