//! RGB color types

use super::math::*;
pub use super::traits::*;
use std::convert::From;
use std::fmt;
use std::ops::{Index, IndexMut};

#[cfg(feature = "f16")]
use half::f16;

/// Floating-point RGB type
#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Default)]
pub struct RGBf<T> {
    pub r: T,
    pub g: T,
    pub b: T,
}

impl<T> RGBf<T>
where
    T: Real,
{
    pub fn new(r: T, g: T, b: T) -> RGBf<T> {
        RGBf::<T> { r, g, b }
    }

    pub fn from_scalar(s: T) -> RGBf<T> {
        RGBf::<T> { r: s, g: s, b: s }
    }

    pub fn powf(&self, x: T) -> RGBf<T> {
        RGBf::<T> {
            r: self.r.powf(x),
            g: self.r.powf(x),
            b: self.r.powf(x),
        }
    }
}

pub type RGBf32 = RGBf<f32>;

#[inline]
pub fn rgbf32(r: f32, g: f32, b: f32) -> RGBf32 {
    RGBf32::new(r, g, b)
}

impl<T> Zero for RGBf<T>
where
    T: Real,
{
    fn zero() -> RGBf<T>
    where
        T: Real,
    {
        RGBf::<T>::from_scalar(T::zero())
    }
    fn is_zero(&self) -> bool
    where
        T: Scalar,
    {
        self.r.is_zero() && self.g.is_zero() && self.b.is_zero()
    }
}

impl<T> One for RGBf<T>
where
    T: Real,
{
    fn one() -> RGBf<T>
    where
        T: Real,
    {
        RGBf::<T>::from_scalar(T::one())
    }
}

impl<T> Bounded for RGBf<T>
where
    T: Scalar,
{
    fn min_value() -> RGBf<T> {
        RGBf::<T> {
            r: T::min_value(),
            g: T::min_value(),
            b: T::min_value(),
        }
    }
    fn max_value() -> RGBf<T> {
        RGBf::<T> {
            r: T::max_value(),
            g: T::max_value(),
            b: T::max_value(),
        }
    }
}

impl<T> Index<usize> for RGBf<T>
where
    T: Scalar,
{
    type Output = T;

    fn index<'a>(&'a self, i: usize) -> &'a T {
        match i {
            0 => &self.r,
            1 => &self.g,
            2 => &self.b,
            _ => panic!("Tried to access RGBf with index of {}", i),
        }
    }
}

impl<T> IndexMut<usize> for RGBf<T>
where
    T: Scalar,
{
    fn index_mut<'a>(&'a mut self, i: usize) -> &'a mut T {
        match i {
            0 => &mut self.r,
            1 => &mut self.g,
            2 => &mut self.b,
            _ => panic!("Tried to access RGBf with index of {}", i),
        }
    }
}

impl<T> fmt::Display for RGBf<T>
where
    T: Scalar + fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {}, {})", self.r, self.g, self.b)
    }
}

/// Addition operator
impl<T> Add for RGBf<T>
where
    T: Scalar,
{
    type Output = RGBf<T>;

    fn add(self, rhs: RGBf<T>) -> RGBf<T> {
        RGBf::<T> {
            r: self.r + rhs.r,
            g: self.g + rhs.g,
            b: self.b + rhs.b,
        }
    }
}

/// Subtraction operator
impl<T> Sub for RGBf<T>
where
    T: Scalar,
{
    type Output = RGBf<T>;

    fn sub(self, rhs: RGBf<T>) -> RGBf<T> {
        RGBf::<T> {
            r: self.r - rhs.r,
            g: self.g - rhs.g,
            b: self.b - rhs.b,
        }
    }
}

/// Multiplication operator
impl<T> Mul for RGBf<T>
where
    T: Scalar,
{
    type Output = RGBf<T>;

    fn mul(self, rhs: RGBf<T>) -> RGBf<T> {
        RGBf::<T> {
            r: self.r * rhs.r,
            g: self.g * rhs.g,
            b: self.b * rhs.b,
        }
    }
}

/// Division operator
impl<T> Div for RGBf<T>
where
    T: Scalar,
{
    type Output = RGBf<T>;

    fn div(self, rhs: RGBf<T>) -> RGBf<T> {
        RGBf::<T> {
            r: self.r / rhs.r,
            g: self.g / rhs.g,
            b: self.b / rhs.b,
        }
    }
}

/// Unary negation
impl<T> Neg for RGBf<T>
where
    T: Scalar,
{
    type Output = RGBf<T>;

    fn neg(self) -> RGBf<T> {
        RGBf::<T> {
            r: -self.r,
            g: -self.g,
            b: -self.b,
        }
    }
}

/// Multiplication by a T
impl<T> Mul<T> for RGBf<T>
where
    T: Scalar,
{
    type Output = RGBf<T>;

    fn mul(self, rhs: T) -> RGBf<T> {
        RGBf::<T> {
            r: self.r * rhs,
            g: self.g * rhs,
            b: self.b * rhs,
        }
    }
}

/// Division by a T
impl<T> Div<T> for RGBf<T>
where
    T: Scalar,
{
    type Output = RGBf<T>;

    fn div(self, rhs: T) -> RGBf<T> {
        RGBf::<T> {
            r: self.r / rhs,
            g: self.g / rhs,
            b: self.b / rhs,
        }
    }
}

/// Addition by a T
impl<T> Add<T> for RGBf<T>
where
    T: Scalar,
{
    type Output = RGBf<T>;

    fn add(self, rhs: T) -> RGBf<T> {
        RGBf::<T> {
            r: self.r + rhs,
            g: self.g + rhs,
            b: self.b + rhs,
        }
    }
}

/// Subtraction by a T
impl<T> Sub<T> for RGBf<T>
where
    T: Scalar,
{
    type Output = RGBf<T>;

    fn sub(self, rhs: T) -> RGBf<T> {
        RGBf::<T> {
            r: self.r - rhs,
            g: self.g - rhs,
            b: self.b - rhs,
        }
    }
}

/// Macro to implement right-side multiplication: T * RGBf<T>
macro_rules! rgbf_impl_rhs_mul {
    ($($t:ty)*) => ($(
        impl Mul<RGBf<$t>> for $t {
            type Output = RGBf<$t>;
            fn mul(self, rhs: RGBf<$t>) -> RGBf<$t> {
                RGBf {
                    r: self * rhs.r,
                    g: self * rhs.g,
                    b: self * rhs.b,
                }
            }
        }
    )*)
}

rgbf_impl_rhs_mul! {
    f32
}

/// Macro to implement right-side addition: T + Vec2<T>
macro_rules! rgbf_impl_rhs_add {
    ($($t:ty)*) => ($(
        impl Add<RGBf<$t>> for $t {
            type Output = RGBf<$t>;
            fn add(self, rhs: RGBf<$t>) -> RGBf<$t> {
                RGBf {
                    r: rhs.r + self,
                    g: rhs.g + self,
                    b: rhs.b + self,
                }
            }
        }
    )*)
}

rgbf_impl_rhs_add! {
    f32
}

/// Macro to implement right-side subtraction: T - Vec2<T>
macro_rules! rgbf_impl_rhs_sub {
    ($($t:ty)*) => ($(
        impl Sub<RGBf<$t>> for $t {
            type Output = RGBf<$t>;
            fn sub(self, rhs: RGBf<$t>) -> RGBf<$t> {
                RGBf {
                    r: self - rhs.r,
                    g: self - rhs.g,
                    b: self - rhs.b,
                }
            }
        }
    )*)
}

rgbf_impl_rhs_sub! {
    f32
}

/// Macro to implement right-side division: T / Vec2<T>
macro_rules! rgbf_impl_rhs_div {
    ($($t:ty)*) => ($(
        impl Div<RGBf<$t>> for $t {
            type Output = RGBf<$t>;
            fn div(self, rhs: RGBf<$t>) -> RGBf<$t> {
                RGBf {
                    r: self / rhs.r,
                    g: self / rhs.g,
                    b: self / rhs.b,
                }
            }
        }
    )*)
}

rgbf_impl_rhs_div! {
    f32
}

pub fn hmax<T>(c: RGBf<T>) -> T
where
    T: Real,
{
    c.r.max(c.g.max(c.b))
}

pub fn normalize<T>(c: RGBf<T>) -> RGBf<T>
where
    T: Real,
{
    c / hmax(c)
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Default)]
pub struct RGBu8 {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Default)]
pub struct RGBu16 {
    pub r: u16,
    pub g: u16,
    pub b: u16,
}

#[cfg(feature = "f16")]
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Default)]
pub struct RGBf16 {
    pub r: f16,
    pub g: f16,
    pub b: f16,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Default)]
pub struct RGBAf32 {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

#[cfg(feature = "f16")]
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Default)]
pub struct RGBAf16 {
    pub r: f16,
    pub g: f16,
    pub b: f16,
    pub a: f16,
}

#[inline]
pub fn rgbu8(r: u8, g: u8, b: u8) -> RGBu8 {
    RGBu8 { r, g, b }
}

#[inline]
pub fn rgbu16(r: u16, g: u16, b: u16) -> RGBu16 {
    RGBu16 { r, g, b }
}

#[cfg(feature = "f16")]
#[inline]
pub fn rgbf16(r: f16, g: f16, b: f16) -> RGBf16 {
    RGBf16 { r, g, b }
}

#[cfg(feature = "f16")]
#[inline]
pub fn rgbaf16(r: f16, g: f16, b: f16, a: f16) -> RGBAf16 {
    RGBAf16 { r, g, b, a }
}

#[inline]
pub fn rgbaf32(r: f32, g: f32, b: f32, a: f32) -> RGBAf32 {
    RGBAf32 { r, g, b, a }
}

impl From<RGBf32> for RGBu8 {
    fn from(c: RGBf32) -> RGBu8 {
        RGBu8 {
            r: (clamp(c.r, 0.0, 1.0) * 255.0).round() as u8,
            g: (clamp(c.g, 0.0, 1.0) * 255.0).round() as u8,
            b: (clamp(c.b, 0.0, 1.0) * 255.0).round() as u8,
        }
    }
}

impl From<RGBf32> for RGBu16 {
    fn from(c: RGBf32) -> RGBu16 {
        RGBu16 {
            r: (clamp(c.r, 0.0, 1.0) * 65535.0).round() as u16,
            g: (clamp(c.g, 0.0, 1.0) * 65535.0).round() as u16,
            b: (clamp(c.b, 0.0, 1.0) * 65535.0).round() as u16,
        }
    }
}

impl From<RGBu8> for RGBf32 {
    fn from(c: RGBu8) -> RGBf32 {
        RGBf32 {
            r: c.r as f32 / 255.0,
            g: c.g as f32 / 255.0,
            b: c.b as f32 / 255.0,
        }
    }
}

impl From<RGBu16> for RGBf32 {
    fn from(c: RGBu16) -> RGBf32 {
        RGBf32 {
            r: c.r as f32 / 65535.0,
            g: c.g as f32 / 65535.0,
            b: c.b as f32 / 65535.0,
        }
    }
}

impl fmt::Display for RGBu8 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {}, {})", self.r, self.g, self.b)
    }
}

impl fmt::Display for RGBu16 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {}, {})", self.r, self.g, self.b)
    }
}

#[cfg(feature = "f16")]
impl fmt::Display for RGBf16 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {}, {})", self.r, self.g, self.b)
    }
}
