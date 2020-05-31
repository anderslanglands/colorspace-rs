#![allow(non_snake_case)]
#![allow(clippy::excessive_precision, clippy::unreadable_literal)]
use super::math::*;
use super::xyz::*;

use numeric_literals::replace_float_literals;

/// Compute the Bradford chromatic adaptation transform matrix.
/// XYZ colors are specified relative to a reference illuminant. The
/// chromatic adaptation transform allows to adapt from one illuminant
/// to another.
/// See http://www.brucelindbloom.com for more information.
#[replace_float_literals(T::from(literal).unwrap())]
pub fn bradford<T, X1: Into<XYZ<T>>, X2: Into<XYZ<T>>>(
    wp_src: X1,
    wp_dst: X2,
) -> Matrix33<T>
where
    T: Real,
{
    let wp_src: XYZ<T> = wp_src.into();
    let wp_dst: XYZ<T> = wp_dst.into();
    if wp_src == wp_dst {
        return Matrix33::<T>::make_identity();
    }

    #[rustfmt::skip]
    let M_A = Matrix33::<T>::new([
        0.8951000, 0.2664000, -0.1614000, 
        -0.7502000, 1.7135000, 0.0367000,
        0.0389000, -0.0685000, 1.0296000,
    ]);
    let M_A_inv = M_A.inverse().unwrap();

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

/// Compute the Von Kries chromatic adaptation transform matrix.
/// XYZ colors are specified relative to a reference illuminant. The
/// chromatic adaptation transform allows to adapt from one illuminant
/// to another.
/// See http://www.brucelindbloom.com for more information.
#[replace_float_literals(T::from(literal).unwrap())]
pub fn von_kries<T, X1: Into<XYZ<T>>, X2: Into<XYZ<T>>>(
    wp_src: X1,
    wp_dst: X2,
) -> Matrix33<T>
where
    T: Real,
{
    let wp_src: XYZ<T> = wp_src.into();
    let wp_dst: XYZ<T> = wp_dst.into();
    if wp_src == wp_dst {
        return Matrix33::<T>::make_identity();
    }

    #[rustfmt::skip]
    let M_A = Matrix33::<T>::new([
        0.4002400,  0.7076000, -0.0808100,
       -0.2263000,  1.1653200,  0.0457000,
        0.0000000,  0.0000000,  0.9182200,
    ]);
    #[rustfmt::skip]
    let M_A_inv = Matrix33::<T>::new([
        1.8599364, -1.1293816,  0.2198974,
        0.3611914,  0.6388125, -0.0000064,
        0.0000000,  0.0000000,  1.0890636,
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

/// Compute the CAT02 chromatic adaptation transform matrix.
/// XYZ colors are specified relative to a reference illuminant. The
/// chromatic adaptation transform allows to adapt from one illuminant
/// to another.
/// See https://en.wikipedia.org/wiki/CIECAM02#CAT02 for more information.
#[replace_float_literals(T::from(literal).unwrap())]
pub fn cat02<T, X1: Into<XYZ<T>>, X2: Into<XYZ<T>>>(
    wp_src: X1,
    wp_dst: X2,
) -> Matrix33<T>
where
    T: Real,
{
    let wp_src: XYZ<T> = wp_src.into();
    let wp_dst: XYZ<T> = wp_dst.into();
    if wp_src == wp_dst {
        return Matrix33::<T>::make_identity();
    }

    let wp_src = wp_src;
    let wp_dst = wp_dst;

    #[rustfmt::skip]
    let M_A = Matrix33::<T>::new([
        0.7328, 0.4296, -0.1624,
       -0.7036, 1.6975,  0.0061,
        0.0030, 0.0136,  0.9834,
    ]);
    #[rustfmt::skip]
    let M_A_inv = M_A.inverse().unwrap();

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
