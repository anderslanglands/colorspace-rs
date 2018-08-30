//! Explicit conversion to and from SPDs

use super::spectral_power_distribution::SPD;
use super::xyz::XYZ;
use super::cmf::CMF;
use super::traits::*;

pub fn spd_to_xyz(spd: &SPD, cmf: &CMF) -> XYZ {
    let lambda_start = if spd.start() > cmf.x_bar.start() {
        spd.start()
    } else {
        cmf.x_bar.start()
    };
    let lambda_end = if spd.end() < cmf.x_bar.end() {
        spd.end()
    } else {
        cmf.x_bar.end()
    };

    let mut idx_start = 0;
    while spd[idx_start].0 < lambda_start {
        idx_start += 1;
    }

    let mut idx_end = 0;
    while spd[idx_end].0 < lambda_end && idx_end < spd.num_samples() {
        idx_end += 1;
    }

    let mut xyz = XYZ::zero();
    for i in idx_start..idx_end {
        let samp = spd[i];
        xyz.x += samp.1 * cmf.x_bar.value_at(samp.0);
        xyz.y += samp.1 * cmf.y_bar.value_at(samp.0);
        xyz.z += samp.1 * cmf.z_bar.value_at(samp.0);
    }

    xyz
}

pub fn spd_to_xyz_with_illuminant(spd: &SPD, cmf: &CMF, illum: &SPD) -> XYZ {
    let lambda_start = if spd.start() > cmf.x_bar.start() {
        spd.start()
    } else {
        cmf.x_bar.start()
    };
    let lambda_end = if spd.end() < cmf.x_bar.end() {
        spd.end()
    } else {
        cmf.x_bar.end()
    };

    let mut idx_start = 0;
    while spd[idx_start].0 < lambda_start {
        idx_start += 1;
    }

    let mut idx_end = 0;
    while spd[idx_end].0 < lambda_end && idx_end < spd.num_samples() {
        idx_end += 1;
    }

    let mut xyz = XYZ::zero();
    let mut N = 0.0_f32;
    for i in idx_start..idx_end {
        let samp = spd[i];
        let M_e = samp.1 * illum.value_at(samp.0);
        xyz.x += cmf.x_bar.value_at(samp.0) * M_e;
        xyz.y += cmf.y_bar.value_at(samp.0) * M_e;
        xyz.z += cmf.z_bar.value_at(samp.0) * M_e;
        N += cmf.y_bar.value_at(samp.0) * illum.value_at(samp.0);
    }

    xyz / N
}