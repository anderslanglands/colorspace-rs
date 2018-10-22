//! xyY coordinates
use super::xyz::XYZ;
use std::convert::From;

/// Defines a pair of `xy` chromaticity coordinates
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct xyY {
    pub x: f32,
    pub y: f32,
    pub Y: f32,
}

impl xyY {
    /// Convert the given XYZ tristimulus value to a chromaticity xyY
    /// value
    pub fn from_xyz(c: XYZ) -> xyY {
        xyY {
            x: (c.x / (c.x + c.y + c.z)),
            y: (c.y / (c.x + c.y + c.z)),
            Y: c.y,
        }
    }
}

impl From<XYZ> for xyY {
    fn from(c: XYZ) -> xyY {
        xyY::from_xyz(c)
    }
}
