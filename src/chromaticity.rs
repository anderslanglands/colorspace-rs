//! Chromaticity coordinates
use super::xyz::XYZ;
use std::convert::From;

/// Defines a pair of `xy` chromaticity coordinates
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct Chromaticity {
    pub x: f32,
    pub y: f32,
}

impl Chromaticity {
    fn new(x: f32, y: f32) -> Chromaticity {
        Chromaticity { x, y }
    }

    /// Convert the given XYZ tristimulus value to a (normalized) chromaticity
    /// value
    pub fn from_xyz(c: XYZ) -> Chromaticity {
        Chromaticity {
            x: (c.x / (c.x + c.y + c.z)) / c.y,
            y: (c.y / (c.x + c.y + c.z)) / c.y,
        }
    }
}

impl From<XYZ> for Chromaticity {
    fn from(c: XYZ) -> Chromaticity {
        Chromaticity::from_xyz(c)
    }
}
