//!
//! [![Build Status](https://travis-ci.com/anderslanglands/colorspace-rs.svg?branch=master)](https://travis-ci.com/anderslanglands/colorspace-rs)
//!
//! # colorspace
//!
//!  A crate for colorimetry in Rust.
//!  This crate contains types and functions for working with color. The intended
//!  use is to support rendering applications (I use it to manage color in a spectral pathtracer), but if you want to be able to
//!  convert between spectral, XYZ, L'a'b' and RGB spaces of various flavors such as
//!  sRGB, ACES, DCI P3 and ALEXA Wide Gamut then this is the crate for you.
//!  
//!  ## Types
//!  ### Tristimulus
//!  The library contains two main types for working with color values: [xyz::XYZ] and [rgb::RGBf32]. These are both 32-bit floating point and have common component-wise math operations defined for them.
//!
//!  The [rgb::RGBu8] and [rgb::RGBu16] types are for storage only (for writing to images or passing to e.g. OpenGL for display) and do not define any operations. In order to perform mathematical operations on these types you should convert them to [rgb::RGBf32] first.
//!
//!  ### Spectral Power Distribution
//!  The main type for working with SPDs is [vspd::VSPD] (for Varying SPD). This is a double-precision vector or (wavelength, value) sample pairs. The library supplies spectral data for CIE illuminants in the [illuminant] module, and for the color checker chart in the [colorchecker] module.
//!  
//!  ## Examples
//!  ### Spectral to 8-bit, gamma-encoded sRGB conversion
//!  ```rust
//! use colorspace::*;
//!
//!  // Convert the spectral data for a measured MacBeth chart swatch to XYZ
//!  // using the CIE 1931 2-degree CMFs and a D65 illuminant
//!  let xyz = colorchecker::SPECTRAL["dark_skin"]
//!      .to_xyz(&illuminant::spd::D65, &cmf::CIE_1931_2_DEGREE);
//!
//!  // Convert the XYZ value to scene-referred (i.e. linear) sRGB
//! let model_srgb = &color_space_rgb::model_f64::SRGB;
//!  let xf_xyz_to_srgb = xyz_to_rgb_matrix(model_srgb.white, model_srgb);
//!  let rgb = xyz_to_rgb(&xf_xyz_to_srgb, xyz);
//!
//!  // Convert the scene-referred sRGB value to an 8-bit, display-referred
//!  // value by applying the opto-electrical transfer function and using RGBu8's
//!  // From<RGBf32> impl
//!  let rgb: RGBu8 = (model_srgb.oetf)(rgb).into();
//!
//!  assert_eq!(rgb, rgbu8(115, 82, 68));
//!  ```
//!  
//!  ## Licence
//!  colorspace is licensed under Apache License, Version 2.0
//!  http://www.apache.org/licenses/LICENSE-2.0
//!  
//!  This crate contains some data taken from the excellent colour-science python
//!  library by Mansencal et al.: <https://www.colour-science.org>
//!  Copyright (c) 2013-2018, Colour Developers
//!  
//!  Most of the conversion algorithms are based on those published at
//!  Bruce Lindbloom's site: <http://www.brucelindbloom.com>
//!  Copyright © 2001 - 2018 Bruce Justin Lindbloom.
//!  
//!  BabelColor color-checker data is copyright © 2004‐2012 Danny Pascale (www.babelcolor.com); used with permission.
//!  <http://www.babelcolor.com/index_htm_files/ColorChecker_RGB_and_spectra.xls>
//!  <http://www.babelcolor.com/index_htm_files/ColorChecker_RGB_and_spectra.zip>
//!
//! Contains data from https://github.com/imallett/simple-spectral accompanying the EGSR 2019 paper "Mallett & Yuksel - Spectral Primary Decomposition for Rendering with sRGB Reflectance".
//!

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
pub use rgb::{
    rgbf32, rgbf64, rgbu16, rgbu8, RGBAf32, RGBf32, RGBf64, RGBu16, RGBu8,
};

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
