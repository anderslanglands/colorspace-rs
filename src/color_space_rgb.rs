use super::chromaticity::Chromaticity;
use super::illuminant::D65;
use super::rgb::{rgbf32, RGBf32};
use super::spectral_power_distribution::SPD;
use super::xyz::XYZf32;
use imath::matrix::M33f;
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
    #[allow(non_snake_case)]
    pub fn srgb_RGBf32(x: RGBf32) -> RGBf32 {
        RGBf32 {
            r: srgb_f32(x.r),
            g: srgb_f32(x.g),
            b: srgb_f32(x.b),
        }
    }

    #[inline]
    pub fn rec709_f32(x: f32) -> f32 {
        if x <= 0.018 {
            x * 4.5
        } else {
            1.099 * x.powf(0.45) - 0.099
        }
    }

    #[inline]
    #[allow(non_snake_case)]
    pub fn rec709_RGBf32(x: RGBf32) -> RGBf32 {
        RGBf32 {
            r: rec709_f32(x.r),
            g: rec709_f32(x.g),
            b: rec709_f32(x.b),
        }
    }

    #[inline]
    pub fn linear_f32(x: f32) -> f32 {
        x
    }

    #[inline]
    #[allow(non_snake_case)]
    pub fn linear_RGBf32(x: RGBf32) -> RGBf32 {
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
    #[allow(non_snake_case)]
    pub fn srgb_RGBf32(x: RGBf32) -> RGBf32 {
        RGBf32 {
            r: srgb_f32(x.r),
            g: srgb_f32(x.g),
            b: srgb_f32(x.b),
        }
    }

    #[inline]
    pub fn rec709_f32(f: f32) -> f32 {
        if f <= 0.018 * 4.5 {
            f / 4.5
        } else {
            ((f + 0.099) / 1.099).powf(1.0 / 0.45)
        }
    }

    #[inline]
    #[allow(non_snake_case)]
    pub fn rec709_RGBf32(x: RGBf32) -> RGBf32 {
        RGBf32 {
            r: rec709_f32(x.r),
            g: rec709_f32(x.g),
            b: rec709_f32(x.b),
        }
    }

    #[inline]
    pub fn linear_f32(x: f32) -> f32 {
        x
    }

    #[inline]
    #[allow(non_snake_case)]
    pub fn linear_RGBf32(x: RGBf32) -> RGBf32 {
        RGBf32 {
            r: linear_f32(x.r),
            g: linear_f32(x.g),
            b: linear_f32(x.b),
        }
    }

}

pub struct Primaries {
    red: Chromaticity,
    green: Chromaticity,
    blue: Chromaticity,
}

pub type TransferFunction = Box<Fn(RGBf32) -> RGBf32>;

pub struct ColorSpaceRGB {
    xf_xyz_to_rgb: M33f,
    xf_rgb_to_xyz: M33f,
    primaries: Primaries,
    whitepoint: Chromaticity,
}

impl ColorSpaceRGB {
    pub fn new(
        primaries: Primaries,
        whitepoint: Chromaticity,
    ) -> ColorSpaceRGB {
        let xf_xyz_to_rgb = build_xyz_to_rgb_matrix(&primaries, &whitepoint);
        let xf_rgb_to_xyz = xf_xyz_to_rgb.inverse().unwrap();

        ColorSpaceRGB {
            xf_xyz_to_rgb,
            xf_rgb_to_xyz,
            primaries,
            whitepoint,
        }
    }

    pub fn xyz_to_rgb(&self, xyz: XYZf32) -> RGBf32 {
        let m = &self.xf_xyz_to_rgb;
        rgbf32(
            m[0][0] * xyz.x + m[0][1] * xyz.y + m[0][2] * xyz.z,
            m[1][0] * xyz.x + m[1][1] * xyz.y + m[1][2] * xyz.z,
            m[2][0] * xyz.x + m[2][1] * xyz.y + m[2][2] * xyz.z,
        )
    }
}

fn build_xyz_to_rgb_matrix(
    primaries: &Primaries,
    whitepoint: &Chromaticity,
) -> M33f {
    let xr = primaries.red.x;
    let yr = primaries.red.y;
    let zr = 1.0 - (xr + yr);
    let xg = primaries.green.x;
    let yg = primaries.green.y;
    let zg = 1.0 - (xg + yg);
    let xb = primaries.blue.x;
    let yb = primaries.blue.y;
    let zb = 1.0 - (xb + yb);

    let xw = whitepoint.x;
    let yw = whitepoint.y;
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
    let mut xf_xyz_to_rgb = M33f::make_identity();
    xf_xyz_to_rgb[0][0] = rx / rw;
    xf_xyz_to_rgb[0][1] = ry / rw;
    xf_xyz_to_rgb[0][2] = rz / rw;
    xf_xyz_to_rgb[1][0] = gx / gw;
    xf_xyz_to_rgb[1][1] = gy / gw;
    xf_xyz_to_rgb[1][2] = gz / gw;
    xf_xyz_to_rgb[2][0] = bx / bw;
    xf_xyz_to_rgb[2][1] = by / bw;
    xf_xyz_to_rgb[2][2] = bz / bw;

    xf_xyz_to_rgb
}

lazy_static! {
    pub static ref ITUR_BT709: ColorSpaceRGB = {
        ColorSpaceRGB::new(
            Primaries {
                red: Chromaticity { x: 0.64, y: 0.33 },
                green: Chromaticity { x: 0.30, y: 0.60 },
                blue: Chromaticity { x: 0.15, y: 0.06 },
            },
            Chromaticity {
                x: 0.3128,
                y: 0.3290,
            },
        )
    };
}
