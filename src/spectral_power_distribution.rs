use std::iter::{Iterator, Map};
use std::ops::{Index, IndexMut};

/// Distribution of the spectral data. Some algorithms can be optimized if it
/// is known that the samples are evenly distributed
#[derive(Copy, Clone)]
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
/// pairs.
pub struct SPD {
    samples: Vec<(f32, f32)>,
    distribution: Distribution,
}

impl SPD {
    pub fn new(samples: &[(f32, f32)]) -> SPD {
        let samples = samples.to_vec();
        let distribution = calculate_distribution(&samples);
        SPD {
            samples,
            distribution,
        }
    }

    pub fn consume(samples: Vec<(f32, f32)>) -> SPD {
        let distribution = calculate_distribution(&samples);
        SPD {
            samples,
            distribution,
        }
    }

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

    pub fn start(&self) -> f32 {
        self.samples.first().unwrap().0
    }

    pub fn end(&self) -> f32 {
        self.samples.last().unwrap().0
    }

    pub fn range(&self) -> f32 {
        self.end() - self.start()
    }

    pub fn num_samples(&self) -> usize {
        self.samples.len()
    }

    pub fn distribution(&self) -> Distribution {
        self.distribution
    }

    /// Interpolates the value for `lambda` from the SPD. If `lambda` is
    /// outside of the range of the SPD, it is clamped to lie within the range.
    pub fn value_at(&self, lambda: f32) -> f32 {
        use num::clamp;
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

    pub fn samples(&self) -> &Vec<(f32, f32)> {
        &self.samples
    }
}

impl Index<usize> for SPD {
    type Output = (f32, f32);
    fn index(&self, index: usize) -> &(f32, f32) {
        &self.samples[index]
    }
}

// impl IndexMut<usize> for SPD {
//     fn index_mut(&mut self, index: usize) -> &mut (f32, f32) {
//         &mut self.samples[index]
//     }
// }