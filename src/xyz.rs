//! XYZ color type

use super::chromaticity::xyY;
use super::math::*;
use super::traits::*;
use std::convert::From;
use std::fmt;
use std::ops::{Index, IndexMut};

/// XYZ color type
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Default)]
pub struct XYZ {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl XYZ {
    pub fn new(x: f32, y: f32, z: f32) -> XYZ {
        XYZ { x, y, z }
    }

    pub fn from_scalar(a: f32) -> XYZ {
        XYZ { x: a, y: a, z: a }
    }

    /// Returns true if self and v are equal with error no greater than e
    pub fn equal_with_abs_error(self, v: XYZ, e: f32) -> bool {
        equal_with_abs_error(self.x, v.x, e)
            && equal_with_abs_error(self.y, v.y, e)
            && equal_with_abs_error(self.z, v.z, e)
    }

    /// Returns true if self and v are equal with error no greater than e
    pub fn equal_with_rel_error(self, v: XYZ, e: f32) -> bool {
        equal_with_rel_error(self.x, v.x, e)
            && equal_with_rel_error(self.y, v.y, e)
            && equal_with_rel_error(self.z, v.z, e)
    }

    /// Returns a unit-luminance version of this color.
    pub fn normalized(&self) -> XYZ {
        *self / self.y
    }
}

impl XYZ {
    /// Creates a new XYZ from the given `xyY` coordinates
    #[allow(non_snake_case)]
    pub fn from_chromaticity(c: xyY) -> XYZ {
        XYZ {
            x: c.x * c.Y / c.y,
            y: c.Y,
            z: (1.0 - c.x - c.y) * c.Y / c.y,
        }
    }
}

impl From<xyY> for XYZ {
    fn from(c: xyY) -> XYZ {
        XYZ::from_chromaticity(c)
    }
}

impl Zero for XYZ {
    fn zero() -> XYZ {
        XYZ::from_scalar(0.0)
    }

    fn is_zero(&self) -> bool {
        self.x.is_zero() && self.y.is_zero() && self.z.is_zero()
    }
}

impl One for XYZ {
    fn one() -> XYZ {
        XYZ::from_scalar(1.0)
    }
}

impl Bounded for XYZ {
    fn min_value() -> XYZ {
        XYZ {
            x: std::f32::MIN,
            y: std::f32::MIN,
            z: std::f32::MIN,
        }
    }
    fn max_value() -> XYZ {
        XYZ {
            x: std::f32::MAX,
            y: std::f32::MAX,
            z: std::f32::MAX,
        }
    }
}

impl Index<usize> for XYZ {
    type Output = f32;

    fn index(&self, i: usize) -> &f32 {
        match i {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Tried to access XYZf with index of {}", i),
        }
    }
}

impl IndexMut<usize> for XYZ {
    fn index_mut(&mut self, i: usize) -> &mut f32 {
        match i {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("Tried to access XYZf with index of {}", i),
        }
    }
}

impl fmt::Display for XYZ {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

/// Addition operator
impl Add for XYZ {
    type Output = XYZ;

    fn add(self, rhs: XYZ) -> XYZ {
        XYZ {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

/// Subtraction operator
impl Sub for XYZ {
    type Output = XYZ;

    fn sub(self, rhs: XYZ) -> XYZ {
        XYZ {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

/// Multiplication operator
impl Mul for XYZ {
    type Output = XYZ;

    fn mul(self, rhs: XYZ) -> XYZ {
        XYZ {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

/// Division operator
impl Div for XYZ {
    type Output = XYZ;

    fn div(self, rhs: XYZ) -> XYZ {
        XYZ {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.z,
        }
    }
}

/// Unary negation
impl Neg for XYZ {
    type Output = XYZ;

    fn neg(self) -> XYZ {
        XYZ {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

/// Multiplication by a f32
impl Mul<f32> for XYZ {
    type Output = XYZ;

    fn mul(self, rhs: f32) -> XYZ {
        XYZ {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

/// Division by a f32
impl Div<f32> for XYZ {
    type Output = XYZ;

    fn div(self, rhs: f32) -> XYZ {
        XYZ {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

/// Addition by a f32
impl Add<f32> for XYZ {
    type Output = XYZ;

    fn add(self, rhs: f32) -> XYZ {
        XYZ {
            x: self.x + rhs,
            y: self.y + rhs,
            z: self.z + rhs,
        }
    }
}

/// Subtraction by a f32
impl Sub<f32> for XYZ {
    type Output = XYZ;

    fn sub(self, rhs: f32) -> XYZ {
        XYZ {
            x: self.x - rhs,
            y: self.y - rhs,
            z: self.z - rhs,
        }
    }
}

impl Mul<XYZ> for f32 {
    type Output = XYZ;
    fn mul(self, rhs: XYZ) -> XYZ {
        XYZ {
            x: self * rhs.x,
            y: self * rhs.y,
            z: self * rhs.z,
        }
    }
}

impl Add<XYZ> for f32 {
    type Output = XYZ;
    fn add(self, rhs: XYZ) -> XYZ {
        XYZ {
            x: self + rhs.x,
            y: self + rhs.y,
            z: self + rhs.z,
        }
    }
}

impl Div<XYZ> for f32 {
    type Output = XYZ;
    fn div(self, rhs: XYZ) -> XYZ {
        XYZ {
            x: self / rhs.x,
            y: self / rhs.y,
            z: self / rhs.z,
        }
    }
}

impl Sub<XYZ> for f32 {
    type Output = XYZ;
    fn sub(self, rhs: XYZ) -> XYZ {
        XYZ {
            x: self - rhs.x,
            y: self - rhs.y,
            z: self - rhs.z,
        }
    }
}
