#![allow(non_snake_case)]
use super::math::*;
use super::xyz::*;

pub fn bradford(wp_src: XYZ, wp_dst: XYZ) -> Matrix33 {
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

    let wp_src_A = M_A * wp_src;
    let wp_dst_A = M_A * wp_dst;

    let M_wp = Matrix33::new([
        wp_dst_A.x / wp_src_A.x,
        0.0,
        0.0,
        0.0,
        wp_dst_A.y / wp_src_A.y,
        0.0,
        0.0,
        0.0,
        wp_dst_A.z / wp_src_A.z,
    ]);

    M_A_inv * M_wp * M_A
}
