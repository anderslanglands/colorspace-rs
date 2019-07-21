use crate::{VSPD, SpdElement};

pub struct InterpolatorSprague<T>
where
    T: SpdElement,
{
    x: Vec<T>,
    y: Vec<T>,
}

pub trait SpragueCoefficients {
    type Item: SpdElement;
    fn coeff_c0() -> [Self::Item; 6];
    fn coeff_c1() -> [Self::Item; 6];
    fn coeff_c2() -> [Self::Item; 6];
    fn coeff_c3() -> [Self::Item; 6];

    fn coeff_a(r: &Vec<Self::Item>, i: usize) -> [Self::Item; 6];
}

impl SpragueCoefficients for f32 {
    type Item = f32;
    fn coeff_c0() -> [f32; 6] {
        [884.0, -1960.0, 3033.0, -2648.0, 1080.0, -180.0]
    }
    fn coeff_c1() -> [f32; 6] {
        [508.0, -540.0, 488.0, -367.0, 144.0, -24.0]
    }
    fn coeff_c2() -> [f32; 6] {
        [-24.0, 144.0, -367.0, 488.0, -540.0, 508.0]
    }
    fn coeff_c3() -> [f32; 6] {
        [-180.0, 1080.0, -2648.0, 3033.0, -1960.0, 884.0]
    }
    fn coeff_a(r: &Vec<f32>, i: usize) -> [f32; 6] {
        let a0p = r[i];
        let a1p = (2.0 * r[i - 2] - 16.0 * r[i - 1] + 16.0 * r[i + 1] - 2.0 * r[i + 2]) / 24.0;
        let a2p = (-r[i - 2] + 16.0 * r[i - 1] - 30.0 * r[i] + 16.0 * r[i + 1] - r[i + 2]) / 24.0;
        let a3p = (-9.0 * r[i - 2] + 39.0 * r[i - 1] - 70.0 * r[i] + 66.0 * r[i + 1]
            - 33.0 * r[i + 2]
            + 7.0 * r[i + 3])
            / 24.0;
        let a4p = (13.0 * r[i - 2] - 64.0 * r[i - 1] + 126.0 * r[i] - 124.0 * r[i + 1]
            + 61.0 * r[i + 2]
            - 12.0 * r[i + 3])
            / 24.0;
        let a5p = (-5.0 * r[i - 2] + 25.0 * r[i - 1] - 50.0 * r[i] + 50.0 * r[i + 1]
            - 25.0 * r[i + 2]
            + 5.0 * r[i + 3])
            / 24.0;

        [a0p, a1p, a2p, a3p, a4p, a5p]
    }
}

impl SpragueCoefficients for f64 {
    type Item = f64;
    fn coeff_c0() -> [f64; 6] {
        [884.0, -1960.0, 3033.0, -2648.0, 1080.0, -180.0]
    }
    fn coeff_c1() -> [f64; 6] {
        [508.0, -540.0, 488.0, -367.0, 144.0, -24.0]
    }
    fn coeff_c2() -> [f64; 6] {
        [-24.0, 144.0, -367.0, 488.0, -540.0, 508.0]
    }
    fn coeff_c3() -> [f64; 6] {
        [-180.0, 1080.0, -2648.0, 3033.0, -1960.0, 884.0]
    }
    fn coeff_a(r: &Vec<f64>, i: usize) -> [f64; 6] {
        let a0p = r[i];
        let a1p = (2.0 * r[i - 2] - 16.0 * r[i - 1] + 16.0 * r[i + 1] - 2.0 * r[i + 2]) / 24.0;
        let a2p = (-r[i - 2] + 16.0 * r[i - 1] - 30.0 * r[i] + 16.0 * r[i + 1] - r[i + 2]) / 24.0;
        let a3p = (-9.0 * r[i - 2] + 39.0 * r[i - 1] - 70.0 * r[i] + 66.0 * r[i + 1]
            - 33.0 * r[i + 2]
            + 7.0 * r[i + 3])
            / 24.0;
        let a4p = (13.0 * r[i - 2] - 64.0 * r[i - 1] + 126.0 * r[i] - 124.0 * r[i + 1]
            + 61.0 * r[i + 2]
            - 12.0 * r[i + 3])
            / 24.0;
        let a5p = (-5.0 * r[i - 2] + 25.0 * r[i - 1] - 50.0 * r[i] + 50.0 * r[i + 1]
            - 25.0 * r[i + 2]
            + 5.0 * r[i + 3])
            / 24.0;

        [a0p, a1p, a2p, a3p, a4p, a5p]
    }
}

impl<T> InterpolatorSprague<T>
where
    T: SpdElement + SpragueCoefficients<Item = T>,
{
    pub fn new(vspd: &VSPD) -> InterpolatorSprague<f64> {
        // FIXME: take only a uniform SPD here (USPD?) rather than assuming
        // this is one
        let first = vspd.samples.first().unwrap().nm;
        let last = vspd.samples.last().unwrap().nm;
        let interval = vspd.samples[1].nm - first;
        let x1 = first - interval * 2.0;
        let x2 = first - interval;
        let x3 = last + interval;
        let x4 = last + interval * 2.0;

        let mut x = Vec::with_capacity(vspd.len() + 4);
        x.push(x1);
        x.push(x2);
        x.extend(vspd.iter().map(|s| s.nm));
        x.push(x3);
        x.push(x4);

        let mut y = Vec::with_capacity(vspd.len() + 4);

        let y1 = f64::coeff_c0()
            .iter()
            .zip(vspd.iter())
            .map(|(c, s)| *c * s.v)
            .sum::<f64>();

        let y2 = f64::coeff_c1()
            .iter()
            .zip(vspd.iter())
            .map(|(c, s)| *c * s.v)
            .sum::<f64>();

        let y3 = f64::coeff_c2()
            .iter()
            .rev()
            .zip(vspd.iter().rev())
            .map(|(c, s)| *c * s.v)
            .sum::<f64>();

        let y4 = f64::coeff_c3()
            .iter()
            .rev()
            .zip(vspd.iter().rev())
            .map(|(c, s)| *c * s.v)
            .sum::<f64>();

        y.push(y1 / 209.0);
        y.push(y2 / 209.0);
        y.extend(vspd.iter().map(|s| s.v));
        y.push(y3 / 209.0);
        y.push(y4 / 209.0);

        InterpolatorSprague { x, y }
    }

    pub fn evaluate(&self, x: T) -> T {
        let i = (self.x.iter().position(|t| x < *t).unwrap() - 1)
            .max(2)
            .min(self.x.len() - 4);
        let dx = (x - self.x[i]) / (self.x[i + 1] - self.x[i]);

        let a = T::coeff_a(&self.y, i);

        a[0] + a[1] * dx
            + a[2] * dx.powi(2)
            + a[3] * dx.powi(3)
            + a[4] * dx.powi(4)
            + a[5] * dx.powi(5)
    }

}

pub struct ExtrapolatorConstant<'a> {
    spd: &'a VSPD,
}

impl<'a> ExtrapolatorConstant<'a> {
    pub fn new(spd: &'a VSPD) -> ExtrapolatorConstant<'a> {
        ExtrapolatorConstant { spd }
    }

    // FIXME: what do we do if given a wavelength that's in domain?
    pub fn evaluate(&self, x: f64) -> f64 {
        if x < self.spd.samples.first().unwrap().nm {
            self.spd.samples.first().unwrap().v
        } else {
            self.spd.samples.last().unwrap().v
        }
    }
}

pub struct InterpolatorLinear<'a> {
    spd: &'a VSPD,
}

impl<'a> InterpolatorLinear<'a> {
    pub fn new(spd: &'a VSPD) -> InterpolatorLinear<'a> {
        InterpolatorLinear { spd }
    }

    pub fn evaluate(&self, x: f64) -> f64 {
        if x <= self.spd.first().nm {
            self.spd.first().v
        } else if x >= self.spd.last().nm {
            self.spd.last().v
        } else {
            let i = self.spd.iter().position(|s| x < s.nm).unwrap() - 1;
            let d = (x - self.spd.samples()[i].nm) / (self.spd.samples()[i+1].nm - self.spd.samples()[i].nm);
            (1.0 - d) * self.spd.samples()[i].v + d * self.spd.samples()[i+1].v 
        }
    }
}

