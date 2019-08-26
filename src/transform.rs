use super::chromatic_adaptation::*;
use super::chromaticity::*;
use super::color_space_rgb::*;
use super::math::*;
use super::rgb::*;
use super::xyz::*;

pub fn xyz_to_rgb_matrix<T>(xyz_white: XYY<T>, color_space_rgb: &ColorSpaceRGB<T>) -> Matrix33<T>
where
    T: Real,
{
    color_space_rgb.xf_xyz_to_rgb * cat02(xyz_white, color_space_rgb.white)
}

pub fn xyz_to_rgb_matrix_with_cat<T>(
    cat_mtx: &Matrix33<T>,
    color_space_rgb: &ColorSpaceRGB<T>,
) -> Matrix33<T>
where
    T: Real,
{
    color_space_rgb.xf_xyz_to_rgb * (*cat_mtx)
}

pub fn rgb_to_xyz_matrix<T>(xyz_white: XYY<T>, color_space_rgb: &ColorSpaceRGB<T>) -> Matrix33<T>
where
    T: Real,
{
    cat02(color_space_rgb.white, xyz_white) * color_space_rgb.xf_rgb_to_xyz
}

pub fn xyz_to_rgb<T>(mtx: &Matrix33<T>, xyz: XYZ<T>) -> RGBf<T>
where
    T: Real,
{
    let x = *mtx * (xyz / XYZ::<T>::from_scalar(T::from(100.0).unwrap()));
    rgbf(x.x, x.y, x.z)
}

pub fn xyz_slice_to_rgb<T>(mtx: &Matrix33<T>, xyzs: &[XYZ<T>]) -> Vec<RGBf<T>>
where
    T: Real,
{
    let mut result = Vec::with_capacity(xyzs.len());
    for xyz in xyzs {
        let x = *mtx * (*xyz / XYZ::<T>::from_scalar(T::from(100.0).unwrap()));
        result.push(rgbf(x.x, x.y, x.z))
    }

    result
}

pub fn rgb_to_xyz<T>(mtx: &Matrix33<T>, rgb: RGBf<T>) -> XYZ<T>
where
    T: Real,
{
    let x = *mtx * rgb;
    XYZ::new(x.r, x.g, x.b) * XYZ::from_scalar(T::from(100.0).unwrap())
}

pub fn rgb_to_rgb_matrix<T>(
    from_space: &ColorSpaceRGB<T>,
    to_space: &ColorSpaceRGB<T>,
) -> Matrix33<T>
where
    T: Real,
{
    to_space.xf_xyz_to_rgb
        * cat02(from_space.white, to_space.white)
        * from_space.xf_rgb_to_xyz
}

#[cfg(all(target_arch = "x86_64", target_feature = "avx"))]
pub fn xyz_slice_to_rgb_avx_planes<S:simdeez::Simd>(mtx: &M3f32, xs: &[f32], ys: &[f32], zs: &[f32]) -> (Vec<f32>, Vec<f32>, Vec<f32>) {
    let mut result_r = vec![0.0f32; xs.len()];
    let mut result_g = vec![0.0f32; xs.len()];
    let mut result_b = vec![0.0f32; xs.len()];
    let num_iters = xs.len() / S::VF32_WIDTH;
    let start_remaining = xs.len() - num_iters;

    let m0 = unsafe { S::set1_ps(mtx.x[0])};
    let m1 = unsafe { S::set1_ps(mtx.x[1])};
    let m2 = unsafe { S::set1_ps(mtx.x[2])};
    let m3 = unsafe { S::set1_ps(mtx.x[3])};
    let m4 = unsafe { S::set1_ps(mtx.x[4])};
    let m5 = unsafe { S::set1_ps(mtx.x[5])};
    let m6 = unsafe { S::set1_ps(mtx.x[6])};
    let m7 = unsafe { S::set1_ps(mtx.x[7])};
    let m8 = unsafe { S::set1_ps(mtx.x[8])};

    let scale = unsafe { S::set1_ps(0.01) };

    for i in 0..num_iters {
        unsafe {
            // First calculate memory indices for this loop operation
            // Gather memory to registers
            let x = S::loadu_ps(xs.get_unchecked(i*S::VF32_WIDTH)) * scale;
            let y = S::loadu_ps(ys.get_unchecked(i*S::VF32_WIDTH)) * scale;
            let z = S::loadu_ps(zs.get_unchecked(i*S::VF32_WIDTH)) * scale;

            // Matrix multiplication
            let r = m0 * x;
            let r = S::fmadd_ps(m1, y, r);
            let r = S::fmadd_ps(m2, z, r);

            let g = m3 * x;
            let g = S::fmadd_ps(m4, y, g);
            let g = S::fmadd_ps(m5, z, g);

            let b = m6 * x;
            let b = S::fmadd_ps(m7, y, b);
            let b = S::fmadd_ps(m8, z, b);

            // Store results
            S::storeu_ps(result_r.get_unchecked_mut(i*S::VF32_WIDTH), r);
            S::storeu_ps(result_g.get_unchecked_mut(i*S::VF32_WIDTH), g);
            S::storeu_ps(result_b.get_unchecked_mut(i*S::VF32_WIDTH), b);
        }
    }

    use itertools::izip;
    for (r, g, b, x, y, z) in izip!(
        result_r.iter_mut().skip(start_remaining),
        result_g.iter_mut().skip(start_remaining),
        result_b.iter_mut().skip(start_remaining),
        xs.iter().skip(start_remaining),
        ys.iter().skip(start_remaining),
        zs.iter().skip(start_remaining),
    ) {
        let x = *mtx * XYZf32::new(*x * 0.01, *y * 0.01, *z * 0.01);
        *r = x.x;
        *g = x.y;
        *b = x.z;
    }

    (result_r, result_g, result_b)
}

#[cfg(all(target_arch = "x86_64", target_feature = "avx"))]
#[test]
fn test_checker_xyz_to_rgb_avx_planes() {
    use crate::math::*;
    use crate::colorchecker;
    use float_cmp::{ApproxEq, F32Margin};
    use simdeez::avx2::*;

    let xyz_to_rgb_mtx: M3f32 = xyz_to_rgb_matrix(model_f64::SRGB.white, &model_f64::SRGB).into();

    let xs = colorchecker::NAMES.iter().map(|n| colorchecker::XYZ_D65[*n].x as f32).collect::<Vec<_>>();
    let ys = colorchecker::NAMES.iter().map(|n| colorchecker::XYZ_D65[*n].y as f32).collect::<Vec<_>>();
    let zs = colorchecker::NAMES.iter().map(|n| colorchecker::XYZ_D65[*n].z as f32).collect::<Vec<_>>();

    let (rr, rg, rb) = xyz_slice_to_rgb_avx_planes::<Avx2>(&xyz_to_rgb_mtx, &xs, &ys, &zs);

    use itertools::izip;
    for (r, g, b, name) in izip!(rr.into_iter(), rg.into_iter(), rb.into_iter(), colorchecker::NAMES.iter()) {
        let rgb = rgbf32(r, g, b);
        let rgb_ref = RGBf32::from(colorchecker::SRGB_LINEAR[*name]);
        println!("{} rgb: {}", name, rgb);
        println!("{} ref: {}", name, rgb_ref);
        assert!(
            rgb.approx_eq(
            rgb_ref,
            F32Margin {
                epsilon: 1e-7,
                ulps: 2
            }
        ));
    }
    
}
