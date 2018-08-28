use super::traits::*;
use std::fmt;
use std::ops::{Index, IndexMut};
use super::math::*;

/// 3D generic vector
/// Implemented for f32, f64, i32
#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Default)]
pub struct XYZ<T>
where
    T: Scalar,
{
    pub x: T,
    pub y: T,
    pub z: T,
}

/// Type aliases
pub type XYZf32 = XYZ<f32>;

impl<T> XYZ<T>
where
    T: Scalar,
{
    pub fn new(x: T, y: T, z: T) -> XYZ<T> {
        XYZ::<T> { x, y, z }
    }

    pub fn from_scalar(a: T) -> XYZ<T> {
        XYZ::<T> { x: a, y: a, z: a }
    }

    /// Returns true if self and v are equal with error no greater than e
    pub fn equal_with_abs_error(self, v: XYZ<T>, e: T) -> bool
    where
        T: PartialOrd,
    {
        equal_with_abs_error(self.x, v.x, e)
            && equal_with_abs_error(self.y, v.y, e)
            && equal_with_abs_error(self.z, v.z, e)
    }

    /// Returns true if self and v are equal with error no greater than e
    pub fn equal_with_rel_error(self, v: XYZ<T>, e: T) -> bool
    where
        T: Real,
    {
        equal_with_rel_error(self.x, v.x, e)
            && equal_with_rel_error(self.y, v.y, e)
            && equal_with_rel_error(self.z, v.z, e)
    }

    /// Calculate the squared length of the vector
    pub fn length2(self) -> T {
        self.x * self.x + self.y * self.y + self.z * self.z
    }
}

impl<T> Zero for XYZ<T>
where
    T: Scalar,
{
    fn zero() -> XYZ<T>
    where
        T: Scalar,
    {
        XYZ::<T>::from_scalar(T::zero())
    }
    fn is_zero(&self) -> bool
    where
        T: Scalar,
    {
        self.x.is_zero() && self.y.is_zero() && self.z.is_zero()
    }
}

impl<T> One for XYZ<T>
where
    T: Scalar,
{
    fn one() -> XYZ<T>
    where
        T: Scalar,
    {
        XYZ::<T>::from_scalar(T::one())
    }
}

impl<T> Bounded for XYZ<T>
where
    T: Scalar,
{
    fn min_value() -> XYZ<T> {
        XYZ::<T> {
            x: T::min_value(),
            y: T::min_value(),
            z: T::min_value(),
        }
    }
    fn max_value() -> XYZ<T> {
        XYZ::<T> {
            x: T::max_value(),
            y: T::max_value(),
            z: T::max_value(),
        }
    }
}

impl<T> Index<usize> for XYZ<T>
where
    T: Scalar,
{
    type Output = T;

    fn index<'a>(&'a self, i: usize) -> &'a T {
        match i {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Tried to access XYZ with index of {}", i),
        }
    }
}

impl<T> IndexMut<usize> for XYZ<T>
where
    T: Scalar,
{
    fn index_mut<'a>(&'a mut self, i: usize) -> &'a mut T {
        match i {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("Tried to access XYZ with index of {}", i),
        }
    }
}

impl<T> fmt::Display for XYZ<T>
where
    T: Scalar + fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

/// Addition operator
impl<T> Add for XYZ<T>
where
    T: Scalar,
{
    type Output = XYZ<T>;

    fn add(self, rhs: XYZ<T>) -> XYZ<T> {
        XYZ::<T> {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

/// Subtraction operator
impl<T> Sub for XYZ<T>
where
    T: Scalar,
{
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
impl<T> Mul for XYZ<T>
where
    T: Scalar,
{
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
impl<T> Div for XYZ<T>
where
    T: Scalar,
{
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
impl<T> Neg for XYZ<T>
where
    T: Scalar,
{
    type Output = XYZ<T>;

    fn neg(self) -> XYZ<T> {
        XYZ::<T> {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

/// Multiplication by a T
impl<T> Mul<T> for XYZ<T>
where
    T: Scalar,
{
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
impl<T> Div<T> for XYZ<T>
where
    T: Scalar,
{
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
impl<T> Add<T> for XYZ<T>
where
    T: Scalar,
{
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
impl<T> Sub<T> for XYZ<T>
where
    T: Scalar,
{
    type Output = XYZ<T>;

    fn sub(self, rhs: T) -> XYZ<T> {
        XYZ::<T> {
            x: self.x - rhs,
            y: self.y - rhs,
            z: self.z - rhs,
        }
    }
}

/// Macro to implement right-side multiplication: T * XYZ<T>
macro_rules! vec3_impl_rhs_mul {
    ($($t:ty)*) => ($(
        impl Mul<XYZ<$t>> for $t {
            type Output = XYZ<$t>;
            fn mul(self, rhs: XYZ<$t>) -> XYZ<$t> {
                XYZ {
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
        impl Add<XYZ<$t>> for $t {
            type Output = XYZ<$t>;
            fn add(self, rhs: XYZ<$t>) -> XYZ<$t> {
                XYZ {
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
        impl Sub<XYZ<$t>> for $t {
            type Output = XYZ<$t>;
            fn sub(self, rhs: XYZ<$t>) -> XYZ<$t> {
                XYZ {
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
        impl Div<XYZ<$t>> for $t {
            type Output = XYZ<$t>;
            fn div(self, rhs: XYZ<$t>) -> XYZ<$t> {
                XYZ {
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
