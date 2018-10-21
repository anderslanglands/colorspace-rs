use super::chromatic_adaptation::*;
use super::chromaticity::*;
use super::color_space_rgb::*;
use super::math::*;
use super::rgb::*;
use super::xyz::*;

pub fn xyz_to_rgb_matrix(
    xyz_white: Chromaticity,
    color_space_rgb: &ColorSpaceRGB,
) -> Matrix33 {
    color_space_rgb.xf_xyz_to_rgb * bradford(xyz_white, color_space_rgb.white)
}

pub fn xyz_to_rgb(mtx: &Matrix33, xyz: XYZ) -> RGBf32 {
    let x = *mtx * xyz;
    rgbf32(x.x, x.y, x.z)
}

pub fn rgb_to_rgb_matrix(
    from_space: &ColorSpaceRGB,
    to_space: &ColorSpaceRGB,
) -> Matrix33 {
    to_space.xf_xyz_to_rgb
        * bradford(from_space.white, to_space.white)
        * from_space.xf_rgb_to_xyz
}
