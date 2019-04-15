//! # colorspace

//!  A crate for colorimetry in Rust.
//!  This crate contains types and functions for working with color. The intended
//!  use is to support rendering applications (I use it to manage color in a spectral pathtracer), but if you want to be able to
//!  convert between spectral, XYZ, L'a'b' and RGB spaces of various flavors such as
//!  sRGB, ACES, DCI P3 and ALEXA Wide Gamut then this is the crate for you.

//!  Note that currently the spectral->XYZ results are not as accurate as they could be, due
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
//!  use colorspace::color_space_rgb::SRGB;
//!  // The prelude brings in common types
//!  use colorspace::prelude::*;
//!  // Convert the spectral data for a measured MacBeth chart swatch to XYZ
//!  // using the CIE 1931 2-degree CMFs and a D65 illuminant
//!  let xyz = babel_average::SPECTRAL["dark_skin"]
//!      .to_xyz_with_illuminant(&illuminant::D65.spd);
//!  // Convert the XYZ value to scene-referred (i.e. linear) sRGB
//!  let xf_xyz_to_srgb = xyz_to_rgb_matrix(SRGB.white, &SRGB);
//!  let rgb = xyz_to_rgb(&xf_xyz_to_srgb, xyz);
//!  // Convert the scene-referred sRGB value to an 8-bit, display-referred
//!  // value by applying the opto-electrical transfer function and using RGBu8's
//!  // From<RGBf32> impl
//!  let rgb: RGBu8 = (SRGB.oetf)(rgb).into();
//!  assert_eq!(rgb, rgbu8(115, 82, 68));
//!  ```
//!
//! ### RGB to spectral
//!  ```rust
//!// Use Smits' spectral uplifting to convert each swatch of the Macbeth chart
//!// to a spectral reflectivity, then convert back to XYZ and check that the
//!// result is within tolerance using delta_E in L'a'b' space.
//! use colorspace::prelude::*;
//!// We create the matrix for the RGB->XYZ transform up front, in case we
//!// want to convert many colors
//!let xf_rec709_to_xyz = rgb_to_xyz_matrix(ITUR_BT709.white, &ITUR_BT709);
//!// We use the Bradford CAT to convert from sRGB D65 to D50 for the Lab
//!// comparison (by convention)
//!let cat_d65_to_d50 = crate::chromatic_adaptation::bradford(
//!    illuminant::D65.xyz,
//!    illuminant::D50.xyz,
//!);
//!// color_checker::babel_average supplies sRGB u8 versions for convenience
//!for (_name, srgbu8) in babel_average::SRGB_U8.iter() {
//!    // Convert to a scene-referred float RGB value using the EOTF
//!    let rgb = eotf::srgb(RGBf32::from(*srgbu8));
//!    // Upsample the RGB value to an SPD
//!    let ups_spd = rgb_to_spd_smits_refl(rgb);
//!    // Convert the SPD to XYZ
//!    let ups_xyz = ups_spd.to_xyz_with_illuminant(&illuminant::D65.spd);
//!    // check delta_E is within tolerance after our round trip
//!    // first convert to XYZ, then to Lab
//!    let xyz = rgb_to_xyz(&xf_rec709_to_xyz, rgb);
//!    let orig_lab =
//!        crate::lab::xyz_to_lab(cat_d65_to_d50 * xyz, illuminant::D50.xyz);
//!    let ups_lab = crate::lab::xyz_to_lab(
//!        cat_d65_to_d50 * ups_xyz,
//!        illuminant::D50.xyz,
//!    );
//!    assert!(delta_E(orig_lab, ups_lab) < 1.39f32);
//!}
//!  ```
//!
//!  ## Licence
//!  colorspace is licensed under Apache License, Version 2.0
//!  http://www.apache.org/licenses/LICENSE-2.0
//!
//!  This crate contains some data (specifically the standard illuminants) taken from the excellent colour-science python
//!  library by Mansencal et al.: <https://www.colour-science.org>
//!  Colour by Colour Developers - 2013-2019
//!  Copyright © 2013-2019 – Colour Developers – colour-science@googlegroups.com
//!  This software is released under terms of New BSD License: http://opensource.org/licenses/BSD-3-Clause
//!  http://github.com/colour-science/colour
//!
//!  Most of the conversion algorithms are based on those published at
//!  Bruce Lindbloom's site: <http://www.brucelindbloom.com>
//!  Copyright © 2001 - 2018 Bruce Justin Lindbloom.
//!
//!  The Smits upsampling code and associated data are taken from PBRT
//!  Copyright (c) 1998-2015, Matt Pharr, Greg Humphreys, and Wenzel Jakob.
//!  All rights reserved.
//!  Redistribution and use in source and binary forms, with or without modification, are permitted provided that the following conditions are met:
//!  Redistributions of source code must retain the above copyright notice, this list of conditions and the following disclaimer.
//!  Redistributions in binary form must reproduce the above copyright notice, this list of conditions and the following disclaimer in the documentation and/or other materials provided with the distribution.
//!  THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
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
    ACESCG, ALEXAWIDE, DCI_P3, ITUR_BT2020, ITUR_BT709, SRGB,
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

        for (name, ref spd) in &*babel_average::SPECTRAL {
            let xyz = spd.to_xyz_with_illuminant(&illuminant::D65.spd);
            let rgb = xyz_to_rgb(&xf_xyz_to_rec709, xyz);
            let srgb = RGBu8::from(oetf::srgb(rgb));
            assert_eq!(srgb, babel_average::SRGB_U8[name]);

            let lab = crate::lab::xyz_to_lab(
                cat_d65_to_d50 * xyz,
                illuminant::D50.xyz,
            );

            let lab_ref = babel_average::LAB_D50[name];

            let delta_e = delta_E(lab, lab_ref);
            assert!(delta_e < 1.4);
        }
    }
}
