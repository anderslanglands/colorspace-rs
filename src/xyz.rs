//! XYZ color type

use super::chromaticity::XYY;
use std::convert::From;
use std::fmt;
use std::ops::{AddAssign, Index, IndexMut, Add, Sub, Mul, Div, Neg};
use num_traits::{Bounded, One, Zero};
use float_cmp::{F32Margin, F64Margin, ApproxEq};

use crate::math::Real;

pub type XYZf32 = XYZ<f32>;
pub type XYZf64 = XYZ<f64>;

/// XYZ color type
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Default)]
pub struct XYZ<T> where T: Real {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T> XYZ<T>  where T: Real {
    pub fn new(x: T, y: T, z: T) -> XYZ<T> {
        XYZ::<T> { x, y, z }
    }

    pub fn from_scalar(a: T) -> XYZ<T> {
        XYZ::<T> { x: a, y: a, z: a }
    }

    /// Returns a unit-luminance version of this color.
    pub fn normalized(&self) -> XYZ<T>  {
        *self / Self::from_scalar(self.y)
    }

    pub fn abs(&self) -> XYZ<T> {
        XYZ::<T> {
            x: self.x.abs(),
            y: self.y.abs(),
            z: self.z.abs(),
        }
    }
}

pub fn xyz<T>(x: T, y: T, z: T) -> XYZ<T> where T: Real {
    XYZ::new(x, y, z)
}

impl<T> XYZ<T> where T: Real + One {
    /// Creates a new XYZ from the given `xyY` coordinates
    #[allow(non_snake_case)]
    pub fn from_chromaticity(c: XYY<T>) -> XYZ<T> {
        XYZ::<T> {
            x: c.x * c.Y / c.y,
            y: c.Y,
            z: (T::one() - c.x - c.y) * c.Y / c.y,
        } * T::from(100.0).unwrap()
    }

    pub fn from_xy(x: T, y: T) -> XYZ<T> {
        Self::from_chromaticity(XYY::new(x, y, T::one()))  
    }

    pub fn normalized_y(&self) -> XYZ<T> {
        (*self) / self.y * T::from(100.0).unwrap()
    }

}

impl<T> From<XYY<T>> for XYZ<T> where T: Real {
    fn from(c: XYY<T>) -> XYZ<T> {
        XYZ::from_chromaticity(c)
    }
}

impl<T> Zero for XYZ<T> where T: Real {
    fn zero() -> XYZ<T> {
        XYZ::from_scalar(T::zero())
    }

    fn is_zero(&self) -> bool {
        self.x.is_zero() && self.y.is_zero() && self.z.is_zero()
    }
}

impl<T> One for XYZ<T> where T: Real {
    fn one() -> XYZ<T> {
        XYZ::from_scalar(T::one())
    }
}

impl<T> Bounded for XYZ<T> where T: Real {
    fn min_value() -> XYZ<T> {
        XYZ::<T> {
            x: Bounded::min_value(),
            y: Bounded::min_value(),
            z: Bounded::min_value(),
        }
    }
    fn max_value() -> XYZ<T> {
        XYZ::<T> {
            x: Bounded::max_value(),
            y: Bounded::max_value(),
            z: Bounded::max_value(),
        }
    }
}

impl<T> Index<usize> for XYZ<T> where T: Real {
    type Output = T;

    fn index(&self, i: usize) -> &T {
        match i {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Tried to access XYZ with index of {}", i),
        }
    }
}

impl<T> IndexMut<usize> for XYZ<T> where T: Real {
    fn index_mut(&mut self, i: usize) -> &mut T {
        match i {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("Tried to access XYZ with index of {}", i),
        }
    }
}

impl<T> fmt::Display for XYZ<T> where T: Real {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}


impl ApproxEq for XYZf32 {
    type Margin = F32Margin;
    fn approx_eq<T: Into<Self::Margin>>(self, other: Self, margin: T) -> bool {
        let margin = margin.into();
        self.x.approx_eq(other.x, margin) 
        && self.y.approx_eq(other.y, margin)
        && self.z.approx_eq(other.z, margin)
    }
}

impl ApproxEq for XYZf64 {
    type Margin = F64Margin;
    fn approx_eq<T: Into<Self::Margin>>(self, other: Self, margin: T) -> bool {
        let margin = margin.into();
        self.x.approx_eq(other.x, margin) 
        && self.y.approx_eq(other.y, margin)
        && self.z.approx_eq(other.z, margin)
    }
}

impl From<XYZf64> for XYZf32 {
    fn from(x: XYZf64) -> XYZf32 {
        XYZ {
            x: x.x as f32,
            y: x.y as f32,
            z: x.z as f32,
        }
    }
}

impl std::iter::Sum for XYZf32 {
    fn sum<I>(iter: I) -> XYZf32 where I: Iterator<Item=XYZf32> {
        let mut xyz = XYZf32::from_scalar(0.0);
        for i in iter {
            xyz += i;
        }

        xyz
    }
}

/// Addition operator
impl<T> Add for XYZ<T> where T: Real {
    type Output = XYZ<T>;

    fn add(self, rhs: XYZ<T>) -> XYZ<T> {
        XYZ::<T> {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl<T> AddAssign for XYZ<T> where T: Real {
    fn add_assign(&mut self, rhs: XYZ<T>) {
        *self = XYZ::<T> {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

/// Subtraction operator
impl<T> Sub for XYZ<T> where T: Real {
    type Output = XYZ<T>;

    fn sub(self, rhs: XYZ<T>) -> XYZ<T> {
        XYZ::<T> {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

/// Multiplication operator
impl<T> Mul for XYZ<T> where T: Real {
    type Output = XYZ<T>;

    fn mul(self, rhs: XYZ<T>) -> XYZ<T> {
        XYZ::<T> {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

/// Division operator
impl<T> Div for XYZ<T> where T: Real {
    type Output = XYZ<T>;

    fn div(self, rhs: XYZ<T>) -> XYZ<T> {
        XYZ::<T> {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.z,
        }
    }
}

/// Unary negation
impl<T> Neg for XYZ<T> where T: Real {
    type Output = XYZ<T>;

    fn neg(self) -> XYZ<T> {
        XYZ::<T> {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

/// Multiplication by a f32
impl<T> Mul<T> for XYZ<T> where T:Real {
    type Output = XYZ<T>;

    fn mul(self, rhs: T) -> XYZ<T> {
        XYZ::<T> {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

/// Division by a T
impl<T> Div<T> for XYZ<T> where T: Real {
    type Output = XYZ<T>;

    fn div(self, rhs: T) -> XYZ<T> {
        XYZ::<T> {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

/// Addition by a T
impl<T> Add<T> for XYZ<T> where T: Real {
    type Output = XYZ<T>;

    fn add(self, rhs: T) -> XYZ<T> {
        XYZ::<T> {
            x: self.x + rhs,
            y: self.y + rhs,
            z: self.z + rhs,
        }
    }
}

/// Subtraction by a T
impl<T> Sub<T> for XYZ<T> where T: Real {
    type Output = XYZ<T>;

    fn sub(self, rhs: T) -> XYZ<T> {
        XYZ::<T> {
            x: self.x - rhs,
            y: self.y - rhs,
            z: self.z - rhs,
        }
    }
}