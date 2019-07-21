use colorspace::*;
use rand::prelude::*;

const NUM_SAMPLES: usize = 16384;
const SPD_START: f64 = 360.0;
const SPD_END: f64 = 780.0;
const SPD_RANGE: f64 = SPD_END - SPD_START;

fn main() {
    let x_bar = InterpolatorSprague::<f64>::new(&cmf::CIE_1931_2_DEGREE.x_bar);
    let y_bar = InterpolatorSprague::<f64>::new(&cmf::CIE_1931_2_DEGREE.y_bar);
    let z_bar = InterpolatorSprague::<f64>::new(&cmf::CIE_1931_2_DEGREE.z_bar);

    let d65 = InterpolatorSprague::<f64>::new(&illuminant::spd::D65);

    let mtx: M3f64 = xyz_to_rgb_matrix(SRGB.white, &SRGB);

    let cat_d65_to_d50: M3f64 = chromatic_adaptation::cat02(illuminant::xy::D65, illuminant::xy::D50);

    let mut rng = rand::thread_rng();
    let mut dE_min = std::f64::MAX;
    let mut dE_max = std::f64::MIN;
    let mut dE_mean = 0.0f64;
    for name in colorchecker::NAMES.iter() {
        let spd = &colorchecker::SPECTRAL[*name];
        let swatch = InterpolatorSprague::<f64>::new(&spd.align(SpdShape::new(SPD_START, SPD_END, 1.0)));

        let mut xyz = XYZf64::from_scalar(0.0);
        let mut xyz_w = XYZf64::from_scalar(0.0);
        for i in 0..NUM_SAMPLES {
            // stratified sample pattern
            let x = ((i as f64) + rng.gen::<f64>()) / NUM_SAMPLES as f64;

            // initialize hero wavelengths
            let h = SPD_START + x * SPD_RANGE;
            let lambda = {
                let mut lambda = [
                    h,
                    h + SPD_RANGE / 4.0,
                    h + 2.0 * SPD_RANGE / 4.0,
                    h + 3.0 * SPD_RANGE / 4.0,
                ];

                for l in lambda.iter_mut() {
                    if *l >= SPD_END {
                        *l -= SPD_RANGE;
                    }
                }

                lambda
            };

            // accumulate
            for l in lambda.iter() {
                let s = d65.evaluate(*l) * swatch.evaluate(*l);
                xyz += XYZf64::new(
                    x_bar.evaluate(*l) * s,
                    y_bar.evaluate(*l) * s,
                    z_bar.evaluate(*l) * s,
                );
                xyz_w += XYZf64::new(
                    x_bar.evaluate(*l) * d65.evaluate(*l),
                    y_bar.evaluate(*l) * d65.evaluate(*l),
                    z_bar.evaluate(*l) * d65.evaluate(*l),
                );
            }
        }

        xyz = xyz / XYZf64::from_scalar(NUM_SAMPLES as f64);
        xyz_w = xyz_w / XYZf64::from_scalar(NUM_SAMPLES as f64);
        let rgb = xyz_to_rgb(&mtx, xyz * 100.0 / xyz_w.y);
        let rgb_w = xyz_to_rgb(&mtx, xyz_w * 100.0 / xyz_w.y);
        // println!("----------------");
        // println!("xyz wht {}: {}", name, xyz_w);
        // println!("rgb wht {}: {}", name, rgb_w);
        // println!("hws rgb {}: {}", name, rgb);
        // println!("nrm rgb {}: {}", name, rgb / rgb_w);

        let xyz_ref = colorchecker::SPECTRAL[*name].align(SpdShape::astm_e308()).to_xyz(&illuminant::spd::D65, &cmf::CIE_1931_2_DEGREE);
        let xyz_d50 = cat_d65_to_d50 * xyz;
        let xyz_ref_d50 = cat_d65_to_d50 * xyz_ref;
        let dE = delta_E(xyz_to_lab(xyz_d50, illuminant::xy::D50),
        xyz_to_lab(xyz_ref_d50, illuminant::xy::D50));
        // println!("dE: {}", dE);

        dE_mean += dE;
        dE_min = dE_min.min(dE);
        dE_max = dE_max.max(dE);
    }

    dE_mean /= 24.0;
    println!("======== SPECTRAL -> XYZ ========");
    println!("dE min : {}", dE_min);
    println!("dE max : {}", dE_max);
    println!("dE mean: {}", dE_mean);

    // test Mallett & Yuksel uplifting with Sprague interpolation
    let mut rng = rand::thread_rng();
    let mut dE_min = std::f64::MAX;
    let mut dE_max = std::f64::MIN;
    let mut dE_mean = 0.0f64;

    let my_r = InterpolatorSprague::<f64>::new(&uplifting::MY_RED.align(SpdShape::new(SPD_START, SPD_END, 1.0)));
    let my_g = InterpolatorSprague::<f64>::new(&uplifting::MY_GREEN.align(SpdShape::new(SPD_START, SPD_END, 1.0)));
    let my_b = InterpolatorSprague::<f64>::new(&uplifting::MY_BLUE.align(SpdShape::new(SPD_START, SPD_END, 1.0)));

    for name in colorchecker::NAMES.iter() {
        let rgb_ref = colorchecker::SRGB_LINEAR[*name];
        let mut xyz = XYZf64::from_scalar(0.0);
        let mut xyz_w = XYZf64::from_scalar(0.0);
        for i in 0..NUM_SAMPLES {
            // stratified sample pattern
            let x = ((i as f64) + rng.gen::<f64>()) / NUM_SAMPLES as f64;

            // initialize hero wavelengths
            let h = SPD_START + x * SPD_RANGE;
            let lambda = {
                let mut lambda = [
                    h,
                    h + SPD_RANGE / 4.0,
                    h + 2.0 * SPD_RANGE / 4.0,
                    h + 3.0 * SPD_RANGE / 4.0,
                ];

                for l in lambda.iter_mut() {
                    if *l >= SPD_END {
                        *l -= SPD_RANGE;
                    }
                }

                lambda
            };

            // accumulate
            for l in lambda.iter() {
                let rho = my_r.evaluate(*l) * rgb_ref.r 
                        + my_g.evaluate(*l) * rgb_ref.g 
                        + my_b.evaluate(*l) * rgb_ref.b;

                let s = d65.evaluate(*l) * rho; 
                xyz += XYZf64::new(
                    x_bar.evaluate(*l) * s,
                    y_bar.evaluate(*l) * s,
                    z_bar.evaluate(*l) * s,
                );
                xyz_w += XYZf64::new(
                    x_bar.evaluate(*l) * d65.evaluate(*l),
                    y_bar.evaluate(*l) * d65.evaluate(*l),
                    z_bar.evaluate(*l) * d65.evaluate(*l),
                );
            }
        }

        xyz = xyz / XYZf64::from_scalar(NUM_SAMPLES as f64);
        xyz_w = xyz_w / XYZf64::from_scalar(NUM_SAMPLES as f64);
        let rgb = xyz_to_rgb(&mtx, xyz * 100.0 / xyz_w.y);
        let rgb_w = xyz_to_rgb(&mtx, xyz_w * 100.0 / xyz_w.y);
        // println!("----------------");
        // println!("xyz wht {}: {}", name, xyz_w);
        // println!("rgb wht {}: {}", name, rgb_w);
        // println!("hws rgb {}: {}", name, rgb);
        // println!("nrm rgb {}: {}", name, rgb / rgb_w);
        // println!("ref rgb {}: {}", name, rgb_ref);

        let xyz_ref = colorchecker::SPECTRAL[*name].align(SpdShape::astm_e308()).to_xyz(&illuminant::spd::D65, &cmf::CIE_1931_2_DEGREE);
        let xyz_d50 = cat_d65_to_d50 * xyz;
        let xyz_ref_d50 = cat_d65_to_d50 * xyz_ref;
        let dE = delta_E(xyz_to_lab(xyz_d50, illuminant::xy::D50),
        xyz_to_lab(xyz_ref_d50, illuminant::xy::D50));
        // println!("dE: {}", dE);

        dE_mean += dE;
        dE_min = dE_min.min(dE);
        dE_max = dE_max.max(dE);
    }

    dE_mean /= 24.0;
    println!("======== RGB -> SPECTRAL -> XYZ (Mallett & Yuksel, Sprague)");
    println!("dE min : {}", dE_min);
    println!("dE max : {}", dE_max);
    println!("dE mean: {}", dE_mean);


    // test Mallett & Yuksel uplifting with Linear interpolation
    let mut rng = rand::thread_rng();
    let mut dE_min = std::f64::MAX;
    let mut dE_max = std::f64::MIN;
    let mut dE_mean = 0.0f64;

    let my_r = InterpolatorLinear::new(&uplifting::MY_RED);
    let my_g = InterpolatorLinear::new(&uplifting::MY_GREEN);
    let my_b = InterpolatorLinear::new(&uplifting::MY_BLUE);

    for name in colorchecker::NAMES.iter() {
        let rgb_ref = colorchecker::SRGB_LINEAR[*name];
        let mut xyz = XYZf64::from_scalar(0.0);
        let mut xyz_w = XYZf64::from_scalar(0.0);
        for i in 0..NUM_SAMPLES {
            // stratified sample pattern
            let x = ((i as f64) + rng.gen::<f64>()) / NUM_SAMPLES as f64;

            // initialize hero wavelengths
            let h = SPD_START + x * SPD_RANGE;
            let lambda = {
                let mut lambda = [
                    h,
                    h + SPD_RANGE / 4.0,
                    h + 2.0 * SPD_RANGE / 4.0,
                    h + 3.0 * SPD_RANGE / 4.0,
                ];

                for l in lambda.iter_mut() {
                    if *l >= SPD_END {
                        *l -= SPD_RANGE;
                    }
                }

                lambda
            };

            // accumulate
            for l in lambda.iter() {
                let rho = my_r.evaluate(*l) * rgb_ref.r 
                        + my_g.evaluate(*l) * rgb_ref.g 
                        + my_b.evaluate(*l) * rgb_ref.b;

                let s = d65.evaluate(*l) * rho; 
                xyz += XYZf64::new(
                    x_bar.evaluate(*l) * s,
                    y_bar.evaluate(*l) * s,
                    z_bar.evaluate(*l) * s,
                );
                xyz_w += XYZf64::new(
                    x_bar.evaluate(*l) * d65.evaluate(*l),
                    y_bar.evaluate(*l) * d65.evaluate(*l),
                    z_bar.evaluate(*l) * d65.evaluate(*l),
                );
            }
        }

        xyz = xyz / XYZf64::from_scalar(NUM_SAMPLES as f64);
        xyz_w = xyz_w / XYZf64::from_scalar(NUM_SAMPLES as f64);
        let rgb = xyz_to_rgb(&mtx, xyz * 100.0 / xyz_w.y);
        let rgb_w = xyz_to_rgb(&mtx, xyz_w * 100.0 / xyz_w.y);
        // println!("----------------");
        // println!("xyz wht {}: {}", name, xyz_w);
        // println!("rgb wht {}: {}", name, rgb_w);
        // println!("hws rgb {}: {}", name, rgb);
        // println!("nrm rgb {}: {}", name, rgb / rgb_w);
        // println!("ref rgb {}: {}", name, rgb_ref);

        let xyz_ref = colorchecker::SPECTRAL[*name].align(SpdShape::astm_e308()).to_xyz(&illuminant::spd::D65, &cmf::CIE_1931_2_DEGREE);
        let xyz_d50 = cat_d65_to_d50 * xyz;
        let xyz_ref_d50 = cat_d65_to_d50 * xyz_ref;
        let dE = delta_E(xyz_to_lab(xyz_d50, illuminant::xy::D50),
        xyz_to_lab(xyz_ref_d50, illuminant::xy::D50));
        // println!("dE: {}", dE);

        dE_mean += dE;
        dE_min = dE_min.min(dE);
        dE_max = dE_max.max(dE);
    }

    dE_mean /= 24.0;
    println!("======== RGB -> SPECTRAL -> XYZ (Mallett & Yuksel, Linear)");
    println!("dE min : {}", dE_min);
    println!("dE max : {}", dE_max);
    println!("dE mean: {}", dE_mean);
}