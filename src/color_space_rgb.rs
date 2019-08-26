//! Defining RGB color spaces from primaries, whitepoint and OETF
#![allow(clippy::excessive_precision, clippy::unreadable_literal)]
use super::chromaticity::*;
use super::math::{M3f32, M3f64, Matrix33, Real};
use super::rgb::{RGBf, RGBf32, RGBf64};
use lazy_static::lazy_static;

use numeric_literals::replace_float_literals;

pub mod encode {

    use crate::math::Real;
    use crate::rgb::RGBf;
    use numeric_literals::replace_float_literals;

    #[inline]
    #[replace_float_literals(T::from(literal).unwrap())]
    pub fn srgb_t<T>(x: T) -> T
    where
        T: Real,
    {
        if x <= 0.0031308 {
            x * 12.92
        } else {
            (1.0 + 0.055) * x.powf(1.0 / 2.4) - 0.055
        }
    }

    #[inline]
    pub fn srgb<T>(x: RGBf<T>) -> RGBf<T>
    where
        T: Real,
    {
        RGBf {
            r: srgb_t(x.r),
            g: srgb_t(x.g),
            b: srgb_t(x.b),
        }
    }

    #[inline]
    #[replace_float_literals(T::from(literal).unwrap())]
    pub fn bt709_t<T>(x: T) -> T
    where
        T: Real,
    {
        if x <= 0.018 {
            x * 4.5
        } else {
            // let alpha = 1.09929682680944;
            1.099 * x.powf(0.45) - 0.099
        }
    }

    #[inline]
    pub fn bt709<T>(x: RGBf<T>) -> RGBf<T>
    where
        T: Real,
    {
        RGBf {
            r: bt709_t(x.r),
            g: bt709_t(x.g),
            b: bt709_t(x.b),
        }
    }

    #[inline]
    #[replace_float_literals(T::from(literal).unwrap())]
    pub fn bt2020_t<T>(x: T) -> T
    where
        T: Real,
    {
        let alpha = 1.099;
        let beta = 0.018;
        if x < beta {
            x * 4.5
        } else {
            alpha * x.powf(0.45) - (alpha - 1.0)
        }
    }

    #[inline]
    pub fn bt2020<T>(x: RGBf<T>) -> RGBf<T>
    where
        T: Real,
    {
        RGBf {
            r: bt2020_t(x.r),
            g: bt2020_t(x.g),
            b: bt2020_t(x.b),
        }
    }

    #[inline]
    pub fn linear_t<T>(x: T) -> T
    where
        T: Real,
    {
        x
    }

    #[inline]
    pub fn linear<T>(x: RGBf<T>) -> RGBf<T>
    where
        T: Real,
    {
        RGBf {
            r: linear_t(x.r),
            g: linear_t(x.g),
            b: linear_t(x.b),
        }
    }

    #[inline]
    #[replace_float_literals(T::from(literal).unwrap())]
    pub fn alexa_logc_v3_t<T>(x: T) -> T
    where
        T: Real,
    {
        // using parameters for EI 800
        let cut = 0.010591;
        let a = 5.555556;
        let b = 0.052272;
        let c = 0.247190;
        let d = 0.385537;
        let e = 5.367655;
        let f = 0.092809;
        if x > cut {
            c * (a * x + b).log10() + d
        } else {
            e * x + f
        }
    }

    #[inline]
    pub fn alexa_logc_v3<T>(x: RGBf<T>) -> RGBf<T>
    where
        T: Real,
    {
        RGBf {
            r: alexa_logc_v3_t(x.r),
            g: alexa_logc_v3_t(x.g),
            b: alexa_logc_v3_t(x.b),
        }
    }
}

pub mod decode {

    use crate::math::Real;
    use crate::rgb::RGBf;
    use numeric_literals::replace_float_literals;

    #[inline]
    #[replace_float_literals(T::from(literal).unwrap())]
    pub fn srgb_t<T>(f: T) -> T
    where
        T: Real,
    {
        if f <= 0.040449936 {
            f / 12.92
        } else {
            ((f + 0.055) / 1.055).powf(2.4)
        }
    }

    #[inline]
    pub fn srgb<T>(x: RGBf<T>) -> RGBf<T>
    where
        T: Real,
    {
        RGBf {
            r: srgb_t(x.r),
            g: srgb_t(x.g),
            b: srgb_t(x.b),
        }
    }

    #[inline]
    #[replace_float_literals(T::from(literal).unwrap())]
    pub fn bt709_t<T>(f: T) -> T
    where
        T: Real,
    {
        if f <= 0.018 * 4.5 {
            f / 4.5
        } else {
            ((f + 0.099) / 1.099).powf(1.0 / 0.45)
        }
    }

    #[inline]
    pub fn bt709<T>(x: RGBf<T>) -> RGBf<T>
    where
        T: Real,
    {
        RGBf {
            r: bt709_t(x.r),
            g: bt709_t(x.g),
            b: bt709_t(x.b),
        }
    }

    #[inline]
    #[replace_float_literals(T::from(literal).unwrap())]
    pub fn bt2020_t<T>(f: T) -> T
    where
        T: Real,
    {
        let alpha = 1.099;
        let beta = 0.018;
        if f < beta * 4.5 {
            f / 4.5
        } else {
            ((f + (alpha - 1.0)) / alpha).powf(1.0 / 0.45)
        }
    }

    #[inline]
    pub fn bt2020<T>(x: RGBf<T>) -> RGBf<T>
    where
        T: Real,
    {
        RGBf {
            r: bt2020_t(x.r),
            g: bt2020_t(x.g),
            b: bt2020_t(x.b),
        }
    }

    #[inline]
    pub fn linear_t<T>(x: T) -> T
    where
        T: Real,
    {
        x
    }

    #[inline]
    pub fn linear<T>(x: RGBf<T>) -> RGBf<T>
    where
        T: Real,
    {
        RGBf {
            r: linear_t(x.r),
            g: linear_t(x.g),
            b: linear_t(x.b),
        }
    }

    #[inline]
    #[replace_float_literals(T::from(literal).unwrap())]
    pub fn alexa_logc_v3_t<T>(t: T) -> T
    where
        T: Real,
    {
        // using parameters for EI 800
        let a = 5.555556;
        let b = 0.052272;
        let c = 0.247190;
        let d = 0.385537;
        let e = 5.367655;
        let f = 0.092809;
        let ecf = 0.149658;
        if t > ecf {
            (10.0.powf((t - d) / c) - b) / a
        } else {
            (t - f) / e
        }
    }

    #[inline]
    pub fn alexa_logc_v3<T>(x: RGBf<T>) -> RGBf<T>
    where
        T: Real,
    {
        RGBf {
            r: alexa_logc_v3_t(x.r),
            g: alexa_logc_v3_t(x.g),
            b: alexa_logc_v3_t(x.b),
        }
    }
}
pub type TransferFunction<T> = Box<dyn Fn(RGBf<T>) -> RGBf<T> + Sync + Send>;

/// Defines a tristimulus RGB color space as a collection of primaries, a
/// whitepoint and OETF.
pub struct ColorSpaceRGB<T>
where
    T: Real,
{
    pub xf_xyz_to_rgb: Matrix33<T>,
    pub xf_rgb_to_xyz: Matrix33<T>,
    pub red: XYY<T>,
    pub green: XYY<T>,
    pub blue: XYY<T>,
    pub white: XYY<T>,
    pub oetf: TransferFunction<T>,
    pub eotf: TransferFunction<T>,
}

/// Create a new color space using the supplied primaries and transfer functions
/// ```
/// // Define the DCI P3 color space
/// use colorspace::*;
/// let cs_dci_p3 = ColorSpaceRGB::<f64>::new(
///     XYYf64 { x: 0.680, y: 0.320, Y: 1.0 },
///     XYYf64 { x: 0.265, y: 0.690, Y: 1.0 },
///     XYYf64 { x: 0.150, y: 0.060 , Y: 1.0},
///     XYYf64 {
///         x: 0.314,
///         y: 0.351,
///         Y: 1.0,
///     },
///     Box::new(|c: RGBf64| c.powf(1.0 / 2.6)),
///     Box::new(|c: RGBf64| c.powf(2.6)),
/// );
/// ```
impl<T> ColorSpaceRGB<T>
where
    T: Real,
{
    pub fn new(
        red: XYY<T>,
        green: XYY<T>,
        blue: XYY<T>,
        white: XYY<T>,
        oetf: TransferFunction<T>,
        eotf: TransferFunction<T>,
    ) -> ColorSpaceRGB<T> {
        let xf_xyz_to_rgb = build_xyz_to_rgb_matrix(&red, &green, &blue, &white);
        let xf_rgb_to_xyz = xf_xyz_to_rgb.inverse().unwrap();

        ColorSpaceRGB {
            xf_xyz_to_rgb,
            xf_rgb_to_xyz,
            red,
            green,
            blue,
            white,
            oetf,
            eotf,
        }
    }

    /// Create a new color space using the supplied XYZ->RGB conversion matrices
    /// instead of deriving them from the primaries. This is useful when the
    /// published spec for a color space differs from its mathematical definition,
    /// for example in the case of sRGB.
    ///
    /// ```
    /// // sRGB's published definition is different from the calculated values
    /// // due to rounding
    /// use colorspace::*;
    /// let cs_srgb = ColorSpaceRGB::<f64>::new_with_specified_matrices(
    ///     XYYf64 { x: 0.64, y: 0.33, Y: 1.0 },
    ///     XYYf64 { x: 0.30, y: 0.60, Y: 1.0 },
    ///     XYYf64 { x: 0.15, y: 0.06, Y: 1.0 },
    ///     XYYf64 {
    ///         x: 0.3127,
    ///         y: 0.3290,
    ///         Y: 1.0,
    ///     },
    ///     M3f64::new([3.2406, -1.5372, -0.4986,
    ///                    -0.9689, 1.8758, 0.0415,
    ///                    0.0557, -0.2040, 1.0570]),
    ///     M3f64::new([0.4124, 0.3576, 0.1805,
    ///                    0.2126, 0.7152, 0.0722,
    ///                    0.0193, 0.1192, 0.9505]),
    ///     Box::new(encode::srgb),
    ///     Box::new(decode::srgb),
    /// );
    /// ```
    #[allow(clippy::too_many_arguments)]
    pub fn new_with_specified_matrices(
        red: XYY<T>,
        green: XYY<T>,
        blue: XYY<T>,
        white: XYY<T>,
        xf_xyz_to_rgb: Matrix33<T>,
        xf_rgb_to_xyz: Matrix33<T>,
        oetf: TransferFunction<T>,
        eotf: TransferFunction<T>,
    ) -> ColorSpaceRGB<T> {
        ColorSpaceRGB {
            xf_xyz_to_rgb,
            xf_rgb_to_xyz,
            red,
            green,
            blue,
            white,
            oetf,
            eotf,
        }
    }

    /// Convert a scene-referred, linear color to a display-referred, possibly
    /// non-linear color using the opto-electrical transfer function.
    /// If the color space does not have an associated OETF then it simply
    /// returns `c` unaltered.
    #[inline(always)]
    pub fn encode(&self, c: RGBf<T>) -> RGBf<T> {
        (self.oetf)(c)
    }

    /// Convert a display-referred, possibly non-linear color to a
    /// scene-referred, linear color using the electro-optical transfer function.
    /// If the color space does not have an associated EOTF then it simply
    /// returns `c` unaltered.
    #[inline(always)]
    pub fn decode(&self, c: RGBf<T>) -> RGBf<T> {
        (self.eotf)(c)
    }
}

#[replace_float_literals(T::from(literal).unwrap())]
fn build_xyz_to_rgb_matrix<T>(
    red: &XYY<T>,
    green: &XYY<T>,
    blue: &XYY<T>,
    white: &XYY<T>,
) -> Matrix33<T>
where
    T: Real,
{
    let xr = red.x;
    let yr = red.y;
    let zr = 1.0 - (xr + yr);
    let xg = green.x;
    let yg = green.y;
    let zg = 1.0 - (xg + yg);
    let xb = blue.x;
    let yb = blue.y;
    let zb = 1.0 - (xb + yb);

    let xw = white.x;
    let yw = white.y;
    let zw = 1.0 - (xw + yw);

    // xyz -> rgb matrix, before scaling to white
    let rx = (yg * zb) - (yb * zg);
    let ry = (xb * zg) - (xg * zb);
    let rz = (xg * yb) - (xb * yg);
    let gx = (yb * zr) - (yr * zb);
    let gy = (xr * zb) - (xb * zr);
    let gz = (xb * yr) - (xr * yb);
    let bx = (yr * zg) - (yg * zr);
    let by = (xg * zr) - (xr * zg);
    let bz = (xr * yg) - (xg * yr);

    // White scaling factors.
    // Dividing by yw scales the white luminance to unity, as conventional
    let rw = ((rx * xw) + (ry * yw) + (rz * zw)) / yw;
    let gw = ((gx * xw) + (gy * yw) + (gz * zw)) / yw;
    let bw = ((bx * xw) + (by * yw) + (bz * zw)) / yw;

    // xyz -> rgb matrix, correctly scaled to white
    Matrix33::new([
        rx / rw,
        ry / rw,
        rz / rw,
        gx / gw,
        gy / gw,
        gz / gw,
        bx / bw,
        by / bw,
        bz / bw,
    ])
}

pub mod model_f64 {
    use super::*;

    lazy_static! {

            /// sRGB
            /// Data taken https://en.wikipedia.org/wiki/SRGB
            pub static ref SRGB: ColorSpaceRGB<f64> = {
                ColorSpaceRGB::new_with_specified_matrices(
                    XYYf64 { x: 0.64, y: 0.33, Y: 1.0 },
                    XYYf64 { x: 0.30, y: 0.60, Y: 1.0 },
                    XYYf64 { x: 0.15, y: 0.06, Y: 1.0 },
                    XYYf64 {
                        x: 0.3127,
                        y: 0.3290,
                        Y: 1.0,
                    },
                    M3f64::new([
                        3.2406, -1.5372, -0.4986,
                        -0.9689, 1.8758, 0.0415,
                        0.0557, -0.2040, 1.0570
                        ]),
                    M3f64::new([
                        0.4124, 0.3576, 0.1805,
                        0.2126, 0.7152, 0.0722,
                        0.0193, 0.1192, 0.9505
                        ]),
                    Box::new(encode::srgb),
                    Box::new(decode::srgb),
                )
            };

            /// sRGB - derived matrices
            /// Data taken https://en.wikipedia.org/wiki/SRGB
            pub static ref SRGB_DRV: ColorSpaceRGB<f64> = {
                ColorSpaceRGB::new(
                    XYYf64 { x: 0.64, y: 0.33, Y: 1.0 },
                    XYYf64 { x: 0.30, y: 0.60, Y: 1.0 },
                    XYYf64 { x: 0.15, y: 0.06, Y: 1.0 },
                    XYYf64 {
                        x: 0.3127,
                        y: 0.3290,
                        Y: 1.0,
                    },
                    Box::new(encode::srgb),
                    Box::new(decode::srgb),
                )
            };

            /// ITU-R Rec. BT.709
            /// Data taken from https://en.wikipedia.org/wiki/Rec._709
            pub static ref ITUR_BT709: ColorSpaceRGB<f64> = {
                ColorSpaceRGB::new(
                    XYYf64 { x: 0.64, y: 0.33, Y: 1.0 },
                    XYYf64 { x: 0.30, y: 0.60, Y: 1.0 },
                    XYYf64 { x: 0.15, y: 0.06, Y: 1.0 },
                    XYYf64 {
                        x: 0.3127,
                        y: 0.3290,
                        Y: 1.0,
                    },
                    Box::new(encode::bt709),
                    Box::new(decode::bt709),
                )
            };

            /// ITU-R Rec. BT.2020
            /// Data taken from https://en.wikipedia.org/wiki/Rec._2020
            /// See also https://www.itu.int/rec/R-REC-BT.1886-0-201103-I
            pub static ref ITUR_BT2020: ColorSpaceRGB<f64> = {
                ColorSpaceRGB::new(
                    XYYf64 { x: 0.708, y: 0.292, Y: 1.0 },
                    XYYf64 { x: 0.17, y: 0.797, Y: 1.0 },
                    XYYf64 { x: 0.131, y: 0.046, Y: 1.0 },
                    XYYf64 {
                        x: 0.3127,
                        y: 0.3290,
                        Y: 1.0,
                    },
                    Box::new(encode::bt2020),
                    Box::new(decode::bt2020),
                )
            };

            /// DCI-P3
            /// Data taken from https://en.wikipedia.org/wiki/DCI-P3
            pub static ref DCI_P3: ColorSpaceRGB<f64> = {
                ColorSpaceRGB::new(
                    XYYf64 { x: 0.680, y: 0.320, Y: 1.0 },
                    XYYf64 { x: 0.265, y: 0.690, Y: 1.0 },
                    XYYf64 { x: 0.150, y: 0.060 , Y: 1.0},
                    XYYf64 {
                        x: 0.314,
                        y: 0.351,
                        Y: 1.0,
                    },
                    Box::new(|c: RGBf64| c.powf(1.0 / 2.6)),
                    Box::new(|c: RGBf64| c.powf(2.6)),
                )
            };

            /// P3 D65
            /// Data taken from https://en.wikipedia.org/wiki/DCI-P3
            pub static ref DCI_P3_D65: ColorSpaceRGB<f64> = {
                ColorSpaceRGB::new(
                    XYYf64 { x: 0.680, y: 0.320 , Y: 1.0},
                    XYYf64 { x: 0.265, y: 0.690 , Y: 1.0},
                    XYYf64 { x: 0.150, y: 0.060 , Y: 1.0},
                    XYYf64 {
                        x: 0.3127,
                        y: 0.3290,
                        Y: 1.0,
                    },
                    Box::new(|c: RGBf64| c.powf(1.0 / 2.6)),
                    Box::new(|c: RGBf64| c.powf(2.6)),
                )
            };

            /// ACES archival color space. AP0 primaries.
            /// Data taken from https://en.wikipedia.org/wiki/Academy_Color_Encoding_System
            pub static ref ACES: ColorSpaceRGB<f64> = {
                ColorSpaceRGB::new_with_specified_matrices(
                    XYYf64 { x: 0.7347, y: 0.2653, Y: 1.0},
                    XYYf64 { x: 0.0000, y: 1.0000, Y: 1.0},
                    XYYf64 { x: 0.0001, y: -0.077, Y: 1.0},
                    XYYf64 {
                        x: 0.32168,
                        y: 0.33767,
                        Y: 1.0,
                    },
                    M3f64::new([
                        1.0498110175, 0.0000000000, -0.0000974845,
                        -0.4959030231, 1.3733130458, 0.0982400361,
                        0.0000000000, 0.0000000000, 0.9912520182,
                    ]),
                    M3f64::new([
                        0.9525523959, 0.0000000000, 0.0000936786,
                        0.3439664498, 0.7281660966, -0.0721325464,
                        0.0000000000, 0.0000000000, 1.0088251844,
                    ]),
                    Box::new(encode::linear),
                    Box::new(decode::linear),
                )
            };

            /// ACEScg color space. AP1 primaries.
            /// Data taken from https://en.wikipedia.org/wiki/Academy_Color_Encoding_System
            pub static ref ACES_CG: ColorSpaceRGB<f64> = {
                ColorSpaceRGB::new(
                    XYYf64 { x: 0.713, y: 0.293, Y: 1.0},
                    XYYf64 { x: 0.165, y: 0.830, Y: 1.0},
                    XYYf64 { x: 0.128, y: 0.044, Y: 1.0},
                    XYYf64 {
                        x: 0.32168,
                        y: 0.33767,
                        Y: 1.0,
                    },
                    Box::new(encode::linear),
                    Box::new(decode::linear),
                )
            };

            /// Adobe RGB (1998)
            /// Data taken from
            /// https://www.adobe.com/digitalimag/pdfs/AdobeRGB1998.pdf
            pub static ref ADOBE_RGB_1998: ColorSpaceRGB<f64> = {
                ColorSpaceRGB::new_with_specified_matrices(
                    XYYf64 { x: 0.6400, y: 0.3300, Y: 1.0},
                    XYYf64 { x: 0.2100, y: 0.7100, Y: 1.0},
                    XYYf64 { x: 0.1500, y: 0.0600, Y: 1.0},
                    XYYf64 {
                        x: 0.3127,
                        y: 0.3290,
                        Y: 1.0,
                    },
                    M3f64::new([
                        2.04159, -0.56501, -0.34473,
                        -0.96924, 1.87597, 0.04156,
                        0.01344, -0.11836, 1.01517,
                    ]),
                    M3f64::new([
                        0.57667, 0.18556, 0.18823,
                        0.29734, 0.62736, 0.07529,
                        0.02703, 0.07069, 0.99134,
                    ]),
                    Box::new(|c: RGBf64| c.powf(1.0 / 2.19921875)),
                    Box::new(|c: RGBf64| c.powf(2.19921875)),
                )
            };

            /// ARRI Alexa Wide Gamut.
            /// Data taken from "Alexa LogC Curve in VFX"
            pub static ref ALEXA_WIDE_GAMUT: ColorSpaceRGB<f64> = {
                ColorSpaceRGB::new_with_specified_matrices(
                    XYYf64 { x: 0.6840, y: 0.3130, Y: 1.0},
                    XYYf64 { x: 0.2210, y: 0.8480, Y: 1.0},
                    XYYf64 { x: 0.0861, y: -0.102, Y: 1.0},
                    XYYf64 {
                        x: 0.3127,
                        y: 0.3290,
                        Y: 1.0,
                    },
                    M3f64::new([
                        1.789066, -0.482534, -0.200076,
                        -0.639849, 1.396400, 0.194432,
                        -0.041532, 0.082335, 0.878868,
                    ]),
                    M3f64::new([
                        0.638008, 0.214704, 0.097744,
                        0.291954, 0.823841, -0.115795,
                        0.002798, -0.067034, 1.153294,
                    ]),
                    Box::new(encode::alexa_logc_v3),
                    Box::new(decode::alexa_logc_v3),
                )
            };
        }
}

pub mod model_f32 {
    use super::*;

    lazy_static! {

        /// sRGB
        /// Data taken https://en.wikipedia.org/wiki/SRGB
        pub static ref SRGB: ColorSpaceRGB<f32> = {
            ColorSpaceRGB::new_with_specified_matrices(
                XYYf32 { x: 0.64, y: 0.33, Y: 1.0 },
                XYYf32 { x: 0.30, y: 0.60, Y: 1.0 },
                XYYf32 { x: 0.15, y: 0.06, Y: 1.0 },
                XYYf32 {
                    x: 0.3127,
                    y: 0.3290,
                    Y: 1.0,
                },
                M3f32::new([3.2406, -1.5372, -0.4986,
                -0.9689, 1.8758, 0.0415,
                0.0557, -0.2040, 1.0570]),
                M3f32::new([0.4124, 0.3576, 0.1805,
                0.2126, 0.7152, 0.0722,
                0.0193, 0.1192, 0.9505]),
                Box::new(encode::srgb),
                Box::new(decode::srgb),
            )
        };

        /// ITU-R Rec. BT.709
        /// Data taken from https://en.wikipedia.org/wiki/Rec._709
        pub static ref ITUR_BT709: ColorSpaceRGB<f32> = {
            ColorSpaceRGB::new(
                XYYf32 { x: 0.64, y: 0.33, Y: 1.0 },
                XYYf32 { x: 0.30, y: 0.60, Y: 1.0 },
                XYYf32 { x: 0.15, y: 0.06, Y: 1.0 },
                XYYf32 {
                    x: 0.3127,
                    y: 0.3290,
                    Y: 1.0,
                },
                Box::new(encode::bt709),
                Box::new(decode::bt709),
            )
        };

        /// ITU-R Rec. BT.2020
        /// Data taken from https://en.wikipedia.org/wiki/Rec._2020
        pub static ref ITUR_BT2020: ColorSpaceRGB<f32> = {
            ColorSpaceRGB::new(
                XYYf32 { x: 0.708, y: 0.292, Y: 1.0 },
                XYYf32 { x: 0.17, y: 0.797, Y: 1.0 },
                XYYf32 { x: 0.131, y: 0.046, Y: 1.0 },
                XYYf32 {
                    x: 0.3127,
                    y: 0.3290,
                    Y: 1.0,
                },
                Box::new(encode::bt2020),
                Box::new(decode::bt2020),
            )
        };

        /// DCI-P3
        /// Data taken from https://en.wikipedia.org/wiki/DCI-P3
        pub static ref DCI_P3: ColorSpaceRGB<f32> = {
            ColorSpaceRGB::new(
                XYYf32 { x: 0.680, y: 0.320, Y: 1.0 },
                XYYf32 { x: 0.265, y: 0.690, Y: 1.0 },
                XYYf32 { x: 0.150, y: 0.060 , Y: 1.0},
                XYYf32 {
                    x: 0.314,
                    y: 0.351,
                    Y: 1.0,
                },
                Box::new(|c: RGBf32| c.powf(1.0 / 2.6)),
                Box::new(|c: RGBf32| c.powf(2.6)),
            )
        };

        /// P3 D65
        /// Data taken from https://en.wikipedia.org/wiki/DCI-P3
        pub static ref DCI_P3_D65: ColorSpaceRGB<f32> = {
            ColorSpaceRGB::new(
                XYYf32 { x: 0.680, y: 0.320 , Y: 1.0},
                XYYf32 { x: 0.265, y: 0.690 , Y: 1.0},
                XYYf32 { x: 0.150, y: 0.060 , Y: 1.0},
                XYYf32 {
                    x: 0.3127,
                    y: 0.3290,
                    Y: 1.0,
                },
                Box::new(|c: RGBf32| c.powf(1.0 / 2.6)),
                Box::new(|c: RGBf32| c.powf(2.6)),
            )
        };

        /// ACES archival color space. AP0 primaries.
        /// Data taken from https://en.wikipedia.org/wiki/Academy_Color_Encoding_System
        pub static ref ACES: ColorSpaceRGB<f32> = {
            ColorSpaceRGB::new_with_specified_matrices(
                XYYf32 { x: 0.7347, y: 0.2653, Y: 1.0},
                XYYf32 { x: 0.0000, y: 1.0000, Y: 1.0},
                XYYf32 { x: 0.0001, y: -0.077, Y: 1.0},
                XYYf32 {
                    x: 0.32168,
                    y: 0.33767,
                    Y: 1.0,
                },
                M3f32::new([
                    1.0498110175, 0.0000000000, -0.0000974845,
                    -0.4959030231, 1.3733130458, 0.0982400361,
                    0.0000000000, 0.0000000000, 0.9912520182,
                ]),
                M3f32::new([
                    0.9525523959, 0.0000000000, 0.0000936786,
                    0.3439664498, 0.7281660966, -0.0721325464,
                    0.0000000000, 0.0000000000, 1.0088251844,
                ]),
                Box::new(encode::linear),
                Box::new(decode::linear),
            )
        };

        /// ACEScg color space. AP1 primaries.
        /// Data taken from https://en.wikipedia.org/wiki/Academy_Color_Encoding_System
        pub static ref ACES_CG: ColorSpaceRGB<f32> = {
            ColorSpaceRGB::new(
                XYYf32 { x: 0.713, y: 0.293, Y: 1.0},
                XYYf32 { x: 0.165, y: 0.830, Y: 1.0},
                XYYf32 { x: 0.128, y: 0.044, Y: 1.0},
                XYYf32 {
                    x: 0.32168,
                    y: 0.33767,
                    Y: 1.0,
                },
                Box::new(encode::linear),
                Box::new(decode::linear),
            )
        };

        /// Adobe RGB (1998)
        /// Data taken from https://en.wikipedia.org/wiki/Adobe_RGB_color_space
        pub static ref ADOBE_RGB_1998: ColorSpaceRGB<f32> = {
            ColorSpaceRGB::new_with_specified_matrices(
                XYYf32 { x: 0.6400, y: 0.3300, Y: 1.0},
                XYYf32 { x: 0.2100, y: 0.7100, Y: 1.0},
                XYYf32 { x: 0.1500, y: 0.0600, Y: 1.0},
                XYYf32 {
                    x: 0.3127,
                    y: 0.3290,
                    Y: 1.0,
                },
                M3f32::new([
                    2.04159, -0.56501, -0.34473,
                    -0.96924, 1.87597, 0.04156,
                    0.01344, -0.11836, 1.01517,
                ]),
                M3f32::new([
                    0.57667, 0.18556, 0.18823,
                    0.29734, 0.62736, 0.07529,
                    0.02703, 0.07069, 0.99134,
                ]),
                Box::new(|c: RGBf32| c.powf(1.0 / 2.19921875)),
                Box::new(|c: RGBf32| c.powf(2.19921875)),
            )
        };

        /// ARRI Alexa Wide Gamut.
        /// Data taken from "Alexa LogC Curve in VFX"
        pub static ref ALEXA_WIDE_GAMUT: ColorSpaceRGB<f32> = {
            ColorSpaceRGB::new_with_specified_matrices(
                XYYf32 { x: 0.6840, y: 0.3130, Y: 1.0},
                XYYf32 { x: 0.2210, y: 0.8480, Y: 1.0},
                XYYf32 { x: 0.0861, y: -0.102, Y: 1.0},
                XYYf32 {
                    x: 0.3127,
                    y: 0.3290,
                    Y: 1.0,
                },
                M3f32::new([
                    1.789066, -0.482534, -0.200076,
                    -0.639849, 1.396400, 0.194432,
                    -0.041532, 0.082335, 0.878868,
                ]),
                M3f32::new([
                    0.638008, 0.214704, 0.097744,
                    0.291954, 0.823841, -0.115795,
                    0.002798, -0.067034, 1.153294,
                ]),
                Box::new(encode::alexa_logc_v3),
                Box::new(decode::alexa_logc_v3),
            )
        };

    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::*;

    use float_cmp::{ApproxEq, F64Margin};

    use std::collections::HashMap;

    fn rgb_workout(
        model: &ColorSpaceRGB<f64>,
        checker_linear: &HashMap<String, RGBf64>,
        checker_encoded: &HashMap<String, RGBf64>,
    ) {
        let xyz_to_rgb_mtx = xyz_to_rgb_matrix(model_f64::SRGB.white, model);
        for (name, xyz_ref) in colorchecker::XYZ_D65.iter() {
            // xyz to rgb
            let rgb = xyz_to_rgb(&xyz_to_rgb_mtx, *xyz_ref);
            println!("    rgb {}: {}", name, rgb);
            println!("REF rgb {}: {}", name, checker_linear[name]);
            assert!(rgb.approx_eq(
                checker_linear[name],
                F64Margin {
                    epsilon: 1e-14,
                    ulps: 2
                }
            ));

            // encode with the oetf
            let rgb = model.encode(rgb);
            println!("    encoded {}: {}", name, rgb);
            println!("REF encoded {}: {}", name, checker_encoded[name]);
            assert!(rgb.approx_eq(
                checker_encoded[name],
                F64Margin {
                    epsilon: 1e-14,
                    ulps: 2
                }
            ));

            // decode back to linear
            let rgb = model.decode(rgb);
            println!("    decoded {}: {}", name, rgb);
            println!("REF decoded {}: {}", name, checker_linear[name]);
            assert!(rgb.approx_eq(
                checker_linear[name],
                F64Margin {
                    epsilon: 1e-14,
                    ulps: 2
                }
            ));
        }
    }

    #[test]
    fn checker_srgb() {
        rgb_workout(
            &model_f64::SRGB,
            &colorchecker::SRGB_LINEAR,
            &colorchecker::SRGB_ENCODED,
        );
    }

    #[test]
    fn checker_u8_srgb() {
        let xyz_to_rgb_mtx = xyz_to_rgb_matrix(model_f64::SRGB.white, &model_f64::SRGB);
        for name in colorchecker::NAMES.iter() {
            let xyz_ref = colorchecker::XYZ_D65[*name];
            // xyz to rgb
            let rgb = xyz_to_rgb(&xyz_to_rgb_mtx, xyz_ref);
            // encode with the oetf
            let rgb = RGBu8::from(model_f64::SRGB.encode(rgb));
            println!("{:<20}: {}", name, rgb);
        }
    }

    #[test]
    fn checker_bt709() {
        rgb_workout(
            &model_f64::ITUR_BT709,
            &colorchecker::ITUR_BT709_LINEAR,
            &colorchecker::ITUR_BT709_ENCODED,
        );
    }

    #[test]
    fn checker_arri() {
        rgb_workout(
            &model_f64::ALEXA_WIDE_GAMUT,
            &colorchecker::ALEXA_WIDE_GAMUT_LINEAR,
            &colorchecker::ALEXA_WIDE_GAMUT_ENCODED,
        );
    }

    #[test]
    fn checker_aces0() {
        println!("aces xyz_to_rgb mtx {:?}", model_f64::ACES.xf_xyz_to_rgb);
        println!("srgb wp: {:?} {:?}", model_f64::SRGB.white, crate::xyz::XYZf64::from(model_f64::SRGB.white));
        println!("aces wp: {:?} {:?}", model_f64::ACES.white, crate::xyz::XYZf64::from(model_f64::ACES.white));
        let cat: M3f64 = crate::chromatic_adaptation::cat02(model_f64::SRGB.white, model_f64::ACES.white);
        println!("cat02: {:?}", cat);
        let brad: M3f64 = crate::chromatic_adaptation::bradford(model_f64::SRGB.white, model_f64::ACES.white);
        println!("bradford: {:?}", brad);
        rgb_workout(
            &model_f64::ACES,
            &colorchecker::ACES_LINEAR,
            &colorchecker::ACES_ENCODED,
        );
    }

    #[test]
    fn checker_bt2020() {
        rgb_workout(
            &model_f64::ITUR_BT2020,
            &colorchecker::ITUR_BT2020_LINEAR,
            &colorchecker::ITUR_BT2020_ENCODED,
        );
    }

    #[test]
    fn checker_acescg() {
        rgb_workout(
            &model_f64::ACES_CG,
            &colorchecker::ACES_CG_LINEAR,
            &colorchecker::ACES_CG_ENCODED,
        );
    }

    #[test]
    fn checker_dcip3() {
        rgb_workout(
            &model_f64::DCI_P3,
            &colorchecker::DCI_P3_LINEAR,
            &colorchecker::DCI_P3_ENCODED,
        );
    }

    #[test]
    fn checker_srgb_to_aces() {
        let mtx = rgb_to_rgb_matrix(&model_f64::SRGB, &model_f64::ACES);
        for (name, srgb) in colorchecker::SRGB_LINEAR.iter() {
            let rgb_aces = mtx * *srgb;
            assert!(rgb_aces.approx_eq(
                colorchecker::ACES_FROM_SRGB[name],
                F64Margin {
                    epsilon: 1e-14,
                    ulps: 1
                }
            ));
        }
    }
}
