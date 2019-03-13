//! Spectral Power Distributions

use super::cmf;
use super::math::clamp;
use super::xyz::XYZ;
use std::iter::FromIterator;
use std::ops::Index;

pub use crate::spd_conversion::{spd_to_xyz, spd_to_xyz_with_illuminant};

/// Distribution of the spectral data. Some algorithms can be optimized if it
/// is known that the samples are evenly distributed
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Distribution {
    /// The samples are evenly distributed and the contained value is the
    /// wavelength distance between samples
    Uniform(f32),
    /// The samples are not evenly distributed
    Varying,
}

fn calculate_distribution(samples: &[(f32, f32)]) -> Distribution {
    let mut is_uniform = true;
    let step_size = samples[1].0 - samples[0].0;

    for i in 0..samples.len() {
        if i > 0 {
            let ss = samples[i].0 - samples[i - 1].0;
            if ss != step_size {
                is_uniform = false;
            }
        }
    }

    if is_uniform {
        Distribution::Uniform(step_size)
    } else {
        Distribution::Varying
    }
}

/// A Spectral Power Distribution. An SPD is a vector of (wavelength, value)
/// pairs. Wavelengths are assumed to be in nanometers.
#[derive(PartialEq, Debug, Clone)]
pub struct SPD {
    samples: Vec<(f32, f32)>,
    distribution: Distribution,
}

impl SPD {
    /// Create a new SPD by copying the given slice of samples
    pub fn new(samples: &[(f32, f32)]) -> SPD {
        let samples = samples.to_vec();
        let distribution = calculate_distribution(&samples);
        SPD {
            samples,
            distribution,
        }
    }

    /// Create a new SPD by consuming the given Vec of samples.
    pub fn consume(samples: Vec<(f32, f32)>) -> SPD {
        let distribution = calculate_distribution(&samples);
        SPD {
            samples,
            distribution,
        }
    }

    /// Create a new SPD by copying the given wavelength and value slices
    pub fn from_wavelength_and_value(wavelength: &[f32], value: &[f32]) -> SPD {
        let len = std::cmp::min(wavelength.len(), value.len());
        let mut samples = Vec::<(f32, f32)>::with_capacity(len);

        let w = &wavelength[..len];
        let p = &value[..len];

        let mut is_uniform = true;
        let step_size = w[1] - w[0];

        for i in 0..len {
            samples.push((w[i], p[i]));
            if i > 0 {
                let ss = w[i] - w[i - 1];
                if ss != step_size {
                    is_uniform = false;
                }
            }
        }

        let distribution = if is_uniform {
            Distribution::Uniform(step_size)
        } else {
            Distribution::Varying
        };

        SPD {
            samples,
            distribution,
        }
    }

    /// The smallest wavelength of the range covered by this SPD
    pub fn start(&self) -> f32 {
        self.samples.first().unwrap().0
    }

    /// The largest wavelength of the range covered by this SPD
    pub fn end(&self) -> f32 {
        self.samples.last().unwrap().0
    }

    /// The size of the range covered by this SPD
    pub fn range(&self) -> f32 {
        self.end() - self.start()
    }

    /// The number of samples in this SPD
    pub fn num_samples(&self) -> usize {
        self.samples.len()
    }

    /// The distribution of this SPD
    pub fn distribution(&self) -> Distribution {
        self.distribution
    }

    /// Interpolates the value for `lambda` from the SPD. If `lambda` is
    /// outside of the range of the SPD, it is clamped to lie within the range.
    pub fn value_at(&self, lambda: f32) -> f32 {
        let t = (lambda - self.start()) / self.range();
        let i0 = (t * self.num_samples() as f32) as i32;
        let i1 = i0 + 1;
        let i0 = clamp(i0, 0, self.num_samples() as i32 - 1) as usize;
        let i1 = clamp(i1, 0, self.num_samples() as i32 - 1) as usize;

        let s0 = self.samples[i0];
        let s1 = self.samples[i1];

        if s0.0 == s1.0 {
            s0.1
        } else {
            let dt = clamp((lambda - s0.0) / (s1.0 - s0.0), 0.0, 1.0);
            super::math::lerp(s0.1, s1.1, dt)
        }
    }

    /// Interpolates the value for `lambda` from the SPD. If `lambda` is
    /// outside of the range of the SPD, it is clamped to lie within the range.
    pub fn value_at_extrapolate(&self, lambda: f32) -> f32 {
        if lambda < self.start() {
            let l0 = 0;
            let l1 = 1;
            self.samples[l1].1
                + (lambda - self.samples[l1].0)
                    / (self.samples[l0].0 - self.samples[l1].0)
                    * (self.samples[l0].1 - self.samples[l1].1)
        } else if lambda > self.end() {
            let l0 = self.num_samples() - 1;
            let l1 = self.num_samples() - 2;
            self.samples[l1].1
                + (lambda - self.samples[l1].0)
                    / (self.samples[l0].0 - self.samples[l1].0)
                    * (self.samples[l0].1 - self.samples[l1].1)
        } else {
            let t = (lambda - self.start()) / self.range();
            let i0 = (t * (self.num_samples() - 1) as f32) as i32;
            let i1 = i0 + 1;
            let i0 = clamp(i0, 0, self.num_samples() as i32 - 1) as usize;
            let i1 = clamp(i1, 0, self.num_samples() as i32 - 1) as usize;

            let s0 = self.samples[i0];
            let s1 = self.samples[i1];

            if s0.0 == s1.0 {
                s0.1
            } else {
                let dt = clamp((lambda - s0.0) / (s1.0 - s0.0), 0.0, 1.0);
                super::math::lerp(s0.1, s1.1, dt)
            }
        }
    }

    /// Get a reference to the vector of samples contained in this SPD
    pub fn samples(&self) -> &Vec<(f32, f32)> {
        &self.samples
    }

    /// Convert this SPD to a tristimulus XYZ value using the CIE 1931 2-degree
    /// color matching functions. The SPD is assumed to be emissive.
    pub fn to_xyz(&self) -> XYZ {
        spd_to_xyz(self, &cmf::CIE_1931_2_degree)
    }

    /// Convert this SPD to a tristimulus XYZ value using the CIE 1931 2-degree
    /// color matching functions and the given reference illuminant SPD.
    pub fn to_xyz_with_illuminant(&self, illum: &SPD) -> XYZ {
        spd_to_xyz_with_illuminant(self, &cmf::CIE_1931_2_degree, illum)
    }

    /// Returns an iterator that interpolates this `SPD` over the range [`start`, `end`] with the given number of `steps`
    pub fn interpolate_by(
        &self,
        start: f32,
        end_inc: f32,
        steps: u32,
    ) -> InterpolatingIterator {
        InterpolatingIterator {
            spd: &self,
            current: 0,
            steps: steps,
            start: start,
            range: end_inc - start,
        }
    }

    /// Returns an iterator that interpolates and extrapolates this `SPD` over the range [`start`, `end`] with the given number of `steps`
    pub fn extrapolate_by(
        &self,
        start: f32,
        end_inc: f32,
        steps: u32,
    ) -> ExtrapolatingIterator {
        ExtrapolatingIterator {
            spd: &self,
            current: 0,
            steps: steps,
            start: start,
            range: end_inc - start,
        }
    }

    pub fn zip<'a, 'b>(
        &'a self,
        rhs: &'b SPD,
    ) -> ZippedExtrapolatingIterator<'a, 'b> {
        let start = self.start().min(rhs.start());
        let end = self.end().max(rhs.end());
        let delta = (self.range() / (self.num_samples() as f32 - 1.0))
            .min(rhs.range() / (rhs.num_samples() as f32 - 1.0));
        let num_samples = ((end - start) / delta) as u32 + 1;
        ZippedExtrapolatingIterator {
            spd_l: self,
            spd_r: rhs,
            current: 0,
            steps: num_samples,
            start,
            delta,
        }
    }
}

impl Index<usize> for SPD {
    type Output = (f32, f32);
    fn index(&self, index: usize) -> &(f32, f32) {
        &self.samples[index]
    }
}

pub struct InterpolatingIterator<'a> {
    spd: &'a SPD,
    current: u32,
    steps: u32,
    start: f32,
    range: f32,
}

impl<'a> Iterator for InterpolatingIterator<'a> {
    type Item = (f32, f32);
    fn next(&mut self) -> Option<(f32, f32)> {
        if self.current < self.steps {
            let delta = (self.current as f32) / ((self.steps - 1) as f32);
            let lambda = self.start + self.range * delta;
            self.current += 1;
            Some((lambda, self.spd.value_at(lambda).max(0.0)))
        } else {
            None
        }
    }
}

pub struct ZippedExtrapolatingIterator<'a, 'b> {
    spd_l: &'a SPD,
    spd_r: &'b SPD,
    current: u32,
    steps: u32,
    start: f32,
    delta: f32,
}

impl<'a, 'b> Iterator for ZippedExtrapolatingIterator<'a, 'b> {
    type Item = (f32, f32, f32);
    fn next(&mut self) -> Option<(f32, f32, f32)> {
        if self.current < self.steps {
            let lambda = self.start + self.delta * self.current as f32;
            self.current += 1;
            Some((
                lambda,
                self.spd_l.value_at_extrapolate(lambda).max(0.0),
                self.spd_r.value_at_extrapolate(lambda).max(0.0),
            ))
        } else {
            None
        }
    }
}

pub struct ExtrapolatingIterator<'a> {
    spd: &'a SPD,
    current: u32,
    steps: u32,
    start: f32,
    range: f32,
}

impl<'a> Iterator for ExtrapolatingIterator<'a> {
    type Item = (f32, f32);
    fn next(&mut self) -> Option<(f32, f32)> {
        if self.current < self.steps {
            let delta = (self.current as f32) / ((self.steps - 1) as f32);
            let lambda = self.start + self.range * delta;
            self.current += 1;
            Some((lambda, self.spd.value_at_extrapolate(lambda).max(0.0)))
        } else {
            None
        }
    }
}

impl FromIterator<(f32, f32)> for SPD {
    fn from_iter<I: IntoIterator<Item = (f32, f32)>>(iter: I) -> Self {
        let mut v = Vec::new();
        for i in iter {
            v.push(i)
        }

        SPD::consume(v)
    }
}

use std::ops::{Add, Div, Mul, Neg, Sub};
use std::ops::{AddAssign, DivAssign, MulAssign, SubAssign};

impl Add for SPD {
    type Output = SPD;

    fn add(self, rhs: SPD) -> SPD {
        self.zip(&rhs).map(|(l, v_l, v_r)| (l, v_l + v_r)).collect()
    }
}

impl Mul for SPD {
    type Output = SPD;

    fn mul(self, rhs: SPD) -> SPD {
        self.zip(&rhs).map(|(l, v_l, v_r)| (l, v_l * v_r)).collect()
    }
}

impl Sub for SPD {
    type Output = SPD;

    fn sub(self, rhs: SPD) -> SPD {
        self.zip(&rhs).map(|(l, v_l, v_r)| (l, v_l - v_r)).collect()
    }
}

impl Div for SPD {
    type Output = SPD;

    fn div(self, rhs: SPD) -> SPD {
        self.zip(&rhs).map(|(l, v_l, v_r)| (l, v_l / v_r)).collect()
    }
}

impl Neg for SPD {
    type Output = SPD;

    fn neg(self) -> SPD {
        self.samples.into_iter().map(|(l, v)| (l, -v)).collect()
    }
}

impl Add<f32> for SPD {
    type Output = SPD;

    fn add(self, rhs: f32) -> SPD {
        self.samples
            .into_iter()
            .map(|(l, v)| (l, v + rhs))
            .collect()
    }
}

impl Mul<f32> for SPD {
    type Output = SPD;

    fn mul(self, rhs: f32) -> SPD {
        self.samples
            .into_iter()
            .map(|(l, v)| (l, v * rhs))
            .collect()
    }
}

impl Sub<f32> for SPD {
    type Output = SPD;

    fn sub(self, rhs: f32) -> SPD {
        self.samples
            .into_iter()
            .map(|(l, v)| (l, v - rhs))
            .collect()
    }
}

impl Div<f32> for SPD {
    type Output = SPD;

    fn div(self, rhs: f32) -> SPD {
        self.samples
            .into_iter()
            .map(|(l, v)| (l, v / rhs))
            .collect()
    }
}

impl Add<SPD> for f32 {
    type Output = SPD;
    fn add(self, rhs: SPD) -> SPD {
        rhs.samples
            .into_iter()
            .map(|(l, v)| (l, v + self))
            .collect()
    }
}

impl Sub<SPD> for f32 {
    type Output = SPD;
    fn sub(self, rhs: SPD) -> SPD {
        rhs.samples
            .into_iter()
            .map(|(l, v)| (l, v - self))
            .collect()
    }
}

impl Mul<SPD> for f32 {
    type Output = SPD;
    fn mul(self, rhs: SPD) -> SPD {
        rhs.samples
            .into_iter()
            .map(|(l, v)| (l, v * self))
            .collect()
    }
}

impl Div<SPD> for f32 {
    type Output = SPD;
    fn div(self, rhs: SPD) -> SPD {
        rhs.samples
            .into_iter()
            .map(|(l, v)| (l, v / self))
            .collect()
    }
}

impl AddAssign for SPD {
    fn add_assign(&mut self, rhs: SPD) {
        *self = self.zip(&rhs).map(|(l, v_l, v_r)| (l, v_l + v_r)).collect();
    }
}

impl MulAssign for SPD {
    fn mul_assign(&mut self, rhs: SPD) {
        *self = self.zip(&rhs).map(|(l, v_l, v_r)| (l, v_l * v_r)).collect();
    }
}

impl SubAssign for SPD {
    fn sub_assign(&mut self, rhs: SPD) {
        *self = self.zip(&rhs).map(|(l, v_l, v_r)| (l, v_l - v_r)).collect();
    }
}

impl DivAssign for SPD {
    fn div_assign(&mut self, rhs: SPD) {
        *self = self.zip(&rhs).map(|(l, v_l, v_r)| (l, v_l / v_r)).collect();
    }
}

impl AddAssign<f32> for SPD {
    fn add_assign(&mut self, rhs: f32) {
        self.samples
            .iter_mut()
            .map(|(_, v)| {
                *v += rhs;
            })
            .all(|_| true);
    }
}

impl SubAssign<f32> for SPD {
    fn sub_assign(&mut self, rhs: f32) {
        self.samples
            .iter_mut()
            .map(|(_, v)| {
                *v -= rhs;
            })
            .all(|_| true);
    }
}

impl MulAssign<f32> for SPD {
    fn mul_assign(&mut self, rhs: f32) {
        self.samples
            .iter_mut()
            .map(|(_, v)| {
                *v *= rhs;
            })
            .all(|_| true);
    }
}

impl DivAssign<f32> for SPD {
    fn div_assign(&mut self, rhs: f32) {
        self.samples
            .iter_mut()
            .map(|(l, v)| {
                *v /= rhs;
            })
            .all(|_| true);
    }
}

#[test]
fn test_interpolation() {
    let spd =
        SPD::new(&[(400.0, 0.5), (500.0, 1.0), (600.0, 1.0), (700.0, 0.5)]);

    let i1: SPD = spd.interpolate_by(300.0, 800.0, 6).collect();
    assert_eq!(
        i1,
        SPD::new(&[
            (300.0, 0.5),
            (400.0, 0.5),
            (500.0, 1.0),
            (600.0, 1.0),
            (700.0, 0.5),
            (800.0, 0.5),
        ])
    );

    let i2: SPD = spd.extrapolate_by(300.0, 800.0, 6).collect();
    assert_eq!(
        i2,
        SPD::new(&[
            (300.0, 0.0),
            (400.0, 0.5),
            (500.0, 1.0),
            (600.0, 1.0),
            (700.0, 0.5),
            (800.0, 0.0),
        ])
    );
}

#[test]
fn test_ops() {
    let a = SPD::new(&[(400.0, 1.0), (500.0, 2.0), (600.0, 3.0), (700.0, 4.0)]);

    let b = SPD::new(&[(400.0, 5.0), (500.0, 6.0), (600.0, 7.0), (700.0, 8.0)]);

    assert_eq!(
        a + b,
        SPD::new(&[(400.0, 6.0), (500.0, 8.0), (600.0, 10.0), (700.0, 12.0),])
    );

    let c = SPD::new(&[(500.0, 1.0), (600.0, 1.0), (700.0, 0.0)]);
    let d = SPD::new(&[
        (400.0, 0.0),
        (450.0, 0.25),
        (500.0, 0.5),
        (550.0, 1.0),
        (600.0, 0.5),
        (650.0, 0.25),
        (700.0, 0.0),
    ]);

    assert_eq!(
        c.clone() + d.clone(),
        SPD::new(&[
            (400.0, 1.0),
            (450.0, 1.25),
            (500.0, 1.5),
            (550.0, 2.0),
            (600.0, 1.5),
            (650.0, 0.75),
            (700.0, 0.0),
        ])
    );

    let mut c1 = c.clone();
    c1 += d;
    assert_eq!(
        c1,
        SPD::new(&[
            (400.0, 1.0),
            (450.0, 1.25),
            (500.0, 1.5),
            (550.0, 2.0),
            (600.0, 1.5),
            (650.0, 0.75),
            (700.0, 0.0),
        ])
    );

    let a = SPD::new(&[(100.0, 1.0), (200.0, 1.0)]);
    let b = SPD::new(&[(100.0, 2.0), (200.0, 2.0)]);

    assert_eq!(
        a.clone() * b.clone(),
        SPD::new(&[(100.0, 2.0), (200.0, 2.0),])
    );

    assert_eq!(
        a.clone() / b.clone(),
        SPD::new(&[(100.0, 0.5), (200.0, 0.5),])
    );

    assert_eq!(
        b.clone() - a.clone(),
        SPD::new(&[(100.0, 1.0), (200.0, 1.0),])
    );

    assert_eq!(a.clone() * 4.0, SPD::new(&[(100.0, 4.0), (200.0, 4.0),]));

    assert_eq!(a.clone() / 4.0, SPD::new(&[(100.0, 0.25), (200.0, 0.25),]));

    assert_eq!(a.clone() - 0.5, SPD::new(&[(100.0, 0.5), (200.0, 0.5),]));
}
