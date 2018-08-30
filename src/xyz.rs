//! XYZ color type

use super::traits::*;
use std::fmt;
use std::ops::{Index, IndexMut};
use super::math::*;
use super::chromaticity::Chromaticity;
use std::convert::From;

/// XYZ color type
#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Default)]
pub struct XYZf<T>
where
    T: Scalar,
{
    pub x: T,
    pub y: T,
    pub z: T,
}

pub type XYZ = XYZf<f32>;

impl<T> XYZf<T>
where
    T: Scalar,
{
    pub fn new(x: T, y: T, z: T) -> XYZf<T> {
        XYZf::<T> { x, y, z }
    }

    pub fn from_scalar(a: T) -> XYZf<T> {
        XYZf::<T> { x: a, y: a, z: a }
    }

    /// Returns true if self and v are equal with error no greater than e
    pub fn equal_with_abs_error(self, v: XYZf<T>, e: T) -> bool
    where
        T: PartialOrd,
    {
        equal_with_abs_error(self.x, v.x, e)
            && equal_with_abs_error(self.y, v.y, e)
            && equal_with_abs_error(self.z, v.z, e)
    }

    /// Returns true if self and v are equal with error no greater than e
    pub fn equal_with_rel_error(self, v: XYZf<T>, e: T) -> bool
    where
        T: Real,
    {
        equal_with_rel_error(self.x, v.x, e)
            && equal_with_rel_error(self.y, v.y, e)
            && equal_with_rel_error(self.z, v.z, e)
    }

    pub fn normalized(&self) -> XYZf<T> {
        *self / self.y
    }
}

impl XYZ {
    pub fn from_chromaticity(c: Chromaticity, Y: f32) -> XYZ {
        XYZ { 
            x: c.x * Y / c.y,
            y: Y,
            z: (1.0 - c.x - c.y) * Y / c.y,
        }
    }
}

impl From<Chromaticity> for XYZ {
    fn from(c: Chromaticity) -> XYZ {
        XYZ::from_chromaticity(c, 1.0)
    }
}

impl<T> Zero for XYZf<T>
where
    T: Scalar,
{
    fn zero() -> XYZf<T>
    where
        T: Scalar,
    {
        XYZf::<T>::from_scalar(T::zero())
    }
    fn is_zero(&self) -> bool
    where
        T: Scalar,
    {
        self.x.is_zero() && self.y.is_zero() && self.z.is_zero()
    }
}

impl<T> One for XYZf<T>
where
    T: Scalar,
{
    fn one() -> XYZf<T>
    where
        T: Scalar,
    {
        XYZf::<T>::from_scalar(T::one())
    }
}

impl<T> Bounded for XYZf<T>
where
    T: Scalar,
{
    fn min_value() -> XYZf<T> {
        XYZf::<T> {
            x: T::min_value(),
            y: T::min_value(),
            z: T::min_value(),
        }
    }
    fn max_value() -> XYZf<T> {
        XYZf::<T> {
            x: T::max_value(),
            y: T::max_value(),
            z: T::max_value(),
        }
    }
}

impl<T> Index<usize> for XYZf<T>
where
    T: Scalar,
{
    type Output = T;

    fn index<'a>(&'a self, i: usize) -> &'a T {
        match i {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Tried to access XYZf with index of {}", i),
        }
    }
}

impl<T> IndexMut<usize> for XYZf<T>
where
    T: Scalar,
{
    fn index_mut<'a>(&'a mut self, i: usize) -> &'a mut T {
        match i {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("Tried to access XYZf with index of {}", i),
        }
    }
}

impl<T> fmt::Display for XYZf<T>
where
    T: Scalar + fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

/// Addition operator
impl<T> Add for XYZf<T>
where
    T: Scalar,
{
    type Output = XYZf<T>;

    fn add(self, rhs: XYZf<T>) -> XYZf<T> {
        XYZf::<T> {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

/// Subtraction operator
impl<T> Sub for XYZf<T>
where
    T: Scalar,
{
    type Output = XYZf<T>;

    fn sub(self, rhs: XYZf<T>) -> XYZf<T> {
        XYZf::<T> {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

/// Multiplication operator
impl<T> Mul for XYZf<T>
where
    T: Scalar,
{
    type Output = XYZf<T>;

    fn mul(self, rhs: XYZf<T>) -> XYZf<T> {
        XYZf::<T> {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

/// Division operator
impl<T> Div for XYZf<T>
where
    T: Scalar,
{
    type Output = XYZf<T>;

    fn div(self, rhs: XYZf<T>) -> XYZf<T> {
        XYZf::<T> {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.z,
        }
    }
}

/// Unary negation
impl<T> Neg for XYZf<T>
where
    T: Scalar,
{
    type Output = XYZf<T>;

    fn neg(self) -> XYZf<T> {
        XYZf::<T> {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

/// Multiplication by a T
impl<T> Mul<T> for XYZf<T>
where
    T: Scalar,
{
    type Output = XYZf<T>;

    fn mul(self, rhs: T) -> XYZf<T> {
        XYZf::<T> {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

/// Division by a T
impl<T> Div<T> for XYZf<T>
where
    T: Scalar,
{
    type Output = XYZf<T>;

    fn div(self, rhs: T) -> XYZf<T> {
        XYZf::<T> {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

/// Addition by a T
impl<T> Add<T> for XYZf<T>
where
    T: Scalar,
{
    type Output = XYZf<T>;

    fn add(self, rhs: T) -> XYZf<T> {
        XYZf::<T> {
            x: self.x + rhs,
            y: self.y + rhs,
            z: self.z + rhs,
        }
    }
}

/// Subtraction by a T
impl<T> Sub<T> for XYZf<T>
where
    T: Scalar,
{
    type Output = XYZf<T>;

    fn sub(self, rhs: T) -> XYZf<T> {
        XYZf::<T> {
            x: self.x - rhs,
            y: self.y - rhs,
            z: self.z - rhs,
        }
    }
}

/// Macro to implement right-side multiplication: T * XYZf<T>
macro_rules! vec3_impl_rhs_mul {
    ($($t:ty)*) => ($(
        impl Mul<XYZf<$t>> for $t {
            type Output = XYZf<$t>;
            fn mul(self, rhs: XYZf<$t>) -> XYZf<$t> {
                XYZf {
                    x: self * rhs.x,
                    y: self * rhs.y,
                    z: self * rhs.z,
                }
            }
        }
    )*)
}

vec3_impl_rhs_mul! {
    f32 
}

/// Macro to implement right-side addition: T + Vec2<T>
macro_rules! vec3_impl_rhs_add {
    ($($t:ty)*) => ($(
        impl Add<XYZf<$t>> for $t {
            type Output = XYZf<$t>;
            fn add(self, rhs: XYZf<$t>) -> XYZf<$t> {
                XYZf {
                    x: rhs.x + self,
                    y: rhs.y + self,
                    z: rhs.z + self,
                }
            }
        }
    )*)
}

vec3_impl_rhs_add! {
    f32
}

/// Macro to implement right-side subtraction: T - Vec2<T>
macro_rules! vec3_impl_rhs_sub {
    ($($t:ty)*) => ($(
        impl Sub<XYZf<$t>> for $t {
            type Output = XYZf<$t>;
            fn sub(self, rhs: XYZf<$t>) -> XYZf<$t> {
                XYZf {
                    x: self - rhs.x,
                    y: self - rhs.y,
                    z: self - rhs.z,
                }
            }
        }
    )*)
}

vec3_impl_rhs_sub! {
    f32 
}

/// Macro to implement right-side division: T / Vec2<T>
macro_rules! vec3_impl_rhs_div {
    ($($t:ty)*) => ($(
        impl Div<XYZf<$t>> for $t {
            type Output = XYZf<$t>;
            fn div(self, rhs: XYZf<$t>) -> XYZf<$t> {
                XYZf {
                    x: self / rhs.x,
                    y: self / rhs.y,
                    z: self / rhs.z,
                }
            }
        }
    )*)
}

vec3_impl_rhs_div! {
    f32
}
