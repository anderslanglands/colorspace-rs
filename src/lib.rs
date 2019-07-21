#![recursion_limit="128"]

#[macro_use]
extern crate derive_more;

#[macro_use]
extern crate maplit;

#[macro_use]
pub mod macros;

pub mod cmf;
pub use cmf::CMF;

pub mod interpolation;
pub use interpolation::InterpolatorSprague;
pub use interpolation::InterpolatorLinear;

pub mod colorchecker;

pub mod illuminant;

pub mod xyz;
pub use xyz::{XYZf32, XYZf64};

pub mod rgb;
pub use rgb::{RGBf64, RGBf32, RGBu16, RGBu8};

pub mod math;
pub use math::{Matrix33, M3f64, M3f32};

pub mod color_space_rgb;
pub use color_space_rgb::{ColorSpaceRGB, encode, decode, model_f64::*};

pub mod chromaticity;
pub use chromaticity::*;

pub mod chromatic_adaptation;

pub mod vspd;
pub use vspd::{VSPD, SpdElement, SpdShape};

pub mod uplifting;

pub mod spd;

pub mod transform;
pub use transform::*;

pub mod lab;
pub use lab::{Lab, lab, xyz_to_lab};
pub use lab::delta_E_2000 as delta_E;

pub mod photometry;
pub use photometry::spd_to_lumens;