#![recursion_limit = "128"]

#[macro_use]
extern crate derive_more;

#[macro_use]
extern crate maplit;

#[macro_use]
pub mod macros;

pub mod cmf;
pub use cmf::CMF;

pub mod interpolation;
pub use interpolation::InterpolatorLinear;
pub use interpolation::InterpolatorSprague;

pub mod colorchecker;

pub mod illuminant;

pub mod xyz;
pub use xyz::{XYZf32, XYZf64};

pub mod rgb;
pub use rgb::{RGBAf32, RGBf32, RGBf64, RGBu16, RGBu8};

pub mod math;
pub use math::{M3f32, M3f64, Matrix33};

pub mod color_space_rgb;
pub use color_space_rgb::{decode, encode, model_f64::*, ColorSpaceRGB};

pub mod chromaticity;
pub use chromaticity::*;

pub mod chromatic_adaptation;

pub mod vspd;
pub use vspd::{SpdElement, SpdShape, VSPD};

pub mod uplifting;

pub mod spd;
pub use spd::SPD;

pub mod transform;
pub use transform::*;

pub mod lab;
pub use lab::delta_E_2000 as delta_E;
pub use lab::{lab, xyz_to_lab, Lab};

pub mod photometry;
pub use photometry::spd_to_lumens;
