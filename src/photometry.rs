use crate::vspd::*;
use crate::cmf::CMF;

pub fn spd_to_nit(spd: &VSPD, cmf: &CMF) -> f64 {
    // should probably do a modified verison of ASTM E-308 here but for
    // now just do a straight interpolated integration
    let cmf = cmf.y_bar.align(spd.shape());
    let s = spd.values().zip(cmf.values()).map(|(s, y)| s * y).sum::<f64>();

    s * 683.0 / spd.len() as f64
}