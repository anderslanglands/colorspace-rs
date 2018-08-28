pub mod chromaticity;
pub mod color_checker;
pub mod color_matching_function;
pub mod color_space_rgb;
pub mod illuminant;
pub mod math;
pub mod rgb;
pub mod spd_conversion;
pub mod spectral_power_distribution;
mod traits;
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
        use crate::rgb::{RGBu8, RGBu16};
        let xyz = crate::spd_conversion::spd_to_xyz_with_illuminant(
            &crate::color_checker::babel_average::spectrum::spd_dark_skin,
            &crate::color_matching_function::cmf_CIE_1931_2_degree,
            &crate::illuminant::D65,
        );

        eprintln!("xyz: {}", xyz);

        let rgb = crate::color_space_rgb::ITUR_BT709.xyz_to_rgb(xyz);
        let rgb = crate::color_space_rgb::oetf::srgb_RGBf32(rgb);
        let rgbu8 = RGBu8::from(rgb);
        let rgbu16 = RGBu16::from(rgb);

        eprintln!("rgbu8: {}", rgbu8);
        eprintln!("rgbu16: {}", rgbu16);
    }
}
