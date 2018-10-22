//! Lab color space and difference calculations.
//!
//! See http://www.brucelindbloom.com/index.html?ColorDifferenceCalc.html
use super::math::*;
use super::xyz::*;

#[repr(C, packed)]
#[derive(Copy, Clone, Debug)]
#[allow(non_snake_case)]
pub struct Lab {
    pub L: f32,
    pub a: f32,
    pub b: f32,
}

#[allow(non_snake_case)]
pub fn lab(L: f32, a: f32, b: f32) -> Lab {
    Lab { L, a, b }
}

pub fn xyz_to_lab(xyz: XYZ, ref_white: XYZ) -> Lab {
    let xyz_r = xyz / ref_white;

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

/// Compute the difference between two L*a*b* colors according to the CIE 1976
/// formula.
#[allow(non_snake_case)]
pub fn delta_E_1976(c1: Lab, c2: Lab) -> f32 {
    sqrt((c1.L - c2.L).powi(2) + (c1.a - c2.a).powi(2) + (c1.b - c2.b).powi(2))
}

/// Compute the difference between two L'a'b' colors according to the CIEDE2000
/// formula.
///
/// Implementation based on "The CIEDE2000 Color-Difference Formula:
/// Implementation Notes, Supplementary Test Data, and Mathematical Observations"
/// by Sharma et al.
/// http://www2.ece.rochester.edu/~gsharma/ciede2000/ciede2000noteCRNA.pdf
#[allow(non_snake_case)]
pub fn delta_E_2000(c1: Lab, c2: Lab) -> f32 {
    let L_1 = c1.L;
    let a_1 = c1.a;
    let b_1 = c1.b;
    let L_2 = c2.L;
    let a_2 = c2.a;
    let b_2 = c2.b;

    // Step 1 - Calculate C'i, h'i
    let C_1_ab = hypot(a_1, b_1);
    let C_2_ab = hypot(a_2, b_2);
    let C_bar_ab = (C_1_ab + C_2_ab) / 2.0;
    let G = 0.5
        * (1.0 - sqrt(C_bar_ab.powi(7) / (C_bar_ab.powi(7) + 25.0f32.powi(7))));
    // println!("G: {}", G);
    let a_p_1 = (1.0 + G) * a_1;
    // println!("a_p_1: {}", a_p_1);
    let a_p_2 = (1.0 + G) * a_2;
    // println!("a_p_2: {}", a_p_2);
    let C_p_1 = hypot(a_p_1, b_1);
    // println!("C_p_1: {}", C_p_1);
    let C_p_2 = hypot(a_p_2, b_2);
    // println!("C_p_2: {}", C_p_2);
    let h_p_1 = atan2(b_1, a_p_1).to_degrees();
    let h_p_1 = if h_p_1 < 0.0 { h_p_1 + 360.0 } else { h_p_1 };
    // println!("h_p_1: {}", h_p_1);
    let h_p_2 = atan2(b_2, a_p_2).to_degrees();
    let h_p_2 = if h_p_2 < 0.0 { h_p_2 + 360.0 } else { h_p_2 };
    // println!("h_p_2: {}", h_p_2);

    // Step 2 - Calculate ΔL′, ΔC′, ΔH′
    let delta_L_p = L_2 - L_1;
    let delta_C_p = C_p_2 - C_p_1;
    let delta_h_p = h_p_2 - h_p_1;
    let delta_h_p = if C_p_1 * C_p_2 == 0.0 {
        0.0
    } else {
        if abs(delta_h_p) <= 180.0 {
            delta_h_p
        } else if delta_h_p > 180.0 {
            delta_h_p - 360.0
        } else {
            delta_h_p + 360.0
        }
    };
    let delta_H_p =
        2.0 * sqrt(C_p_1 * C_p_2) * sin((delta_h_p / 2.0).to_radians());

    // Step 3 - Calculate ΔE₀₀
    let L_bar_p = (L_1 + L_2) / 2.0;
    let C_bar_p = (C_p_1 + C_p_2) / 2.0;

    let h_1_m_h_2 = abs(h_p_1 - h_p_2);
    let h_bar_p = if C_p_1 * C_p_2 == 0.0 {
        h_p_1 + h_p_2
    } else {
        if h_1_m_h_2 <= 180.0 {
            (h_p_1 + h_p_2) / 2.0
        } else if (h_p_1 + h_p_2) < 360.0 {
            (h_p_1 + h_p_2 + 360.0) / 2.0
        } else {
            (h_p_1 + h_p_2 - 360.0) / 2.0
        }
    };
    // println!("h_bar_p: {}", h_bar_p);

    let T = 1.0 - 0.17 * cos((h_bar_p - 30.0).to_radians())
        + 0.24 * cos((2.0 * h_bar_p).to_radians())
        + 0.32 * cos((3.0 * h_bar_p + 6.0).to_radians())
        - 0.20 * cos((4.0 * h_bar_p - 63.0).to_radians());
    // println!("T: {}", T);

    let delta_theta = 30.0 * exp(-sqr((h_bar_p - 275.0) / 25.0));

    let R_C = 2.0 * sqrt(C_bar_p.powi(7) / (C_bar_p.powi(7) + 25.0f32.powi(7)));

    let S_L =
        1.0 + (0.015 * sqr(L_bar_p - 50.0)) / sqrt(20.0 + sqr(L_bar_p - 50.0));
    let S_C = 1.0 + 0.045 * C_bar_p;
    let S_H = 1.0 + 0.015 * C_bar_p * T;
    let R_T = -sin((2.0 * delta_theta).to_radians()) * R_C;

    const K_L: f32 = 1.0;
    const K_C: f32 = 1.0;
    const K_H: f32 = 1.0;

    sqrt(
        sqr(delta_L_p / (K_L * S_L))
            + sqr(delta_C_p / (K_C * S_C))
            + sqr(delta_H_p / (K_H * S_H))
            + R_T * ((delta_C_p / (K_C * S_C)) * (delta_H_p / (K_H * S_H))),
    )
}

#[cfg(test)]
fn round_to_places(x: f32, p: i32) -> f32 {
    (x * 10f32.powi(p)).round() / 10f32.powi(p)
}

#[test]
#[allow(non_snake_case)]
fn test_delta_e() {
    let c1_1 = lab(50.0, 2.6772, -79.7751);
    let c1_2 = lab(50.0, 0.0000, -82.7485);
    let dE_1 = delta_E_2000(c1_1, c1_2);
    assert_eq!(round_to_places(dE_1, 4), 2.0425);

    let c2_1 = lab(50.0000, 3.1571, -77.2803);
    let c2_2 = lab(50.0000, 0.0000, -82.7485);
    let dE_2 = delta_E_2000(c2_1, c2_2);
    assert_eq!(round_to_places(dE_2, 4), 2.8615);

    let c3_1 = lab(50.0000, 2.8361, -74.0200);
    let c3_2 = lab(50.0000, 0.0000, -82.7485);
    let dE_3 = delta_E_2000(c3_1, c3_2);
    assert_eq!(round_to_places(dE_3, 4), 3.4412);

    let c4_1 = lab(50.0000, -1.3802, -84.2814);
    let c4_2 = lab(50.0000, 0.0000, -82.7485);
    let dE_4 = delta_E_2000(c4_1, c4_2);
    assert_eq!(round_to_places(dE_4, 4), 1.0);

    let c5_1 = lab(50.0000, -1.1848, -84.8006);
    let c5_2 = lab(50.0000, 0.0000, -82.7485);
    let dE_5 = delta_E_2000(c5_1, c5_2);
    assert_eq!(round_to_places(dE_5, 4), 1.0);

    let c6_1 = lab(50.0000, -0.9009, -85.5211);
    let c6_2 = lab(50.0000, 0.0000, -82.7485);
    let dE_6 = delta_E_2000(c6_1, c6_2);
    assert_eq!(round_to_places(dE_6, 4), 1.0);

    let c7_1 = lab(50.0000, 0.0, 0.0);
    let c7_2 = lab(50.0000, -1.0, 2.0);
    let dE_7 = delta_E_2000(c7_1, c7_2);
    assert_eq!(round_to_places(dE_7, 4), 2.3669);

    let c8_1 = lab(50.0000, -1.0, 2.0);
    let c8_2 = lab(50.0000, 0.0, 0.0);
    let dE_8 = delta_E_2000(c8_1, c8_2);
    assert_eq!(round_to_places(dE_8, 4), 2.3669);

    let c9_1 = lab(50.0000, 2.49, -0.001);
    let c9_2 = lab(50.0000, -2.49, 0.0009);
    let dE_9 = delta_E_2000(c9_1, c9_2);
    assert_eq!(round_to_places(dE_9, 4), 7.1792);

    let c10_1 = lab(50.0000, 2.49, -0.001);
    let c10_2 = lab(50.0000, -2.49, 0.001);
    let dE_10 = delta_E_2000(c10_1, c10_2);
    assert_eq!(round_to_places(dE_10, 4), 7.1792);

    let c11_1 = lab(50.0000, 2.49, -0.001);
    let c11_2 = lab(50.0000, -2.49, 0.0011);
    let dE_11 = delta_E_2000(c11_1, c11_2);
    assert_eq!(round_to_places(dE_11, 4), 7.2195);

    let c12_1 = lab(50.0000, 2.49, -0.001);
    let c12_2 = lab(50.0000, -2.49, 0.0012);
    let dE_12 = delta_E_2000(c12_1, c12_2);
    assert_eq!(round_to_places(dE_12, 4), 7.2195);

    let c13_1 = lab(50.0000, -0.001, 2.49);
    let c13_2 = lab(50.0000, 0.0009, -2.49);
    let dE_13 = delta_E_2000(c13_1, c13_2);
    assert_eq!(round_to_places(dE_13, 4), 4.8045);

    let c14_1 = lab(50.0000, -0.001, 2.49);
    let c14_2 = lab(50.0000, 0.001, -2.49);
    let dE_14 = delta_E_2000(c14_1, c14_2);
    assert_eq!(round_to_places(dE_14, 4), 4.8045);
}
