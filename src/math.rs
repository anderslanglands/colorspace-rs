pub use num_traits::{Bounded, Float, One, Zero};
pub(crate) use std::ops::{Add, Div, Mul, Neg, Sub, AddAssign, SubAssign, MulAssign, DivAssign};

use crate::rgb::RGBf;
use crate::xyz::XYZ;
use std::ops::{Index, IndexMut};

/// Clamp `x` to lie in the range `[a, b]`
pub fn clamp<T>(x: T, a: T, b: T) -> T
where
    T: PartialOrd,
{
    if x < a {
        a
    } else if x > b {
        b
    } else {
        x
    }
}

/// Linearly interpolate from `a` to `b` by `t`
pub fn lerp(a: f32, b: f32, t: f32) -> f32 {
    (1.0 - t) * a + t * b
}

#[inline(always)]
pub fn sqrt<T>(x: T) -> T where T: Real {
    x.sqrt()
}

#[inline(always)]
pub fn sqr<T>(x: T) -> T where T: Real {
    x * x
}

#[inline(always)]
pub fn abs<T>(x: T) -> T where T: Real {
    x.abs()
}

#[inline(always)]
pub fn sin<T>(x: T) -> T where T: Real {
    x.sin()
}

#[inline(always)]
pub fn asin<T>(x: T) -> T where T: Real {
    x.asin()
}

#[inline(always)]
pub fn cos<T>(x: T) -> T where T: Real {
    x.cos()
}

#[inline(always)]
pub fn acos<T>(x: T) -> T where T: Real {
    x.acos()
}

#[inline(always)]
pub fn tan<T>(x: T) -> T where T: Real {
    x.tan()
}

#[inline(always)]
pub fn atan2<T>(x: T, y: T) -> T where T: Real {
    x.atan2(y)
}

#[inline(always)]
pub fn exp<T>(x: T) -> T where T: Real {
    x.exp()
}

#[inline(always)]
pub fn pow<T>(x: T, y: T) -> T where T: Real {
    x.powf(y)
}

#[inline(always)]
pub fn hypot<T>(x: T, y: T) -> T where T: Real {
    x.hypot(y)
}

#[inline(always)]
pub fn powi<T>(x: T, i: i32) -> T where T: Real {
    x.powi(i)
}

/// 3x3 Matrix type
///
/// Based on the Imath implementation
/// https://github.com/openexr/openexr
/// Copyright (c) 2006-17, Industrial Light & Magic, a division of Lucasfilm
/// Entertainment Company Ltd.  Portions contributed and copyright held by
/// others as indicated.  All rights reserved.
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Matrix33<T> where T: Real {
    pub x: [T; 9],
}

pub type M3f64 = Matrix33<f64>;
pub type M3f32 = Matrix33<f32>;

impl<T> Matrix33<T> where T: Real {
    /// Return a new identity matrix
    pub fn make_identity() -> Matrix33<T> {
        Matrix33::<T> {
            x: [T::one(), T::zero(), T::zero(), 
                T::zero(), T::one(), T::zero(), 
                T::zero(), T::zero(), T::one()],
        }
    }

    /// Return a new matrix initialized with the `values` passed
    pub fn new(values: [T; 9]) -> Matrix33<T> {
        Matrix33 { x: values }
    }

    /// Return the transpose of this matrix
    pub fn transposed(&self) -> Matrix33<T> {
        Matrix33 {
            x: [
                self[0][0], self[1][0], self[2][0], self[0][1], self[1][1],
                self[2][1], self[0][2], self[1][2], self[2][2],
            ],
        }
    }

    /// Return the determinant of this matrix
    pub fn determinant(&self) -> T {
        self[0][0] * (self[1][1] * self[2][2] - self[1][2] * self[2][1])
            + self[0][1] * (self[1][2] * self[2][0] - self[1][0] * self[2][2])
            + self[0][2] * (self[1][0] * self[2][1] - self[1][1] * self[2][0])
    }

    /// Gauss-Jordan matrix inversion
    pub fn gj_inverse(&self) -> Option<Matrix33<T>> {
        let mut mtx_t = self.clone();
        let mut mtx_s = Matrix33::make_identity();

        // Forward elimination
        for i in 0..2 {
            let mut pivot = i;
            let mut pivot_size: T = self[i][i].abs();

            #[allow(clippy::needless_range_loop)]
            for j in (i + 1)..3 {
                let tmp = self[j][i].abs();
                if tmp > pivot_size {
                    pivot = j;
                    pivot_size = tmp;
                }
            }

            if pivot_size == T::zero() {
                // Singular matrix - no solution
                return None;
            }

            if pivot != i {
                #[allow(clippy::manual_swap)]
                for j in 0..3 {
                    let tmp1 = mtx_t[i][j];
                    mtx_t[i][j] = mtx_t[pivot][j];
                    mtx_t[pivot][j] = tmp1;

                    let tmp2 = mtx_s[i][j];
                    mtx_s[i][j] = mtx_s[pivot][j];
                    mtx_s[pivot][j] = tmp2;
                }
            }

            for j in (i + 1)..3 {
                let f = mtx_t[j][i] / mtx_t[i][i];
                for k in 0..3 {
                    let t = mtx_t[i][k];
                    let s = mtx_s[i][k];
                    mtx_t[j][k] -= f * t;
                    mtx_s[j][k] -= f * s;
                }
            }
        }

        // Backward substitution
        for i in (0..3).rev() {
            let f = mtx_t[i][i];
            if f == T::zero() {
                // Singular matrix - cannot invert
                return None;
            }

            for j in 0..3 {
                mtx_t[i][j] /= f;
                mtx_s[i][j] /= f;
            }

            for j in 0..i {
                let f = mtx_t[j][i];
                for k in 0..3 {
                    let t = mtx_t[i][k];
                    let s = mtx_s[i][k];
                    mtx_t[j][k] -= f * t;
                    mtx_s[j][k] -= f * s;
                }
            }
        }

        Some(mtx_s)
    }

    /// Matrix inverse
    pub fn inverse(self) -> Option<Matrix33<T>> {
        if self[0][2] > T::epsilon()
            || self[1][2] > T::epsilon()
            || (self[2][2] - T::one()).abs() > T::epsilon()
        {
            let mut mtx_s = Matrix33::new([
                self[1][1] * self[2][2] - self[2][1] * self[1][2],
                self[2][1] * self[0][2] - self[0][1] * self[2][2],
                self[0][1] * self[1][2] - self[1][1] * self[0][2],
                self[2][0] * self[1][2] - self[1][0] * self[2][2],
                self[0][0] * self[2][2] - self[2][0] * self[0][2],
                self[1][0] * self[0][2] - self[0][0] * self[1][2],
                self[1][0] * self[2][1] - self[2][0] * self[1][1],
                self[2][0] * self[0][1] - self[0][0] * self[2][1],
                self[0][0] * self[1][1] - self[1][0] * self[0][1],
            ]);

            let r = self[0][0] * mtx_s[0][0]
                + self[0][1] * mtx_s[1][0]
                + self[0][2] * mtx_s[2][0];

            if r.abs() >= T::one() {
                for s in mtx_s.x.iter_mut() {
                    *s /= r;
                }
            } else {
                let mr = r.abs() / T::min_positive_value();
                for s in mtx_s.x.iter_mut() {
                    if mr > s.abs() {
                        *s /= r;
                    } else {
                        return None;
                    }
                }
            }

            Some(mtx_s)
        } else {
            let mut mtx_s = Matrix33::new([
                self[1][1],
                -self[0][1],
                T::zero(),
                -self[1][0],
                self[0][0],
                T::zero(),
                T::zero(),
                T::zero(),
                T::one(),
            ]);

            let r = self[0][0] * self[1][1] - self[1][0] * self[0][1];

            if r.abs() >= T::one() {
                for s in mtx_s.x.iter_mut() {
                    *s /= r;
                }
            } else {
                let mr = r.abs() / T::min_positive_value();
                for s in mtx_s.x.iter_mut() {
                    if mr > s.abs() {
                        *s /= r;
                    } else {
                        return None;
                    }
                }
            }

            mtx_s[2][0] = -self[2][0] * mtx_s[0][0] - self[2][1] * mtx_s[1][0];
            mtx_s[2][1] = -self[2][0] * mtx_s[0][1] - self[2][1] * mtx_s[1][1];

            Some(mtx_s)
        }
    }
}

impl From<M3f64> for M3f32 {
    fn from(m: M3f64) -> M3f32 {
        M3f32 {
            x: [
                m.x[0] as f32,
                m.x[1] as f32,
                m.x[2] as f32,
                m.x[3] as f32,
                m.x[4] as f32,
                m.x[5] as f32,
                m.x[6] as f32,
                m.x[7] as f32,
                m.x[8] as f32,
            ]
        }
    }
}

/// Index operator. Returns a slice of the underlying matrix to allow
/// `m[i][j]` indexing
impl<T> Index<usize> for Matrix33<T> where T: Real {
    type Output = [T];

    fn index(&self, index: usize) -> &[T] {
        let offset = index * 3;
        &self.x[offset..(offset + 3)]
    }
}

/// Mutable Index operator. Returns a slice of the underlying matrix to allow
/// `m[i][j]` indexing
impl<T> IndexMut<usize> for Matrix33<T> where T: Real {
    fn index_mut(&mut self, index: usize) -> &mut [T] {
        let offset = index * 3;
        &mut self.x[offset..(offset + 3)]
    }
}

impl<T> Mul for Matrix33<T> where T: Real {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        let mut m = Matrix33::<T>::new([T::zero(); 9]);
        #[allow(clippy::needless_range_loop)]
        for i in 0..3 {
            for j in 0..3 {
                for k in 0..3 {
                    m[i][j] += self[i][k] * rhs[k][j];
                }
            }
        }

        m
    }
}

impl<T> Add<T> for Matrix33<T> where T: Real {
    type Output = Self;

    fn add(self, rhs: T) -> Self {
        Matrix33::new([
            self.x[0] + rhs,
            self.x[1] + rhs,
            self.x[2] + rhs,
            self.x[3] + rhs,
            self.x[4] + rhs,
            self.x[5] + rhs,
            self.x[6] + rhs,
            self.x[7] + rhs,
            self.x[8] + rhs,
        ])
    }
}

impl<T> Sub<T> for Matrix33<T> where T: Real {
    type Output = Self;

    fn sub(self, rhs: T) -> Self {
        Matrix33::new([
            self.x[0] - rhs,
            self.x[1] - rhs,
            self.x[2] - rhs,
            self.x[3] - rhs,
            self.x[4] - rhs,
            self.x[5] - rhs,
            self.x[6] - rhs,
            self.x[7] - rhs,
            self.x[8] - rhs,
        ])
    }
}

impl<T> Mul<T> for Matrix33<T> where T: Real {
    type Output = Self;

    fn mul(self, rhs: T) -> Self {
        Matrix33::new([
            self.x[0] * rhs,
            self.x[1] * rhs,
            self.x[2] * rhs,
            self.x[3] * rhs,
            self.x[4] * rhs,
            self.x[5] * rhs,
            self.x[6] * rhs,
            self.x[7] * rhs,
            self.x[8] * rhs,
        ])
    }
}

impl<T> Div<T> for Matrix33<T> where T: Real {
    type Output = Self;

    fn div(self, rhs: T) -> Self {
        Matrix33::new([
            self.x[0] / rhs,
            self.x[1] / rhs,
            self.x[2] / rhs,
            self.x[3] / rhs,
            self.x[4] / rhs,
            self.x[5] / rhs,
            self.x[6] / rhs,
            self.x[7] / rhs,
            self.x[8] / rhs,
        ])
    }
}

impl<T> Neg for Matrix33<T> where T: Real {
    type Output = Self;

    fn neg(self) -> Self {
        Matrix33::new([
            -self.x[0], -self.x[1], -self.x[2], -self.x[3], -self.x[4],
            -self.x[5], -self.x[6], -self.x[7], -self.x[8],
        ])
    }
}

impl<T> Mul<XYZ<T>> for Matrix33<T> where T: Real {
    type Output = XYZ<T>;

    fn mul(self, xyz: XYZ<T>) -> XYZ<T> {
        XYZ::new(
            self.x[0] * xyz.x + self.x[1] * xyz.y + self.x[2] * xyz.z,
            self.x[3] * xyz.x + self.x[4] * xyz.y + self.x[5] * xyz.z,
            self.x[6] * xyz.x + self.x[7] * xyz.y + self.x[8] * xyz.z,
        )
    }
}

impl<T> Mul<RGBf<T>> for Matrix33<T> where T: Real {
    type Output = RGBf<T>;

    fn mul(self, rgb: RGBf<T>) -> RGBf<T> {
        RGBf::new(
            self.x[0] * rgb.r + self.x[1] * rgb.g + self.x[2] * rgb.b,
            self.x[3] * rgb.r + self.x[4] * rgb.g + self.x[5] * rgb.b,
            self.x[6] * rgb.r + self.x[7] * rgb.g + self.x[8] * rgb.b,
        )
    }
}


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
    + AddAssign
    + SubAssign
    + MulAssign
    + DivAssign
    + std::fmt::Display
    + std::fmt::Debug
{
}

/// Further constrains what we need from a Scalar to cover just the real numbers
/// in order to be generic for functions like sqrt() that are not defined for
/// integers
pub trait Real: Scalar + Float {}

impl<T> Real for T where T: Scalar + Float {}

impl Scalar for f32 {}
impl Scalar for f64 {}