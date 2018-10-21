use super::traits::*;
use std::ops::{Index, IndexMut};
use crate::xyz::XYZ;
use crate::rgb::RGBf32;

pub fn clamp<T>(x: T, a: T, b: T) -> T where T: PartialOrd {
    if x < a {
        a
    } else if x > b {
        b
    } else {
        x
    }
}

pub fn lerp(a: f32, b: f32, t: f32) -> f32 {
    (1.0 - t) * a + t * b
}

/// Returns true if x and y are equal with an absolute error of e
pub fn equal_with_abs_error(x: f32, y: f32, e: f32) -> bool
{
    let a = if x > y { x - y } else { y - x };
    a <= e
}

/// Returns true if x and y are equal with a relative error of e
pub fn equal_with_rel_error(x: f32, y: f32, e: f32) -> bool
{
    let a = if x > y { x - y } else { y - x };
    let b = if x > 0.0 { x } else { -x };
    a <= e * b
}

#[inline(always)]
pub fn sqrt(x: f32) -> f32 {
    x.sqrt()
}

#[inline(always)]
pub fn sqr(x: f32) -> f32 {
    x*x
}

#[inline(always)]
pub fn pow(x: f32, y: f32) -> f32 {
    x.powf(y)
}

#[inline(always)]
pub fn powi(x: f32, i: i32) -> f32 {
    x.powi(i)
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
/// 3x3 Matrix type 
pub struct Matrix33
{
    pub x: [f32; 9],
}

impl Matrix33
{
    /// Return a new identity matrix
    pub fn make_identity() -> Matrix33 {
        Matrix33 {
            x: [
                1.0, 0.0, 0.0,
                0.0, 1.0, 0.0,
                0.0, 0.0, 1.0,
            ],
        }
    }

    /// Return a new matrix initialized with the `values` passed
    pub fn new(values: [f32; 9]) -> Matrix33 {
        Matrix33{ x: values }
    }

    /// Compare 2 matrices for equality with absolute tolerance `e`
    pub fn equal_with_abs_error(self, other: &Matrix33, e: f32) -> bool {
        for it in self.x.iter().zip(other.x.iter()) {
            let (a, b) = it;
            if !equal_with_abs_error(*a, *b, e) {
                return false;
            }
        }

        true
    }

    /// Compare 2 matrices for equality with relative tolerance `e`
    pub fn equal_with_rel_error(self, other: &Matrix33, e: f32) -> bool {
        for it in self.x.iter().zip(other.x.iter()) {
            let (a, b) = it;
            if !equal_with_rel_error(*a, *b, e) {
                return false;
            }
        }

        true
    }

    /// Return the transpose of this matrix
    pub fn transposed(self) -> Matrix33 {
        Matrix33 {
            x: [
                self[0][0], self[1][0], self[2][0], self[0][1], self[1][1],
                self[2][1], self[0][3], self[1][3], self[2][3],
            ],
        }
    }

    /// Return the determinant of this matrix
    pub fn determinant(self) -> f32 {
        self[0][0] * (self[1][1] * self[2][2] - self[1][2] * self[2][1])
            + self[0][1] * (self[1][2] * self[2][0] - self[1][0] * self[2][2])
            + self[0][2] * (self[1][0] * self[2][1] - self[1][1] * self[2][0])
    }

/*
    /// Transform the Vec2 `v` as a point
    pub fn transformp(self, v: Vec2<T>) -> Vec2<T> {
        let a = v.x * self[0][0] + v.y * self[1][0] + self[2][0];
        let b = v.x * self[0][1] + v.y * self[1][1] + self[2][1];
        let w = v.x * self[0][2] + v.y * self[1][2] + self[2][2];

        Vec2::<T> { x: a / w, y: b / w }
    }

    /// Transform the Vec2 `v` as a vector
    pub fn transformv(self, v: Vec2<T>) -> Vec2<T> {
        let a = v.x * self[0][0] + v.y * self[1][0];
        let b = v.x * self[0][1] + v.y * self[1][1];

        Vec2::<T> { x: a, y: b }
    }

    /// Wrapper for Imath's confusingly named convention for matrix * point
    pub fn mult_vec_matrix(self, v: Vec2<T>) -> Vec2<T> {
        self.transformp(v)
    }

    /// Wrapper for Imath's confusingly named convention for matrix * vector
    pub fn mult_dir_matrix(self, v: Vec2<T>) -> Vec2<T> {
        self.transformv(v)
    }

*/
    /// Gauss-Jordan matrix inversion
    pub fn gj_inverse(self) -> Option<Matrix33> {
        let mut mtx_t = self;
        let mut mtx_s = Matrix33::make_identity();

        // Forward elimination
        for i in 0..2 {
            let mut pivot = i;
            let mut pivot_size: f32 = self[i][i].abs();

            for j in (i + 1)..3 {
                let tmp = self[j][i].abs();
                if tmp > pivot_size {
                    pivot = j;
                    pivot_size = tmp;
                }
            }

            if pivot_size == 0.0 {
                // Singular matrix - no solution
                return None;
            }

            if pivot != i {
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
                    mtx_t[j][k] = mtx_t[j][k] - (f * mtx_t[i][k]);
                    mtx_s[j][k] = mtx_s[j][k] - (f * mtx_s[i][k]);
                }
            }
        }

        // Backward substitution
        for i in (0..3).rev() {
            let f = mtx_t[i][i];
            if f == 0.0 {
                // Singular matrix - cannot invert
                return None;
            }

            for j in 0..3 {
                mtx_t[i][j] = mtx_t[i][j] / f;
                mtx_s[i][j] = mtx_s[i][j] / f;
            }

            for j in 0..i {
                let f = mtx_t[j][i];
                for k in 0..3 {
                    mtx_t[j][k] = mtx_t[j][k] - (f * mtx_t[i][k]);
                    mtx_s[j][k] = mtx_s[j][k] - (f * mtx_s[i][k]);
                }
            }
        }

        Some(mtx_s)
    }

    /// Matrix inverse
    pub fn inverse(self) -> Option<Matrix33> {
        if self[0][2] != 0.0
            || self[1][2] != 0.0
            || self[2][2] != 1.0
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

            if r.abs() >= 1.0 {
                for s in mtx_s.x.iter_mut() {
                    *s = *s / r;
                }
            } else {
                let mr = r.abs() / std::f32::MIN_POSITIVE;
                for s in mtx_s.x.iter_mut() {
                    if mr > s.abs() {
                        *s = *s / r;
                    } else {
                        return None;
                    }
                }
            }

            return Some(mtx_s);
        } else {
            let mut mtx_s = Matrix33::new([
                self[1][1],
                -self[0][1],
                0.0,
                -self[1][0],
                self[0][0],
                0.0,
                0.0,
                0.0,
                1.0,
            ]);

            let r = self[0][0] * self[1][1] - self[1][0] * self[0][1];

            if r.abs() >= 1.0 {
                for s in mtx_s.x.iter_mut() {
                    *s = *s / r;
                }
            } else {
                let mr = r.abs() / std::f32::MIN_POSITIVE;
                for s in mtx_s.x.iter_mut() {
                    if mr > s.abs() {
                        *s = *s / r;
                    } else {
                        return None;
                    }
                }
            }

            mtx_s[2][0] = -self[2][0] * mtx_s[0][0] - self[2][1] * mtx_s[1][0];
            mtx_s[2][1] = -self[2][0] * mtx_s[0][1] - self[2][1] * mtx_s[1][1];

            return Some(mtx_s);
        }
    }
}

/// Index operator. Returns a slice of the underlying matrix to allow
/// `m[i][j]` indexing
impl Index<usize> for Matrix33
{
    type Output = [f32];

    fn index(&self, index: usize) -> &[f32] {
        let offset = index * 3;
        &self.x[offset..(offset + 3)]
    }
}

/// Mutable Index operator. Returns a slice of the underlying matrix to allow
/// `m[i][j]` indexing
impl IndexMut<usize> for Matrix33
{
    fn index_mut(&mut self, index: usize) -> &mut [f32] {
        let offset = index * 3;
        &mut self.x[offset..(offset + 3)]
    }
}

impl Mul for Matrix33
{
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        let mut m = Matrix33::new([0.0; 9]);
        for i in 0..3 {
            for j in 0..3 {
                for k in 0..3 {
                    m[i][j] = m[i][j] + self[i][k] * rhs[k][j];
                }
            }
        }

        m
    }
}

impl Add<f32> for Matrix33
{
    type Output = Self;

    fn add(self, rhs: f32) -> Self {
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

impl Sub<f32> for Matrix33
{
    type Output = Self;

    fn sub(self, rhs: f32) -> Self {
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

impl Mul<f32> for Matrix33
{
    type Output = Self;

    fn mul(self, rhs: f32) -> Self {
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

impl Div<f32> for Matrix33
{
    type Output = Self;

    fn div(self, rhs: f32) -> Self {
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

impl Neg for Matrix33
{
    type Output = Self;

    fn neg(self) -> Self {
        Matrix33::new([
            -self.x[0], -self.x[1], -self.x[2], -self.x[3], -self.x[4],
            -self.x[5], -self.x[6], -self.x[7], -self.x[8],
        ])
    }
}

impl Mul<XYZ> for Matrix33
{
    type Output = XYZ;

    fn mul(self, xyz: XYZ) -> XYZ {
        XYZ::new(
            self.x[0] * xyz.x + self.x[1] * xyz.y + self.x[2] * xyz.z,
            self.x[3] * xyz.x + self.x[4] * xyz.y + self.x[5] * xyz.z,
            self.x[6] * xyz.x + self.x[7] * xyz.y + self.x[8] * xyz.z,
        )
    }
}

impl Mul<RGBf32> for Matrix33
{
    type Output = RGBf32;

    fn mul(self, rgb: RGBf32) -> RGBf32 {
        RGBf32::new(
            self.x[0] * rgb.r + self.x[1] * rgb.g + self.x[2] * rgb.b,
            self.x[3] * rgb.r + self.x[4] * rgb.g + self.x[5] * rgb.b,
            self.x[6] * rgb.r + self.x[7] * rgb.g + self.x[8] * rgb.b,
        )
    }
}