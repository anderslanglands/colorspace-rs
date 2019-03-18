//! # colorspace

//!  A crate for colorimetry in Rust.
//!  This crate contains types and functions for working with color. The intended
//!  use is to support rendering applications (I use it to manage color in a spectral pathtracer), but if you want to be able to
//!  convert between spectral, XYZ, L'a'b' and RGB spaces of various flavors such as
//!  sRGB, ACES, DCI P3 and ALEXA Wide Gamut then this is the crate for you.

//!  Note that currently the results are not as accurate as they could be, due
//!  to the spectral->XYZ conversion not being implemented according to spec,
//!  but the results should be "good enough" for casual visual inspection. Be
//!  aware that future versions of the library will change some decimal places
//!  as the accuracy is improved.

//!  ## Types
//!  ### Tristimulus
//!  The library contains two main types for working with color values: `XYZ` and `RGBf32`. These are both 32-bit floating point and have common component-wise math operations defined for them.

//!  The `RGBu8` and `RGBu16` types are for storage only (for writing to images or passing to e.g. OpenGL for display) and do not define any operations. Additionally, `RGBf16` is provided if the `f16` feature is enabled. In order to perform mathematical operations on these types you should convert them to `RGBf32` first.

//!  ### Spectral Power Distribution
//!  `SPD`s are defined as a pair of `Vec`s of wavelengths and associated values. The library supplies spectral data for CIE illuminants in the `illuminant` module, and for the color checker chart in the `color_checker` module.

//!  ## Examples
//!  ### Spectral to 8-bit, gamma-encoded sRGB conversion
//!  ```rust
//!  // Definition of the sRGB color space
//!  use colorspace::color_space_rgb::sRGB;
//!  // The prelude brings in common types
//!  use colorspace::prelude::*;
//!  // Convert the spectral data for a measured MacBeth chart swatch to XYZ
//!  // using the CIE 1931 2-degree CMFs and a D65 illuminant
//!  let xyz = babel_average::spd["dark_skin"]
//!      .to_xyz_with_illuminant(&illuminant::D65.spd);
//!  // Convert the XYZ value to scene-referred (i.e. linear) sRGB
//!  let xf_xyz_to_srgb = xyz_to_rgb_matrix(sRGB.white, &sRGB);
//!  let rgb = xyz_to_rgb(&xf_xyz_to_srgb, xyz);
//!  // Convert the scene-referred sRGB value to an 8-bit, display-referred
//!  // value by applying the opto-electrical transfer function and using RGBu8's
//!  // From<RGBf32> impl
//!  let rgb: RGBu8 = (sRGB.oetf)(rgb).into();
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

pub mod chromatic_adaptation;
pub mod chromaticity;
pub mod cmf;
pub mod color_checker;
pub mod color_space_rgb;
pub mod illuminant;
pub mod lab;
pub mod math;
pub mod prelude;
pub mod rgb;
pub mod spd;
pub mod spd_conversion;
mod traits;
pub mod transform;
pub mod xyz;
pub use crate::color_space_rgb::{
    sRGB, ACEScg, AlexaWide, DCI_P3, ITUR_BT2020, ITUR_BT709,
};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        use super::rgb::*;
        use super::spd::*;

        let c1 = RGBf32::from_scalar(0.18);
        assert!(c1.r == 0.18 && c1.g == 0.18 && c1.b == 0.18);

        let s1 = SPD::consume(vec![
            (400.0, 1.0),
            (500.0, 2.0),
            (600.0, 3.0),
            (700.0, 4.0),
        ]);

        assert_eq!(s1.value_at(450.0), 1.5);
        assert_eq!(s1.value_at(380.0), 1.0);
        assert_eq!(s1.value_at(700.0), 4.0);
        assert_eq!(s1.value_at(720.0), 4.0);
    }

    #[test]
    fn spectral_to_rgb_conversion() {
        use crate as colorspace;
        use crate::prelude::*;

        let xyz = babel_average::spd["dark_skin"]
            .to_xyz_with_illuminant(&illuminant::D65.spd);

        let xf_xyz_to_rec709 = xyz_to_rgb_matrix(
            colorspace::ITUR_BT709.white,
            &colorspace::ITUR_BT709,
        );

        let ones_xyz = illuminant::E
            .spd
            .to_xyz_with_illuminant(&illuminant::D65.spd);
        let ones_rgb = xyz_to_rgb(&xf_xyz_to_rec709, ones_xyz);
        println!("ones_rgb: {}", ones_rgb);

        let cat_d65_to_d50 = crate::chromatic_adaptation::bradford(
            illuminant::D65.xyz,
            illuminant::D50.xyz,
        );

        for (name, ref spd) in &*babel_average::spd {
            let xyz = spd.to_xyz_with_illuminant(&illuminant::D65.spd);
            println!("{}: {}", name, xyz);
            let rgb = xyz_to_rgb(&xf_xyz_to_rec709, xyz);
            let srgb = RGBu8::from(oetf::srgb(rgb));
            assert_eq!(srgb, babel_average::sRGB_u8[name]);

            let lab = crate::lab::xyz_to_lab(
                cat_d65_to_d50 * xyz,
                illuminant::D50.xyz,
            );

            let lab_ref = babel_average::Lab_D50[name];

            let delta_e = delta_E(lab, lab_ref);
            assert!(delta_e < 1.4);
        }
    }
}
