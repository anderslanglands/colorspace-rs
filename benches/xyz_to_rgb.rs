#[macro_use]
extern crate criterion;

use criterion::Criterion;
use criterion::black_box;

use colorspace::*;
use colorspace::color_space_rgb::model_f64;

fn convert_checker_xyz_to_rgb64(mtx: &M3f64, xyzs: &[XYZf64]) {
    for xyz in xyzs {
        let rgb = xyz_to_rgb(mtx, *xyz);
        black_box(rgb);
    }
}

fn convert_checker_xyz_to_rgb32(mtx: &M3f32, xyzs: &[XYZf32]) {
    for xyz in xyzs {
        let rgb = xyz_to_rgb(mtx, XYZf32::from(*xyz));
        black_box(rgb);
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    let mtx64: M3f64 = xyz_to_rgb_matrix(model_f64::SRGB.white, &model_f64::SRGB);
    let mtx32: M3f32 = xyz_to_rgb_matrix::<f64>(model_f64::SRGB.white, &model_f64::SRGB).into();

    let xyzs = colorchecker::XYZ_D65.iter().cycle().take(512 * 512 + 17).map(|(_, x)| *x).collect::<Vec<_>>();

    c.bench_function("xyz_to_rgb_f64", move |b| b.iter(|| convert_checker_xyz_to_rgb64(&mtx64, &xyzs)));

    let xyzs = colorchecker::XYZ_D65.iter().cycle().take(512 * 512 + 17).map(|(_, x)| XYZf32::from(*x)).collect::<Vec<_>>();
    c.bench_function("xyz_to_rgb_f32", move |b| b.iter(|| convert_checker_xyz_to_rgb32(&mtx32, &xyzs)));

    let xyzs = colorchecker::XYZ_D65.iter().cycle().take(512 * 512 + 17).map(|(_, x)| XYZf32::from(*x)).collect::<Vec<_>>();
    c.bench_function("xyz_to_rgb_slice_f32", move |b| b.iter(|| black_box(xyz_slice_to_rgb(&mtx32, &xyzs))));

    let mtx32: M3f32 = xyz_to_rgb_matrix::<f64>(model_f64::SRGB.white, &model_f64::SRGB).into();
    let xyzs = colorchecker::XYZ_D65.iter().cycle().take(512 * 512 + 17).map(|(_, x)| XYZf32::from(*x)).collect::<Vec<_>>();
    let xs = xyzs.iter().map(|xyz| xyz.x).collect::<Vec<_>>();
    let ys = xyzs.iter().map(|xyz| xyz.y).collect::<Vec<_>>();
    let zs = xyzs.iter().map(|xyz| xyz.z).collect::<Vec<_>>();
    c.bench_function("xyz_to_rgb_slice_avx_planes", move |b| b.iter(|| black_box(xyz_slice_to_rgb_avx_planes::<simdeez::avx2::Avx2>(&mtx32, &xs, &ys, &zs))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
