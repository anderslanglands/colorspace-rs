use num_traits::{Float, FromPrimitive, ToPrimitive};

use std::fmt::{Debug, Display};
use std::iter::FromIterator;

use float_cmp::{ApproxEq, F64Margin};

use itertools::izip;

use crate::{
    cmf::CMF,
    illuminant,
    interpolation::{
        ExtrapolatorConstant, InterpolatorSprague, SpragueCoefficients,
    },
    xyz::{xyz, XYZf64},
};

#[derive(Display, PartialEq, PartialOrd, Copy, Clone)]
#[display(fmt = "({}, {}, {})", start, end, interval)]
pub struct SpdShape<T>
where
    T: SpdElement,
{
    /// Start of the wavelength range, in nm
    pub start: T,
    /// End of the wavelength range, in nm
    pub end: T,
    /// Interval between two sample values, in nm
    pub interval: Interval<T>,
}

#[derive(Copy, Clone, PartialEq, PartialOrd)]
pub enum Interval<T>
where
    T: Float,
{
    Uniform(T),
    Varying,
}

impl<T> Display for Interval<T>
where
    T: Float + Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Interval::Uniform(v) => write!(f, "{}", v),
            Interval::Varying => write!(f, "Varying"),
        }
    }
}

impl<T> SpdShape<T>
where
    T: SpdElement,
{
    pub fn new(start: T, end: T, interval: T) -> SpdShape<T> {
        SpdShape::<T> {
            start,
            end,
            interval: Interval::Uniform(interval),
        }
    }

    pub fn astm_e308() -> SpdShape<T> {
        SpdShape::<T> {
            start: T::from(360.0).unwrap(),
            end: T::from(780).unwrap(),
            interval: Interval::Uniform(T::from(1.0).unwrap()),
        }
    }

    pub fn iter(&self) -> SpdShapeIterator<T> {
        let interval = match self.interval {
            Interval::Uniform(i) => i,
            _ => unreachable!(),
        };
        SpdShapeIterator::<T> {
            current: 0,
            end: ((self.end - self.start) / interval).to_usize().unwrap() + 1,
            start: self.start.into(),
            interval,
        }
    }
}

pub struct SpdShapeIterator<T>
where
    T: SpdElement,
{
    current: usize,
    end: usize,
    start: T,
    interval: T,
}

impl<T> Iterator for SpdShapeIterator<T>
where
    T: SpdElement,
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        self.current = self.current + 1;
        if self.current <= self.end {
            Some(
                self.start + T::from(self.current - 1).unwrap() * self.interval,
            )
        } else {
            None
        }
    }
}

pub trait SpdElement:
    Float
    + Display
    + SpragueCoefficients
    + std::iter::Sum
    + Debug
    + ToPrimitive
    + FromPrimitive
    + PartialEq
{
}

impl SpdElement for f32 {}
impl SpdElement for f64 {}

#[derive(Display, Clone, Copy, PartialEq)]
#[display(fmt = "({}, {})", nm, v)]
pub struct Sample {
    pub nm: f64,
    pub v: f64,
}

impl Sample {
    pub fn new(nm: f64, v: f64) -> Sample {
        Sample { nm, v }
    }
}

impl std::fmt::Debug for Sample {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.nm, self.v)
    }
}

impl ApproxEq for Sample {
    type Margin = F64Margin;
    fn approx_eq<T: Into<Self::Margin>>(self, other: Self, margin: T) -> bool {
        let margin = margin.into();
        self.nm.approx_eq(other.nm, margin) && self.v.approx_eq(other.v, margin)
    }
}

use super::spd::SPD;
/// A Varying Spectral Power Distribution. Stores a list of [Sample]s,
/// i.e. paired wavelength and power values. Wavelengths are assumed to be in
/// nanometres.
/// The `samples` may have uniform or varying spacing, although most functions
/// that operate on [VSPD] require uniform samples and will either error or
/// pre-interpolate when given a varying [VSPD].
/// [VSPD] is designed for flexbility and accuracy, to be used for generating
/// reference solutions. As such, it uses `f64` as an underlying storage type
/// and its methods generally do a lot of copying of the whole sample vector.
/// If you want a type that is optimized for performance at the expense of
/// accuracy, you should look at [SPD] instead.
#[derive(Clone)]
pub struct VSPD {
    pub(crate) samples: Vec<Sample>,
    shape: SpdShape<f64>,
}

impl Display for VSPD {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "VSPD({}, {}, {})[",
            self.start(),
            self.end(),
            self.interval()
        )?;
        for s in &self.samples {
            write!(f, "{}, ", s)?;
        }
        write!(f, "]")
    }
}

impl Debug for VSPD {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "VSPD({}, {}, {})[",
            self.start(),
            self.end(),
            self.interval()
        )?;
        for s in &self.samples {
            write!(f, "{}, ", s)?;
        }
        write!(f, "]")
    }
}

impl PartialEq for VSPD {
    fn eq(&self, rhs: &VSPD) -> bool {
        self.samples.len() == rhs.samples.len()
            && self.shape == rhs.shape
            && self
                .samples
                .iter()
                .zip(rhs.samples.iter())
                .all(|(l, r)| l == r)
    }
}

impl VSPD {
    /// Create a new [VSPD] with the given [Sample] vector, which must have at
    /// least two samples.
    /// # Panics
    /// If the `samples` vector has less than 2 samples.
    pub fn new(samples: Vec<Sample>) -> VSPD {
        let shape = calculate_shape(&samples);
        VSPD { samples, shape }
    }

    /// Create a new [VSPD] of the given [SpdShape] with all [Sample]s
    /// initialized to the given `value`.
    /// # Panics
    /// If the `samples` vector has less than 2 samples.
    pub fn constant(shape: SpdShape<f64>, value: f64) -> VSPD {
        let samples: Vec<Sample> =
            shape.iter().map(|nm| Sample { nm: nm, v: value }).collect();
        if samples.len() < 2 {
            panic!(
                "VSPD must have at least 2 samples. SpdShape given was: {}",
                shape
            );
        }

        VSPD { samples, shape }
    }

    /// Create a new [VSPD] of the given [SpdShape] with the values of each [Sample]
    /// given by `values`
    /// # Panics
    /// If the `samples` vector has less than 2 samples.
    pub fn from_values(shape: SpdShape<f64>, values: &[f64]) -> VSPD {
        if values.len() < 2 {
            panic!(
                "VSPD must have at least 2 samples. Got slice of {} values",
                values.len()
            );
        }
        let interval = match shape.interval {
            Interval::Uniform(i) => i,
            Interval::Varying => {
                panic!("Cannot create a VSPD with varying interval");
            }
        };
        let num_samples_from_shape =
            ((shape.end - shape.start) / interval) as usize + 1;
        if num_samples_from_shape != values.len() {
            panic!("Length of values slice did not match requested shape. SpdShape has {} samples, but values slice had {} values.", num_samples_from_shape, values.len());
        }
        let samples: Vec<Sample> = shape
            .iter()
            .zip(values.iter())
            .map(|(nm, v)| Sample { nm, v: *v })
            .collect();

        VSPD { samples, shape }
    }

    /// Get this SPD's [SpdShape]
    pub fn shape(&self) -> SpdShape<f64> {
        self.shape
    }

    /// Get the start wavelength of this SPD's [SpdShape].
    pub fn start(&self) -> f64 {
        self.shape.start
    }

    /// Get the end wavelength of this SPD's [SpdShape].
    pub fn end(&self) -> f64 {
        self.shape.end
    }

    /// Get the interval of this SPD's [SpdShape].
    pub fn interval(&self) -> Interval<f64> {
        self.shape.interval
    }

    /// Get the number of [Sample]s this SPD contains.
    pub fn len(&self) -> usize {
        self.samples.len()
    }

    // Get the first [Sample] this SPD contains
    pub fn first(&self) -> &Sample {
        self.samples.first().unwrap()
    }

    // Get the first [Sample] this SPD contains
    pub fn last(&self) -> &Sample {
        self.samples.last().unwrap()
    }

    /// Get an iterator over the SPD's [Sample]s.
    pub fn iter(&self) -> impl DoubleEndedIterator<Item = &Sample> {
        self.samples.iter()
    }

    /// Get a reference to the [Sample] vector.
    pub fn samples(&self) -> &Vec<Sample> {
        &self.samples
    }

    /// Get an iterator over this SPD's values
    pub fn values(&self) -> impl DoubleEndedIterator<Item = f64> + '_ {
        self.samples.iter().map(|s| s.v)
    }

    /// Get an iterator over this SPD's wavelengths
    pub fn wavelengths(&self) -> impl DoubleEndedIterator<Item = f64> + '_ {
        self.samples.iter().map(|s| s.nm)
    }

    /// Returns a new [VSPD] whose boundaries are the narrower of `self` and
    /// `shape`, interpolated to the interval given in `shape`
    pub fn interpolate(&self, mut shape: SpdShape<f64>) -> VSPD {
        let interp = InterpolatorSprague::<f64>::new(self);
        shape.start = shape.start.max(self.start());
        shape.end = shape.end.min(self.end());

        let mut samples = Vec::<Sample>::new();
        samples.extend(
            shape
                .iter()
                .map(|nm| Sample::new(nm.into(), interp.evaluate(nm.into()))),
        );

        VSPD { samples, shape }
    }

    /// Returns a new [VSPD] whose shape matches the supplied [SpdShape] by first
    /// interpolating then extrapolating
    pub fn align(&self, shape: SpdShape<f64>) -> VSPD {
        self.interpolate(shape).extrapolate(shape)
    }

    /// Create a new VSPD by extrapolating the boundaries of the domain of this
    /// VSPD to the given SpdShape. Note that the interval of the resulting VSPD
    /// is taken from self and the SpdShape's interval is ignored unless
    /// this VSPD has a varying interval
    /// # Panics
    /// Panics if both this VSPD's interval and the supplied SpdShape's interval
    /// are varying.
    pub fn extrapolate(&self, shape: SpdShape<f64>) -> VSPD {
        let extrap = ExtrapolatorConstant::new(self);
        let mut samples = Vec::<Sample>::new();
        let start = self.start().min(shape.start);
        let end = self.end().max(shape.end);
        let mut x = start;

        // use this SPD's interval unless it's varying, in which case use the
        // given shape's interval. If that is also varying, panic
        let interval = match self.shape.interval {
            Interval::Uniform(v) => v,
            Interval::Varying => match shape.interval {
                Interval::Uniform(v) => v,
                Interval::Varying => {
                    panic!("Cannot extrapolate without a uniform interval");
                }
            },
        };

        while x < self.start() {
            samples.push(Sample::new(x, extrap.evaluate(x)));
            x = x + interval;
        }
        samples.extend(self.samples.iter());
        x = self.end() + interval;
        while x <= end {
            samples.push(Sample::new(x, extrap.evaluate(x)));
            x = x + interval;
        }

        VSPD {
            samples,
            shape: SpdShape::new(shape.start, shape.end, interval),
        }
    }

    /// Trim this [VSPD] to lie inside the given [SpdShape].
    /// Note that this does not modify the spacing of samples in the SPD.
    /// If you want the boundaries of the new [SpdShape](struct.SpdShape.html) to be
    /// exactly those specified in `shape` you should use [interpolate](VSPD::interpolate) instead.
    pub fn trim(&self, shape: SpdShape<f64>) -> VSPD {
        let samples: Vec<Sample> = self
            .samples
            .iter()
            .skip_while(|s| s.nm < shape.start)
            .take_while(|s| s.nm <= shape.end)
            .map(|s| *s)
            .collect();

        let start = samples.first().unwrap().nm;
        let end = samples.last().unwrap().nm;

        VSPD {
            samples,
            shape: SpdShape::<f64> {
                start,
                end,
                interval: self.shape.interval,
            },
        }
    }

    /// Convert [VSPD] to an [XYZf64] using ASTM E308 method. The conversion
    /// method expects this SPD to have an interval of 1, 5, 10 or 20nm. If
    /// this SPD has any other intervals it will be copied and interpolated
    /// before conversion.
    /// # Arguments
    /// * `illuminant` - The reference illuminant to use, e.g. [static@illuminant::spd::D65]
    /// * `cmf` - The set of color-matching functions to use, e.g. [cmf::CIE_1931_2_DEGREE)
    /// # Returns
    /// An XYZf64 normalized to 100.0 as the perfect diffuser.
    pub fn to_xyz(&self, illuminant: &VSPD, cmf: &CMF) -> XYZf64 {
        // align the cmf and illum
        let illuminant = illuminant.align(SpdShape::new(360.0, 780.0, 1.0));
        let cmf = cmf.align(SpdShape::new(360.0, 780.0, 1.0));
        // first figure out our interval. If it's varying then we need to
        // interpolate to make it uniform
        match self.interval() {
            Interval::Varying => {
                let spd = self.align(SpdShape::new(
                    self.shape.start,
                    self.shape.end,
                    1.0,
                ));
                return spd_to_xyz_integration(
                    &spd,
                    &illuminant,
                    &cmf,
                    spd.shape,
                );
            }
            Interval::Uniform(interval) => {
                match interval.to_usize().unwrap() {
                    1 => {
                        // just integrate
                        spd_to_xyz_integration(
                            self,
                            &illuminant,
                            &cmf,
                            SpdShape::astm_e308(),
                        )
                    }
                    5 => {
                        // Integrate at 5nm
                        let mut shape = SpdShape::astm_e308();
                        shape.interval = Interval::Uniform(5.0);
                        spd_to_xyz_integration(self, &illuminant, &cmf, shape)
                    }
                    10 => {
                        // use ASTME308 weighting factors
                        spd_to_xyz_tristimulus_weighting_factors_astme308(
                            &self,
                            &illuminant,
                            &cmf,
                        )
                    }
                    // 20.0 => {
                    //     // do special thing we haven't implemented yet
                    // }
                    _ => {
                        println!(
                            "Interval must be 1, 5, 10 or 20nm, got: {}. Interpolating",
                            self.interval()
                        );
                        spd_to_xyz_integration(
                            self,
                            &illuminant,
                            &cmf,
                            SpdShape::astm_e308(),
                        )
                    }
                }
            }
        }
    }
}

fn calculate_interval(samples: &[Sample]) -> Interval<f64> {
    if samples.len() < 2 {
        panic!("Must have at least 2 samples");
    }
    let assumed_interval = samples[1].nm - samples[0].nm;
    for i in 1..samples.len() - 1 {
        // This is safe because we guarantee we're in bounds in the for loop
        let interval = unsafe {
            samples.get_unchecked(i).nm - samples.get_unchecked(i - 1).nm
        };
        if !interval.approx_eq(
            assumed_interval,
            F64Margin {
                ulps: 2,
                epsilon: 1.0e-11,
            },
        ) {
            return Interval::Varying;
        }
    }

    Interval::Uniform(assumed_interval)
}

fn calculate_shape(samples: &[Sample]) -> SpdShape<f64> {
    if samples.len() < 2 {
        panic!("Must have at least 2 samples");
    }

    let start = samples.first().unwrap().nm;
    let end = samples.last().unwrap().nm;
    // FIXME: try and round to integer wavelengths here?
    let interval = calculate_interval(samples);
    SpdShape::<f64> {
        start,
        end,
        interval,
    }
}

fn spd_to_xyz_integration(
    spd: &VSPD,
    illuminant: &VSPD,
    cmf: &CMF,
    shape: SpdShape<f64>,
) -> XYZf64 {
    // align everything to the default shape
    let cmf_x = cmf.x_bar.align(shape);
    let cmf_y = cmf.y_bar.align(shape);
    let cmf_z = cmf.z_bar.align(shape);
    let illuminant = illuminant.align(shape);
    let spd = spd.align(shape);

    // Since we're guaranteeing uniform SPDs the dw terms cancel out in the
    // integral, but we include them anyway since it keeps us closer to the
    // colour-science result
    let dw = match shape.interval {
        Interval::Uniform(i) => i,
        Interval::Varying => {
            panic!("Cannot integrate a varying VSPD");
        }
    };

    let k: f64 = 100.0f64
        / illuminant
            .values()
            .zip(cmf_y.values())
            .map(|(i, y)| i * y * dw)
            .sum::<f64>();

    let x = k * izip!(spd.values(), illuminant.values(), cmf_x.values())
        .map(|(s, i, c)| s * i * c * dw)
        .sum::<f64>();
    let y = k * izip!(spd.values(), illuminant.values(), cmf_y.values())
        .map(|(s, i, c)| s * i * c * dw)
        .sum::<f64>();
    let z = k * izip!(spd.values(), illuminant.values(), cmf_z.values())
        .map(|(s, i, c)| s * i * c * dw)
        .sum::<f64>();
    xyz(x, y, z)
}

fn spd_to_xyz_tristimulus_weighting_factors_astme308(
    spd: &VSPD,
    illuminant: &VSPD,
    cmf: &CMF,
) -> XYZf64 {
    // get interval - uniform only
    let interval = match spd.shape.interval {
        Interval::Uniform(i) => i,
        Interval::Varying => {
            panic!("sd_to_xyz_10nm requires a uniform SPD");
        }
    };

    // align illuminant to cmf
    let illuminant = if illuminant.shape != cmf.shape() {
        illuminant.align(cmf.shape())
    } else {
        illuminant.clone()
    };

    // trim spd to cmf boundaries
    let spd = spd.trim(cmf.shape());

    let w = tristimulus_weighting_factors_astme2022(
        &cmf,
        &illuminant,
        SpdShape::new(cmf.shape().start, cmf.shape().end, interval),
    );
    let start_w = cmf.shape().start;
    let end_w = cmf.shape().start + interval * (w.0.len() - 1) as f64;
    let w = adjust_tristimulus_weighting_factors_astme308(
        &w.0,
        &w.1,
        &w.2,
        SpdShape::new(start_w, end_w, interval),
        spd.shape,
    );

    let x =
        w.0.iter()
            .zip(spd.values())
            .map(|(w, r)| w * r)
            .sum::<f64>();
    let y =
        w.1.iter()
            .zip(spd.values())
            .map(|(w, r)| w * r)
            .sum::<f64>();
    let z =
        w.2.iter()
            .zip(spd.values())
            .map(|(w, r)| w * r)
            .sum::<f64>();

    xyz(x, y, z)
}

fn tristimulus_weighting_factors_astme2022(
    cmf: &CMF,
    illuminant: &VSPD,
    shape: SpdShape<f64>,
) -> (Vec<f64>, Vec<f64>, Vec<f64>) {
    // FIXME: should probably just interpolate to 1nm here
    let interval = match cmf.shape().interval {
        Interval::Uniform(i) => i,
        Interval::Varying => panic!(
            "tristimulus_weighting_factors_astme2022 requires uniform SPDs"
        ),
    };
    if interval != 1.0 {
        panic!("Interval must be 1");
    }
    if illuminant.shape.interval != cmf.shape().interval {
        panic!("CMF and illuminant intervals must match");
    }

    let y_x = cmf.x_bar.values().collect::<Vec<_>>();
    let y_y = cmf.y_bar.values().collect::<Vec<_>>();
    let y_z = cmf.z_bar.values().collect::<Vec<_>>();
    let s = illuminant.values().collect::<Vec<_>>();

    let interval = match shape.interval {
        Interval::Uniform(i) => i,
        Interval::Varying => panic!(
            "tristimulus_weighting_factors_astme2022 requires uniform SPDs"
        ),
    };

    // first and last measurement intervals lagrange coefficients
    let c_c = lagrange_coefficients_astm_e2022(interval, 3);
    // inner measurement intervals coefficients
    let c_b = lagrange_coefficients_astm_e2022(interval, 4);

    let interval_i = interval as usize;

    let mut w_x = s
        .iter()
        .step_by(interval_i)
        .zip(y_x.iter().step_by(interval_i))
        .map(|(s, c)| s * c)
        .collect::<Vec<_>>();
    let mut w_y = s
        .iter()
        .step_by(interval_i)
        .zip(y_y.iter().step_by(interval_i))
        .map(|(s, c)| s * c)
        .collect::<Vec<_>>();
    let mut w_z = s
        .iter()
        .step_by(interval_i)
        .zip(y_z.iter().step_by(interval_i))
        .map(|(s, c)| s * c)
        .collect::<Vec<_>>();

    // NOTE:
    // The below is pretty closely copied from colour-science for the sake
    // of correctness. We should really refactor this to be more idiomatic Rust

    // Total number of wavelengths
    let w_c = y_x.len();
    // Measurement interval interpolated values count
    let r_c = c_b.len();
    // Last interval first interpolated wavelength
    let w_lif = w_c - (w_c - 1) % (interval_i) - 1 - r_c;

    // Intervals count
    let i_c = w_x.len();
    let i_cm = i_c - 1;

    // First interval
    for j in 0..r_c {
        for k in 0..3 {
            w_x[k] = w_x[k] + c_c[j][k] * s[j + 1] * y_x[j + 1];
            w_y[k] = w_y[k] + c_c[j][k] * s[j + 1] * y_y[j + 1];
            w_z[k] = w_z[k] + c_c[j][k] * s[j + 1] * y_z[j + 1];
        }
    }

    // Last interval
    for j in 0..r_c {
        for k in (i_cm - 2..i_cm + 1).rev() {
            w_x[k] = w_x[k]
                + c_c[r_c - j - 1][i_cm - k] * s[j + w_lif] * y_x[j + w_lif];
            w_y[k] = w_y[k]
                + c_c[r_c - j - 1][i_cm - k] * s[j + w_lif] * y_y[j + w_lif];
            w_z[k] = w_z[k]
                + c_c[r_c - j - 1][i_cm - k] * s[j + w_lif] * y_z[j + w_lif];
        }
    }

    // intermediate intervals
    for j in 0..(i_c - 3) {
        for k in 0..r_c {
            let w_i = (r_c + 1) * (j + 1) + 1 + k;

            w_x[j + 0] = w_x[j + 0] + c_b[k][0] * s[w_i] * y_x[w_i];
            w_x[j + 1] = w_x[j + 1] + c_b[k][1] * s[w_i] * y_x[w_i];
            w_x[j + 2] = w_x[j + 2] + c_b[k][2] * s[w_i] * y_x[w_i];
            w_x[j + 3] = w_x[j + 3] + c_b[k][3] * s[w_i] * y_x[w_i];

            w_y[j + 0] = w_y[j + 0] + c_b[k][0] * s[w_i] * y_y[w_i];
            w_y[j + 1] = w_y[j + 1] + c_b[k][1] * s[w_i] * y_y[w_i];
            w_y[j + 2] = w_y[j + 2] + c_b[k][2] * s[w_i] * y_y[w_i];
            w_y[j + 3] = w_y[j + 3] + c_b[k][3] * s[w_i] * y_y[w_i];

            w_z[j + 0] = w_z[j + 0] + c_b[k][0] * s[w_i] * y_z[w_i];
            w_z[j + 1] = w_z[j + 1] + c_b[k][1] * s[w_i] * y_z[w_i];
            w_z[j + 2] = w_z[j + 2] + c_b[k][2] * s[w_i] * y_z[w_i];
            w_z[j + 3] = w_z[j + 3] + c_b[k][3] * s[w_i] * y_z[w_i];
        }
    }

    // extrapolation of potentially incomplete interval
    for j in (w_c - ((w_c - 1) % interval_i))..w_c {
        w_x[i_cm] = w_x[i_cm] + s[j] * y_x[j];
        w_y[i_cm] = w_y[i_cm] + s[j] * y_y[j];
        w_z[i_cm] = w_z[i_cm] + s[j] * y_z[j];
    }

    let k: f64 = 100.0 / w_y.iter().sum::<f64>();

    w_x.iter_mut().map(|x| *x = *x * k).all(|_| true);
    w_y.iter_mut().map(|x| *x = *x * k).all(|_| true);
    w_z.iter_mut().map(|x| *x = *x * k).all(|_| true);

    (w_x, w_y, w_z)
}

fn adjust_tristimulus_weighting_factors_astme308(
    w_x: &[f64],
    w_y: &[f64],
    w_z: &[f64],
    shape_r: SpdShape<f64>,
    shape_t: SpdShape<f64>,
) -> (Vec<f64>, Vec<f64>, Vec<f64>) {
    let r_interval = match shape_r.interval {
        Interval::Uniform(i) => i,
        Interval::Varying => panic!("shape must be uniform"),
    };

    let mut w_x = w_x.to_vec();
    let mut w_y = w_y.to_vec();
    let mut w_z = w_z.to_vec();

    let start_index = ((shape_t.start - shape_r.start) / r_interval) as usize;
    for i in 0..start_index {
        w_x[start_index] += w_x[i];
        w_y[start_index] += w_y[i];
        w_z[start_index] += w_z[i];
    }

    let end_index = ((shape_r.end - shape_t.end) / r_interval) as usize;
    let n = w_x.len();
    for i in 0..end_index {
        w_x[n - end_index - 1] += w_x[n - i - 1];
        w_y[n - end_index - 1] += w_y[n - i - 1];
        w_z[n - end_index - 1] += w_z[n - i - 1];
    }

    let first = start_index;
    let last = n - end_index;

    (
        w_x[first..last].to_vec(),
        w_y[first..last].to_vec(),
        w_z[first..last].to_vec(),
    )
}

fn lagrange_coefficients(r: f64, n: usize) -> Vec<f64> {
    let mut l_j = vec![1.0; n];
    for j in 0..n {
        for i in 0..n {
            if i != j {
                l_j[j] *= (r - i as f64) / (j as f64 - i as f64);
            }
        }
    }
    l_j
}

pub struct FloatRange {
    current: usize,
    steps: usize,
    start: f64,
    delta: f64,
}

impl Iterator for FloatRange {
    type Item = f64;
    fn next(&mut self) -> Option<f64> {
        if self.current < self.steps {
            self.current += 1;
            Some((self.current - 1) as f64 * self.delta + self.start)
        } else {
            None
        }
    }
}

pub fn linspace(start: f64, end: f64, steps: usize) -> FloatRange {
    let delta = (end - start) / (steps - 1) as f64;
    FloatRange {
        current: 0,
        steps,
        start,
        delta,
    }
}

/// Compute the Lagrange coefficients for given interval size using
/// ASTM E2022-11 method
pub fn lagrange_coefficients_astm_e2022(
    interval: f64,
    degree: usize,
) -> Vec<Vec<f64>> {
    let num = interval as usize - 1;
    let d = if degree == 4 { 1.0 } else { 0.0 };

    linspace(1.0 / interval, 1.0 - (1.0 / interval), num)
        .map(|r| lagrange_coefficients(r + d, degree))
        .collect::<Vec<_>>()
}

impl FromIterator<Sample> for VSPD {
    fn from_iter<I: IntoIterator<Item = Sample>>(iter: I) -> VSPD {
        let mut samples: Vec<Sample> = Vec::new();
        for i in iter {
            samples.push(i);
        }
        VSPD::new(samples)
    }
}

impl FromIterator<(f64, f64)> for VSPD {
    fn from_iter<I: IntoIterator<Item = (f64, f64)>>(iter: I) -> VSPD {
        let mut samples: Vec<Sample> = Vec::new();
        for i in iter {
            samples.push(Sample::new(i.0, i.1));
        }
        VSPD::new(samples)
    }
}

impl<'a> ApproxEq for &'a VSPD {
    type Margin = F64Margin;
    fn approx_eq<T: Into<Self::Margin>>(self, other: Self, margin: T) -> bool {
        let margin = margin.into();
        self.samples.len() == other.samples.len()
            && self
                .samples
                .iter()
                .zip(other.samples.iter())
                .all(|(l, r)| l.approx_eq(*r, margin))
    }
}

impl std::ops::Div<f64> for VSPD {
    type Output = Self;

    fn div(self, rhs: f64) -> VSPD {
        self.samples().iter().map(|s| { Sample {nm: s.nm, v: s.v / rhs}}).collect()
    }
}

impl std::ops::Div<f64> for &VSPD {
    type Output = VSPD;

    fn div(self, rhs: f64) -> VSPD {
        self.samples().iter().map(|s| { Sample {nm: s.nm, v: s.v / rhs}}).collect()
    }
}

impl std::ops::Mul<f64> for VSPD {
    type Output = Self;

    fn mul(self, rhs: f64) -> VSPD {
        self.samples().iter().map(|s| { Sample {nm: s.nm, v: s.v * rhs}}).collect()
    }
}

impl std::ops::Mul<f64> for &VSPD {
    type Output = VSPD;

    fn mul(self, rhs: f64) -> VSPD {
        self.samples().iter().map(|s| { Sample {nm: s.nm, v: s.v * rhs}}).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{cmf, colorchecker, illuminant};
    #[test]
    fn macro_initialization() {
        let spd = VSPD::new(vec![
            Sample::new(380.0, 0.5),
            Sample::new(400.0, 0.4),
            Sample::new(420.0, 0.3),
            Sample::new(440.0, 0.2),
            Sample::new(460.0, 0.1),
            Sample::new(480.0, 0.0),
        ]);
        let mspd = vspd!(
            380.0f64 => 0.5,
            400.0 => 0.4,
            420.0 => 0.3,
            440.0 => 0.2,
            460.0 => 0.1,
            480.0 => 0.0,
        );
        assert_eq!(spd, mspd);
    }

    #[test]
    fn interpolate() {
        let spd = vspd!(
            380.0f64 => 0.5,
            400.0 => 0.4,
            420.0 => 0.3,
            440.0 => 0.2,
            460.0 => 0.1,
            480.0 => 0.0,
        );

        let interp = InterpolatorSprague::<f64>::new(&spd);
        assert_eq!(interp.evaluate(390.0), 0.45);

        let spd2 = spd.interpolate(SpdShape::new(380.0, 480.0, 10.0));
        let target = vspd!(
            380.0f64 => 0.5,
            390.0f64 => 0.45,
            400.0f64 => 0.4,
            410.0f64 => 0.35,
            420.0f64 => 0.3,
            430.0f64 => 0.25,
            440.0f64 => 0.2,
            450.0f64 => 0.15,
            460.0f64 => 0.1,
            470.0f64 => 0.05,
            480.0f64 => 0.0,
        );

        assert!(spd2.approx_eq(
            &target,
            F64Margin {
                ulps: 2,
                epsilon: 1e-15
            }
        ));
    }

    #[test]
    fn extrapolate() {
        let spd = vspd!(
            380.0f64 => 0.5,
            400.0 => 0.4,
            420.0 => 0.3,
            440.0 => 0.2,
            460.0 => 0.1,
            480.0 => 0.0,
        );

        let spd3 = spd.extrapolate(SpdShape::new(320.0, 520.0, 10.0));
        assert_eq!(
            vspd!(
                320.0 => 0.5,
                340.0 => 0.5,
                360.0 => 0.5,
                380.0 => 0.5,
                400.0 => 0.4,
                420.0 => 0.3,
                440.0 => 0.2,
                460.0 => 0.1,
                480.0 => 0.0,
                500.0 => 0.0,
                520.0 => 0.0,
            ),
            spd3
        );
    }

    #[test]
    fn trim() {
        let spd = vspd!(
            380.0f64 => 0.5,
            400.0 => 0.4,
            420.0 => 0.3,
            440.0 => 0.2,
            460.0 => 0.1,
            480.0 => 0.0,
        );

        let spd3 = spd.trim(SpdShape::new(400.0, 440.0, 10.0));
        assert_eq!(
            vspd!(
                400.0 => 0.4,
                420.0 => 0.3,
                440.0 => 0.2,
            ),
            spd3
        );

        let spd3 = spd.trim(SpdShape::new(390.0, 450.0, 10.0));
        assert_eq!(
            vspd!(
                400.0 => 0.4,
                420.0 => 0.3,
                440.0 => 0.2,
            ),
            spd3
        );
    }

    #[test]
    fn to_xyz() {
        // check at 1nm
        let xyz = spd_to_xyz_integration(
            &colorchecker::DARK_SKIN,
            &illuminant::spd::D65,
            &cmf::CIE_1931_2_DEGREE,
            SpdShape::astm_e308(),
        );
        assert!(xyz.approx_eq(
            XYZf64::new(11.14725784521762, 10.072542226497, 6.8048713133720),
            F64Margin {
                ulps: 2,
                epsilon: 1e-11
            }
        ));

        // check at 5nm
        let spd = colorchecker::DARK_SKIN.clone();
        let spd = spd.interpolate(SpdShape::new(spd.start(), spd.end(), 5.0));
        let xyz = spd.to_xyz(&illuminant::spd::D65, &cmf::CIE_1931_2_DEGREE);
        assert!(XYZf64::new(
            11.14726060385657824269856064347550,
            10.07254417119669,
            6.80486371314964,
        )
        .approx_eq(
            xyz,
            F64Margin {
                ulps: 2,
                epsilon: 1.0e-13
            }
        ));

        // check at 10nm
        let spd = colorchecker::DARK_SKIN.clone();
        let spd = spd.align(SpdShape::new(380.0, 730.0, 10.0));
        let xyz = spd_to_xyz_tristimulus_weighting_factors_astme308(
            &spd,
            &illuminant::spd::D65.align(SpdShape::new(360.0, 780.0, 1.0)),
            &cmf::CIE_1931_2_DEGREE.align(SpdShape::new(360.0, 780.0, 1.0)),
        );
        assert!(XYZf64::new(
            11.14724658576002802590210194466636,
            10.07258885098873690822074422612786,
            6.80485137438652287755758152343333,
        )
        .approx_eq(
            xyz,
            F64Margin {
                epsilon: 1.0e-14,
                ulps: 1
            }
        ));

        let xyz = spd.to_xyz(
            &illuminant::spd::D65.align(SpdShape::new(360.0, 780.0, 1.0)),
            &cmf::CIE_1931_2_DEGREE.align(SpdShape::new(360.0, 780.0, 1.0)),
        );
        assert!(XYZf64::new(
            11.14724658576002802590210194466636,
            10.07258885098873690822074422612786,
            6.80485137438652287755758152343333,
        )
        .approx_eq(
            xyz,
            F64Margin {
                epsilon: 1.0e-14,
                ulps: 1
            }
        ));
    }

    #[test]
    fn lagrange_coeff() {
        let ln =
            linspace(1.0 / 10.0, 1.0 - (1.0 / 10.0), 9).collect::<Vec<_>>();
        println!("{:?}", ln);

        let c = lagrange_coefficients(0.1, 4);
        println!("{:?}", c);
        assert!(c
            .iter()
            .zip(vec![0.8265, 0.2755, -0.1305, 0.0285])
            .all(|(a, b)| a.approx_eq(b, (0.0, 1))));

        let c = lagrange_coefficients_astm_e2022(10.0, 4);
        println!("{:?}", c);

        let c = lagrange_coefficients_astm_e2022(10.0, 3);
        println!("{:?}", c);
    }

    #[test]
    fn weighting() {
        let d65 = illuminant::spd::D65.align(SpdShape::astm_e308());
        let cmf = cmf::CIE_1931_2_DEGREE.align(SpdShape::astm_e308());
        let w = tristimulus_weighting_factors_astme2022(
            &cmf,
            &d65,
            SpdShape::new(360.0, 780.0, 20.0),
        );
        for (x, y, z) in izip!(w.0.iter(), w.1.iter(), w.2.iter()) {
            println!("{}, {}, {}", x, y, z);
        }

        let w = adjust_tristimulus_weighting_factors_astme308(
            &w.0,
            &w.1,
            &w.2,
            SpdShape::new(360.0, 780.0, 20.0),
            SpdShape::new(400.0, 700.0, 20.0),
        );
        for (x, y, z) in izip!(w.0.iter(), w.1.iter(), w.2.iter()) {
            println!("{}, {}, {}", x, y, z);
        }
    }

    #[test]
    fn checker_xyz() {
        for (name, ref_xyz) in colorchecker::XYZ_D65.iter() {
            let spd = &colorchecker::SPECTRAL[name];
            let xyz =
                spd.to_xyz(&illuminant::spd::D65, &cmf::CIE_1931_2_DEGREE);
            assert!(ref_xyz.approx_eq(
                xyz,
                F64Margin {
                    epsilon: 1.0e-14,
                    ulps: 2
                }
            ));
        }
    }
}
