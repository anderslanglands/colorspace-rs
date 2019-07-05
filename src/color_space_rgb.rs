//! Defining RGB color spaces from primaries, whitepoint and OETF
#![allow(clippy::excessive_precision, clippy::unreadable_literal)]
use super::chromaticity::xyY;
use super::math::Matrix33;
use super::rgb::RGBf32;
use lazy_static::lazy_static;

pub mod oetf {
    use crate::rgb::RGBf32;

    #[inline]
    pub fn srgb_f32(x: f32) -> f32 {
        if x <= 0.0031308 {
            x * 12.92
        } else {
            (1.0 + 0.055) * x.powf(1.0 / 2.4) - 0.055
        }
    }

    #[inline]
    pub fn srgb(x: RGBf32) -> RGBf32 {
        RGBf32 {
            r: srgb_f32(x.r),
            g: srgb_f32(x.g),
            b: srgb_f32(x.b),
        }
    }

    #[inline]
    pub fn bt709_f32(x: f32) -> f32 {
        if x <= 0.018 {
            x * 4.5
        } else {
            // let alpha = 1.09929682680944;
            1.099 * x.powf(0.45) - 0.099
        }
    }

    #[inline]
    pub fn bt709(x: RGBf32) -> RGBf32 {
        RGBf32 {
            r: bt709_f32(x.r),
            g: bt709_f32(x.g),
            b: bt709_f32(x.b),
        }
    }

    #[inline]
    pub fn bt2020_f32(x: f32) -> f32 {
        const ALPHA: f32 = 1.09929682680944;
        const BETA: f32 = 0.018053968510807;
        if x <= BETA {
            x * 4.5
        } else {
            ALPHA * x.powf(0.45) - (ALPHA - 1.0)
        }
    }

    #[inline]
    pub fn bt2020(x: RGBf32) -> RGBf32 {
        RGBf32 {
            r: bt2020_f32(x.r),
            g: bt2020_f32(x.g),
            b: bt2020_f32(x.b),
        }
    }

    #[inline]
    pub fn linear_f32(x: f32) -> f32 {
        x
    }

    #[inline]
    pub fn linear(x: RGBf32) -> RGBf32 {
        RGBf32 {
            r: linear_f32(x.r),
            g: linear_f32(x.g),
            b: linear_f32(x.b),
        }
    }

}

pub mod eotf {
    use crate::rgb::RGBf32;

    #[inline]
    pub fn srgb_f32(f: f32) -> f32 {
        if f <= 0.040449936 {
            f / 12.92
        } else {
            ((f + 0.055) / 1.055).powf(2.4)
        }
    }

    #[inline]
    pub fn srgb(x: RGBf32) -> RGBf32 {
        RGBf32 {
            r: srgb_f32(x.r),
            g: srgb_f32(x.g),
            b: srgb_f32(x.b),
        }
    }

    #[inline]
    pub fn bt709_f32(f: f32) -> f32 {
        if f <= 0.018 * 4.5 {
            f / 4.5
        } else {
            ((f + 0.099) / 1.099).powf(1.0 / 0.45)
        }
    }

    #[inline]
    pub fn bt709(x: RGBf32) -> RGBf32 {
        RGBf32 {
            r: bt709_f32(x.r),
            g: bt709_f32(x.g),
            b: bt709_f32(x.b),
        }
    }

    #[inline]
    pub fn bt2020_f32(f: f32) -> f32 {
        const ALPHA: f32 = 1.09929682680944;
        const BETA: f32 = 0.018053968510807;
        if f <= BETA * 4.5 {
            f / 4.5
        } else {
            ((f + (ALPHA - 1.0)) / ALPHA).powf(1.0 / 0.45)
        }
    }

    #[inline]
    pub fn bt2020(x: RGBf32) -> RGBf32 {
        RGBf32 {
            r: bt2020_f32(x.r),
            g: bt2020_f32(x.g),
            b: bt2020_f32(x.b),
        }
    }

    #[inline]
    pub fn linear_f32(x: f32) -> f32 {
        x
    }

    #[inline]
    pub fn linear(x: RGBf32) -> RGBf32 {
        RGBf32 {
            r: linear_f32(x.r),
            g: linear_f32(x.g),
            b: linear_f32(x.b),
        }
    }

}
pub type TransferFunction = Box<Fn(RGBf32) -> RGBf32 + Sync + Send>;

/// Defines a tristimulus RGB color space as a collection of primaries, a
/// whitepoint and OETF.
pub struct ColorSpaceRGB {
    pub xf_xyz_to_rgb: Matrix33,
    pub xf_rgb_to_xyz: Matrix33,
    pub red: xyY,
    pub green: xyY,
    pub blue: xyY,
    pub white: xyY,
    pub oetf: TransferFunction,
    pub eotf: TransferFunction,
}

/// Create a new color space using the supplied primaries and transfer functions
/// ```
/// // Define the DCI P3 color space
/// use colorspace::prelude::*;
/// let cs_dci_p3 = ColorSpaceRGB::new(
///     xyY { x: 0.680, y: 0.320, Y: 1.0 },
///     xyY { x: 0.265, y: 0.690, Y: 1.0 },
///     xyY { x: 0.150, y: 0.060 , Y: 1.0},
///     xyY {
///         x: 0.314,
///         y: 0.351,
///         Y: 1.0,
///     },
///     Box::new(|c: RGBf32| c.powf(1.0 / 2.6)),
///     Box::new(|c: RGBf32| c.powf(2.6)),
/// );
/// ```
impl ColorSpaceRGB {
    pub fn new(
        red: xyY,
        green: xyY,
        blue: xyY,
        white: xyY,
        oetf: TransferFunction,
        eotf: TransferFunction,
    ) -> ColorSpaceRGB {
        let xf_xyz_to_rgb =
            build_xyz_to_rgb_matrix(&red, &green, &blue, &white);
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
    /// published spec for a color space differs from its mathematical definition.
    ///
    /// ```
    /// // sRGB's published definition is different from the calculated values
    /// // due to rounding
    /// use colorspace::prelude::*;
    /// let cs_srgb = ColorSpaceRGB::new_with_specified_matrices(
    ///     xyY { x: 0.64, y: 0.33, Y: 1.0 },
    ///     xyY { x: 0.30, y: 0.60, Y: 1.0 },
    ///     xyY { x: 0.15, y: 0.06, Y: 1.0 },
    ///     xyY {
    ///         x: 0.3127,
    ///         y: 0.3290,
    ///         Y: 1.0,
    ///     },
    ///     Matrix33::new([3.2406, -1.5372, -0.4986,
    ///                    -0.9689, 1.8758, 0.0415,
    ///                    0.0557, -0.2040, 1.0570]),
    ///     Matrix33::new([0.4124, 0.3576, 0.1805,
    ///                    0.2126, 0.7152, 0.0722,
    ///                    0.0193, 0.1192, 0.9505]),
    ///     Box::new(oetf::srgb),
    ///     Box::new(eotf::srgb),
    /// );
    /// ```
    #[allow(clippy::too_many_arguments)]
    pub fn new_with_specified_matrices(
        red: xyY,
        green: xyY,
        blue: xyY,
        white: xyY,
        xf_xyz_to_rgb: Matrix33,
        xf_rgb_to_xyz: Matrix33,
        oetf: TransferFunction,
        eotf: TransferFunction,
    ) -> ColorSpaceRGB {
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
    pub fn encode(&self, c: RGBf32) -> RGBf32 {
        (self.oetf)(c)
    }

    /// Convert a display-referred, possibly non-linear color to a
    /// scene-referred, linear color using the electro-optical transfer function.
    /// If the color space does not have an associated EOTF then it simply
    /// returns `c` unaltered.
    #[inline(always)]
    pub fn decode(&self, c: RGBf32) -> RGBf32 {
        (self.eotf)(c)
    }
}

fn build_xyz_to_rgb_matrix(
    red: &xyY,
    green: &xyY,
    blue: &xyY,
    white: &xyY,
) -> Matrix33 {
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

lazy_static! {

    /// sRGB
    /// Data taken https://en.wikipedia.org/wiki/SRGB
    pub static ref SRGB: ColorSpaceRGB = {
        ColorSpaceRGB::new_with_specified_matrices(
            xyY { x: 0.64, y: 0.33, Y: 1.0 },
            xyY { x: 0.30, y: 0.60, Y: 1.0 },
            xyY { x: 0.15, y: 0.06, Y: 1.0 },
            xyY {
                x: 0.3127,
                y: 0.3290,
                Y: 1.0,
            },
            Matrix33::new([3.2406, -1.5372, -0.4986,
            -0.9689, 1.8758, 0.0415,
            0.0557, -0.2040, 1.0570]),
            Matrix33::new([0.4124, 0.3576, 0.1805,
            0.2126, 0.7152, 0.0722,
            0.0193, 0.1192, 0.9505]),
            Box::new(oetf::srgb),
            Box::new(eotf::srgb),
        )
    };

    /// ITU-R Rec. BT.709
    /// Data taken from https://en.wikipedia.org/wiki/Rec._709
    pub static ref ITUR_BT709: ColorSpaceRGB = {
        ColorSpaceRGB::new_with_specified_matrices(
            xyY { x: 0.64, y: 0.33, Y: 1.0 },
            xyY { x: 0.30, y: 0.60, Y: 1.0 },
            xyY { x: 0.15, y: 0.06, Y: 1.0 },
            xyY {
                x: 0.3127,
                y: 0.3290,
                Y: 1.0,
            },
            Matrix33::new([3.2406, -1.5372, -0.4986,
            -0.9689, 1.8758, 0.0415,
            0.0557, -0.2040, 1.0570]),
            Matrix33::new([0.4124, 0.3576, 0.1805,
            0.2126, 0.7152, 0.0722,
            0.0193, 0.1192, 0.9505]),
            Box::new(oetf::bt709),
            Box::new(eotf::bt709),
        )
    };

    /// ITU-R Rec. BT.2020
    /// Data taken from https://en.wikipedia.org/wiki/Rec._2020
    pub static ref ITUR_BT2020: ColorSpaceRGB = {
        ColorSpaceRGB::new(
            xyY { x: 0.708, y: 0.292, Y: 1.0 },
            xyY { x: 0.17, y: 0.797, Y: 1.0 },
            xyY { x: 0.131, y: 0.046, Y: 1.0 },
            xyY {
                x: 0.3127,
                y: 0.3290,
                Y: 1.0,
            },
            Box::new(oetf::bt2020),
            Box::new(eotf::bt2020),
        )
    };

    /// DCI-P3
    /// Data taken from https://en.wikipedia.org/wiki/DCI-P3
    pub static ref DCI_P3: ColorSpaceRGB = {
        ColorSpaceRGB::new(
            xyY { x: 0.680, y: 0.320, Y: 1.0 },
            xyY { x: 0.265, y: 0.690, Y: 1.0 },
            xyY { x: 0.150, y: 0.060 , Y: 1.0},
            xyY {
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
    pub static ref DCI_P3_D65: ColorSpaceRGB = {
        ColorSpaceRGB::new(
            xyY { x: 0.680, y: 0.320 , Y: 1.0},
            xyY { x: 0.265, y: 0.690 , Y: 1.0},
            xyY { x: 0.150, y: 0.060 , Y: 1.0},
            xyY {
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
    pub static ref ACES2065_1: ColorSpaceRGB = {
        ColorSpaceRGB::new(
            xyY { x: 0.7347, y: 0.2653, Y: 1.0},
            xyY { x: 0.0000, y: 1.0000, Y: 1.0},
            xyY { x: 0.0001, y: -0.077, Y: 1.0},
            xyY {
                x: 0.32168,
                y: 0.33767,
                Y: 1.0,
            },
            Box::new(oetf::linear),
            Box::new(eotf::linear),
        )
    };

    /// ACEScg color space. AP1 primaries.
    /// Data taken from https://en.wikipedia.org/wiki/Academy_Color_Encoding_System
    pub static ref ACESCG: ColorSpaceRGB = {
        ColorSpaceRGB::new(
            xyY { x: 0.713, y: 0.293, Y: 1.0},
            xyY { x: 0.165, y: 0.830, Y: 1.0},
            xyY { x: 0.128, y: 0.044, Y: 1.0},
            xyY {
                x: 0.32168,
                y: 0.33767,
                Y: 1.0,
            },
            Box::new(oetf::linear),
            Box::new(eotf::linear),
        )
    };

    /// Adobe RGB (1998)
    /// Data taken from https://en.wikipedia.org/wiki/Adobe_RGB_color_space
    pub static ref ADOBE: ColorSpaceRGB = {
        ColorSpaceRGB::new(
            xyY { x: 0.6400, y: 0.3300, Y: 1.0},
            xyY { x: 0.2100, y: 0.7100, Y: 1.0},
            xyY { x: 0.1500, y: 0.0600, Y: 1.0},
            xyY {
                x: 0.3127,
                y: 0.3290,
                Y: 1.0,
            },
            Box::new(|c: RGBf32| c.powf(1.0 / 2.19921875)),
            Box::new(|c: RGBf32| c.powf(2.19921875)),
        )
    };

    /// ARRI Alexa Wide Gamut.
    /// Data taken from "Alexa LogC Curve in VFX"
    /// http://www.arri.com/?eID=registration&file_uid=18358
    /// FIXME: Implement logC here as the OETF
    pub static ref ALEXAWIDE: ColorSpaceRGB = {
        ColorSpaceRGB::new(
            xyY { x: 0.6840, y: 0.3130, Y: 1.0},
            xyY { x: 0.2210, y: 0.8480, Y: 1.0},
            xyY { x: 0.0861, y: -0.102, Y: 1.0},
            xyY {
                x: 0.3127,
                y: 0.3290,
                Y: 1.0,
            },
            Box::new(oetf::linear),
            Box::new(eotf::linear),
        )
    };
}
