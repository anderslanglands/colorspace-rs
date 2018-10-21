#![allow(non_snake_case)]
use super::math::*;
use super::xyz::*;
use super::chromaticity::*;

pub fn bradford(wp_src: Chromaticity, wp_dst: Chromaticity) -> Matrix33 {
    if wp_src == wp_dst {
        return Matrix33::make_identity();
    }

    #[rustfmt::skip]
    let M_A = Matrix33::new([
        0.8951000, 0.2664000, -0.1614000, 
        -0.7502000, 1.7135000, 0.0367000,
        0.0389000, -0.0685000, 1.0296000,
    ]);
    #[rustfmt::skip]
    let M_A_inv = Matrix33::new([
        0.9869929, -0.1470543, 0.1599627, 
        0.4323053, 0.5183603, 0.0492912,
        -0.0085287, 0.0400428, 0.9684867,
    ]);

    let wp_src_A = M_A * XYZ::from_chromaticity(wp_src, 1.0);
    let wp_dst_A = M_A * XYZ::from_chromaticity(wp_dst, 1.0);

    let M_wp = Matrix33::new([
        wp_dst_A.x / wp_src_A.x, 0.0, 0.0,
        0.0, wp_dst_A.y / wp_src_A.y, 0.0,
        0.0, 0.0, wp_dst_A.z / wp_src_A.z,
    ]);

    M_A_inv * M_wp * M_A
}
