use super::chromatic_adaptation::*;
use super::chromaticity::*;
use super::color_space_rgb::*;
use super::math::*;
use super::rgb::*;
use super::xyz::*;

pub fn xyz_to_rgb_matrix(
    xyz_white: xyY,
    color_space_rgb: &ColorSpaceRGB,
) -> Matrix33 {
    color_space_rgb.xf_xyz_to_rgb
        * bradford(xyz_white.into(), color_space_rgb.white.into())
}

pub fn rgb_to_xyz_matrix(
    xyz_white: xyY,
    color_space_rgb: &ColorSpaceRGB,
) -> Matrix33 {
    bradford(color_space_rgb.white.into(), xyz_white.into())
        * color_space_rgb.xf_rgb_to_xyz
}

pub fn xyz_to_rgb(mtx: &Matrix33, xyz: XYZ) -> RGBf32 {
    let x = *mtx * xyz;
    rgbf32(x.x, x.y, x.z)
}

pub fn rgb_to_xyz(mtx: &Matrix33, rgb: RGBf32) -> XYZ {
    let x = *mtx * rgb;
    XYZ::new(x.r, x.g, x.b)
}

pub fn rgb_to_rgb_matrix(
    from_space: &ColorSpaceRGB,
    to_space: &ColorSpaceRGB,
) -> Matrix33 {
    to_space.xf_xyz_to_rgb
        * bradford(from_space.white.into(), to_space.white.into())
        * from_space.xf_rgb_to_xyz
}
