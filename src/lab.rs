//! Lab color space and difference calculations.
//! See http://www.brucelindbloom.com/index.html?ColorDifferenceCalc.html
use super::chromaticity::*;
use super::math::*;
use super::rgb::*;
use super::xyz::*;

pub struct Lab {
    pub L: f32,
    pub a: f32,
    pub b: f32,
}

pub fn lab(L: f32, a: f32, b: f32) -> Lab {
    Lab { L, a, b }
}

pub fn xyz_to_lab(xyz: XYZ, ref_white: Chromaticity) -> Lab {
    let xyz_rw = XYZ::from_chromaticity(ref_white, 1.0);
    let xyz_r = xyz / xyz_rw;

    const EPSILON: f32 = 216.0 / 24389.0;
    const KAPPA: f32 = 24389.0 / 27.0;

    let f_x = if xyz_r.x > EPSILON {
        xyz_r.x.powf(1.0 / 3.0)
    } else {
        (KAPPA * xyz_r.x + 16.0) / 116.0
    };

    let f_y = if xyz_r.y > EPSILON {
        xyz_r.y.powf(1.0 / 3.0)
    } else {
        (KAPPA * xyz_r.y + 16.0) / 116.0
    };

    let f_z = if xyz_r.z > EPSILON {
        xyz_r.z.powf(1.0 / 3.0)
    } else {
        (KAPPA * xyz_r.z + 16.0) / 116.0
    };

    lab(116.0 * f_y - 16.0, 500.0 * (f_x - f_y), 200.0 * (f_y - f_z))
}

#[allow(non_snake_case)]
pub fn delta_E_1976(c1: Lab, c2: Lab) -> f32 {
    sqrt((c1.L - c2.L).powi(2) + (c1.a - c2.a).powi(2) + (c1.b - c2.b).powi(2))
}

#[test]
fn test_delta_e() {
    use crate::chromatic_adaptation::*;
    let A_xyz = XYZ::new(0.315756, 0.162732, 0.015905);
    let C_xyz = XYZ::new(0.203465, 0.116458, 0.053125);

    let A_to_C_mtx =
        bradford(A_xyz.into(), C_xyz.into());

    // let A_to_C_mtx =
    //     bradford_xyz(A_xyz, C_xyz);

    let A_to_C = A_to_C_mtx * A_xyz;
    println!("A to C: {}", A_to_C);
}
