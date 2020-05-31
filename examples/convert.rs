use colorspace::*;

fn main() {
    // Read marcie into a Vec of u8 representing packed RGB triples.
    let (px_in_u8, width, height) = read_png("resources/marcie_sRGB.png");

    // We want to convert the u8 pixel data to a multitude of output color
    // spaces for viewing.
    // All colour spaces can be converted to and from XYZ, so we'll first
    // convert our u8 pixel data to RGBf32, then convert that to XYZ, then
    // convert the XYZ data to the desired output spaces, before finally
    // converting back to u8 for writing our output images

    // first get our input pixels as RGBf32
    // Note that if we had an f32 buffer here we could just cast the slice using
    // RGBf32::cast_slice() to avoid a copy.
    let px_srgb: Vec<_> = px_in_u8
        .chunks_exact(3)
        .map(|p| {
            rgbf32(
                p[0] as f32 / 255.0,
                p[1] as f32 / 255.0,
                p[2] as f32 / 255.0,
            )
        })
        .collect();

    // Get references to the color models we're interested in here for brevity
    let srgb = &color_space_rgb::model_f32::SRGB;
    let adobe_rgb = &color_space_rgb::model_f32::ADOBE_RGB_1998;
    let alexa_wide = &color_space_rgb::model_f32::ALEXA_WIDE_GAMUT;
    let dci_p3 = &color_space_rgb::model_f32::DCI_P3;

    let mut px_adobe_rgb_u8 = vec![rgbu8(0, 0, 0); px_srgb.len()];
    let mut px_alexa_wide_u8 = vec![rgbu8(0, 0, 0); px_srgb.len()];
    let mut px_dci_p3_u8 = vec![rgbu8(0, 0, 0); px_srgb.len()];

    // RGB converts the whole slice to a slice of anything that implements
    // From<RGBf<T>>.
    rgb_to_rgb(srgb, adobe_rgb, &px_srgb, &mut px_adobe_rgb_u8);
    rgb_to_rgb(srgb, alexa_wide, &px_srgb, &mut px_alexa_wide_u8);
    rgb_to_rgb(srgb, dci_p3, &px_srgb, &mut px_dci_p3_u8);

    write_png(
        "marcie_AdobeRGB.png",
        width,
        height,
        rgbu8_slice_as_u8(&px_adobe_rgb_u8),
    );
    write_png(
        "marcie_AlexaWide.png",
        width,
        height,
        rgbu8_slice_as_u8(&px_alexa_wide_u8),
    );
    write_png(
        "marcie_DCI-P3.png",
        width,
        height,
        rgbu8_slice_as_u8(&px_dci_p3_u8),
    );
}

use std::fs::File;
fn read_png(filename: &str) -> (Vec<u8>, u32, u32) {
    let decoder = png::Decoder::new(File::open(filename).unwrap());
    let (info, mut reader) = decoder.read_info().unwrap();
    let (width, height) = reader.info().size();
    let mut px_in_u8 = vec![0; info.buffer_size()];
    reader.next_frame(&mut px_in_u8).unwrap();
    (px_in_u8, width, height)
}

fn write_png(filename: &str, width: u32, height: u32, pixels: &[u8]) {
    use std::io::BufWriter;
    let file = File::create(filename).unwrap();
    let ref mut w = BufWriter::new(file);
    let mut encoder = png::Encoder::new(w, width, height);
    encoder.set_color(png::ColorType::RGB);
    encoder.set_depth(png::BitDepth::Eight);
    let mut writer = encoder.write_header().unwrap();
    writer.write_image_data(pixels).unwrap();
}

pub fn rgbu8_slice_as_u8(slice: &[RGBu8]) -> &[u8] {
    unsafe {
        std::slice::from_raw_parts(slice.as_ptr() as *const u8, slice.len() * 3)
    }
}
