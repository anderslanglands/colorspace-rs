//! Types and functions for working with color. This is the crate to use if you
//! care about converting spectral color data to and from various RGB color
//! spaces, and converting RGB colors between those spaces.
//!
//! # Examples
//! ```
//! // Definition of the sRGB color space
//! use color_space::color_space_rgb::sRGB;
//! // The prelude brings in common types
//! use color_space::prelude::*;
//! // Convert the spectral data for a measured MacBeth chart swatch to XYZ
//! // using the CIE 1931 2-degree CMFs and a D65 illuminant
//! let xyz = babel_average::spd["dark_skin"]
//!     .to_xyz_with_illuminant(&illuminant::D65);
//! // Convert the XYZ value to a display-referred, 8-bit RGB value
//! let rgb: RGBu8 = sRGB.xyz_to_rgb_with_oetf(xyz).into();
//! assert_eq!(rgb, rgbu8(115, 82, 68));
//! ```

pub mod chromatic_adaptation;
pub mod chromaticity;
pub mod cmf;
pub mod color_checker;
pub mod color_space_rgb;
pub mod illuminant;
pub mod math;
pub mod prelude;
pub mod rgb;
pub mod spd_conversion;
pub mod spectral_power_distribution;
mod traits;
pub mod transform;
pub mod xyz;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        use super::rgb::*;
        use super::spectral_power_distribution::*;

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
        use crate::prelude::*;

        /*
        let xyz = babel_average::spd["dark_skin"]
            .to_xyz_with_illuminant(&illuminant::D65);

        eprintln!("xyz: {}", xyz);

        let ones_xyz =
            illuminant::Ones.to_xyz_with_illuminant(&illuminant::D65);
        let ones_rgb = color_space_rgb::ITUR_BT709.xyz_to_rgb(ones_xyz);
        eprintln!("ones_rgb: {}", ones_rgb);

        let rgb = color_space_rgb::ITUR_BT709.xyz_to_rgb(xyz);
        eprintln!("rgbf32: {}", rgb);
        let rgb = oetf::srgb(rgb);
        eprintln!("rgbf32 srgb: {}", rgb);
        let rgbu8 = RGBu8::from(rgb);
        let rgbu16 = RGBu16::from(rgb);

        assert_eq!(rgbu8, babel_average::srgb_u8::dark_skin);

        eprintln!("rgbu8: {}", rgbu8);
        eprintln!("rgbu16: {}", rgbu16);

        */
        let d65_xyz = illuminant::D65.to_xyz().normalized();
        eprintln!("D65 xyz: {}", d65_xyz);

        let d65_xyz_from_xy = XYZ::from(Chromaticity {
            x: 0.3127,
            y: 0.3290,
        });
        eprintln!("D65 xy->xyz: {}", d65_xyz_from_xy);
        eprintln!("ACEScg matrix: {:?}", color_space_rgb::ACEScg.xf_xyz_to_rgb);
        eprintln!(
            "sRGB matrix: {:?}",
            color_space_rgb::ITUR_BT709.xf_xyz_to_rgb
        );

        let xf_xyz_to_acescg = xyz_to_rgb_matrix(
            color_space_rgb::ITUR_BT709.white,
            &color_space_rgb::ACEScg,
        );

        let xf_xyz_to_p3 = xyz_to_rgb_matrix(
            color_space_rgb::ITUR_BT709.white,
            &color_space_rgb::DCI_P3,
        );

        let xf_xyz_to_alexawide = xyz_to_rgb_matrix(
            color_space_rgb::ITUR_BT709.white,
            &color_space_rgb::AlexaWide,
        );

        let xf_r709_to_acescg = rgb_to_rgb_matrix(
            &color_space_rgb::ITUR_BT709,
            &color_space_rgb::ACEScg,
        );

        for (name, ref spd) in &*babel_average::spd {
            let xyz = spd.to_xyz_with_illuminant(&illuminant::D65);
            let rgb = color_space_rgb::ITUR_BT709.xyz_to_rgb(xyz);
            let srgb = RGBu8::from(oetf::srgb(rgb));
            assert_eq!(srgb, babel_average::sRGB_u8[name]);

            eprintln!("\n{} ----------", name);

            eprintln!("XYZ       {}", xyz);
            eprintln!("sRGB      {}", rgb);

            let rgb_acescg = xyz_to_rgb(&xf_xyz_to_acescg, xyz);
            eprintln!("ACEScg    {}", rgb_acescg);
            let rgb_acescg = xf_r709_to_acescg * rgb;
            eprintln!("ACEScg (from709) {}", rgb_acescg);

            let rgb_p3 = xyz_to_rgb(&xf_xyz_to_p3, xyz);
            eprintln!("P3        {}", rgb_p3);

            let rgb_alexawide = xyz_to_rgb(&xf_xyz_to_alexawide, xyz);
            eprintln!("AlexaWide {}", rgb_alexawide);
        }
    }
}
