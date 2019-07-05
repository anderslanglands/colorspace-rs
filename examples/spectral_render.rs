//! Do a very simple spectral render by sampling an sRGB image,
//! upsampling to SPD, then converting back to sRGB for output

use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use colorspace::prelude::*;

use image::{DynamicImage, ImageFormat};

use rand::prelude::*;

use rayon::prelude::*;

fn main() {
    let mut f = File::open("resources/marcie_sRGB.png")
        .expect("Could not open marcie image");

    let marcie_dyn_img = match image::load(BufReader::new(f), ImageFormat::PNG)
    {
        Ok(img) => img,
        Err(e) => {
            panic!("Could not read marcie image: {}", e);
        }
    };

    // extract the image and get its dimensions
    let marcie_img = match marcie_dyn_img {
        DynamicImage::ImageRgb8(img) => img,
        _ => {
            panic!("Unhandled image data format");
        }
    };

    let w = marcie_img.width();
    let h = marcie_img.height();

    // Convert the image data to a vec of linear srgb f32
    let img_orig_lin_srgb: Vec<f32> = marcie_img
        .into_raw()
        .into_iter()
        .map(|u| eotf::srgb_f32((u as f32) / 255.0))
        .collect();

    // and to RGBf32
    let img_orig_lin_srgb: Vec<RGBf32> = img_orig_lin_srgb
        .chunks(3)
        .map(|c| rgbf32(c[0], c[1], c[2]))
        .collect();

    // now iterate over the image, and sample 64 times with a
    // spectral upsampling.
    const NUM_SAMPLES: usize = 128;

    let img_sampled_smits: Vec<XYZ> = img_orig_lin_srgb
        .par_iter()
        .map(|rgb| {
            let mut xyz = XYZ::zero();
            let mut rng = rand::thread_rng();
            for _ in 0..NUM_SAMPLES {
                let l_h = rng.gen::<f32>() * LAMBDA_RANGE + LAMBDA_START;

                // convert according to smits
                let mut hws = HWS::new(l_h);
                hws.from_rgb_smits(*rgb);
                xyz += hws.to_xyz();
            }

            xyz / NUM_SAMPLES as f32
        })
        .collect();

    let xf_xyz_to_srgb = xyz_to_rgb_matrix(
        colorspace::ITUR_BT709.white,
        &colorspace::ITUR_BT709,
    );

    let mut img_out_smits = Vec::new();
    for xyz in img_sampled_smits {
        let rgb = xyz_to_rgb(&xf_xyz_to_srgb, xyz);
        let srgb = RGBu8::from(oetf::srgb(rgb));
        img_out_smits.push(srgb.r);;
        img_out_smits.push(srgb.g);;
        img_out_smits.push(srgb.b);;
    }

    image::save_buffer(
        "marcie_smits.png",
        img_out_smits.as_slice(),
        w,
        h,
        image::ColorType::RGB(8),
    )
    .unwrap();

    let img_sampled_mallett: Vec<XYZ> = img_orig_lin_srgb
        .par_iter()
        .map(|rgb| {
            let mut xyz = XYZ::zero();
            let mut rng = rand::thread_rng();
            for _ in 0..NUM_SAMPLES {
                let l_h = rng.gen::<f32>() * LAMBDA_RANGE + LAMBDA_START;

                // convert according to mallett
                let mut hws = HWS::new(l_h);
                hws.from_rgb_mallett(*rgb);
                xyz += hws.to_xyz();
            }

            xyz / NUM_SAMPLES as f32
        })
        .collect();

    let xf_xyz_to_srgb = xyz_to_rgb_matrix(
        colorspace::ITUR_BT709.white,
        &colorspace::ITUR_BT709,
    );

    let mut img_out_mallett = Vec::new();
    for xyz in img_sampled_mallett {
        let rgb = xyz_to_rgb(&xf_xyz_to_srgb, xyz);
        let srgb = RGBu8::from(oetf::srgb(rgb));
        img_out_mallett.push(srgb.r);;
        img_out_mallett.push(srgb.g);;
        img_out_mallett.push(srgb.b);;
    }

    image::save_buffer(
        "marcie_mallett.png",
        img_out_mallett.as_slice(),
        w,
        h,
        image::ColorType::RGB(8),
    )
    .unwrap();
}

const LAMBDA_START: f32 = 380.0;
const LAMBDA_END: f32 = 780.0;
const LAMBDA_RANGE: f32 = LAMBDA_END - LAMBDA_START;

struct HWS {
    pub lambda: [f32; 4],
    pub value: [f32; 4],
}

impl HWS {
    pub fn new(l_0: f32) -> HWS {
        let l_1 = {
            let l_1 = l_0 + (1.0 * LAMBDA_RANGE / 4.0);
            if l_1 < 780.0 {
                l_1
            } else {
                l_1 - LAMBDA_RANGE
            }
        };
        let l_2 = {
            let l_2 = l_0 + (2.0 * LAMBDA_RANGE / 4.0);
            if l_2 < 780.0 {
                l_2
            } else {
                l_2 - LAMBDA_RANGE
            }
        };
        let l_3 = {
            let l_3 = l_0 + (3.0 * LAMBDA_RANGE / 4.0);
            if l_3 < 780.0 {
                l_3
            } else {
                l_3 - LAMBDA_RANGE
            }
        };

        HWS {
            lambda: [l_0, l_1, l_2, l_3],
            value: [0.0, 0.0, 0.0, 0.0],
        }
    }

    pub fn from_rgb_smits(&mut self, rgb: RGBf32) {
        for (l, v) in self.lambda.iter().zip(self.value.iter_mut()) {
            *v = rgb_to_spd_smits_refl_single(rgb, *l);
        }
    }

    pub fn from_rgb_mallett(&mut self, rgb: RGBf32) {
        for (l, v) in self.lambda.iter().zip(self.value.iter_mut()) {
            *v = rgb_to_spd_mallett_single(rgb, *l);
        }
    }

    pub fn to_xyz(&self) -> XYZ {
        let mut xyz = XYZ::zero();
        let mut N = 0.0f32;
        for (l, v) in self.lambda.iter().zip(self.value.iter()) {
            let M_e = *v * illuminant::D65.spd.value_at(*l);
            xyz.x += cmf::CIE_1931_2_DEGREE.x_bar.value_at(*l) * M_e;
            xyz.y += cmf::CIE_1931_2_DEGREE.y_bar.value_at(*l) * M_e;
            xyz.z += cmf::CIE_1931_2_DEGREE.z_bar.value_at(*l) * M_e;
            N += cmf::CIE_1931_2_DEGREE.y_bar.value_at(*l)
                * illuminant::D65.spd.value_at(*l);
        }

        xyz / N
    }
}
