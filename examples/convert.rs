use colorspace::*;
use std::fs::File;

fn main() {
    // Read marcie into a Vec of u8 representing packed RGB triples.
    let decoder =
        png::Decoder::new(File::open("resources/marcie_sRGB.png").unwrap());
    let (info, mut reader) = decoder.read_info().unwrap();
    let (width, height) = reader.info().size();
    let mut px_in_u8 = vec![0; info.buffer_size()];
    reader.next_frame(&mut px_in_u8).unwrap();

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

    // Next we'll need a matrix to go from sRGB to XYZ
    let srgb = &color_space_rgb::model_f32::SRGB;
    let xf_srgb_to_xyz = rgb_to_xyz_matrix(srgb.white, srgb);

    // Now convert. We need to first decode the sRGB pixels to linear instead
    // of display-referred values, then apply the matrix
    let px_xyz: Vec<_> = px_srgb
        .iter()
        .map(|p| {
            let p_linear = srgb.decode(*p);
            rgb_to_xyz(&xf_srgb_to_xyz, p_linear)
        })
        .collect();

    // Now we'll convert to both Adobe RGB and Alexa LogC. This is the reverse
    // process. Again, we'll first need matrices to go from XYZ to the desired
    // space.
    // In the case of P3, this involves a Chromatic Adaptation Transform as well
    // since sRGB and P3 have different white points. This is handled using CAT02
    // by the xyz_to_rgb_matrix() function.
    let adobe_rgb = &color_space_rgb::model_f32::ADOBE_RGB_1998;
    let alexa_wide = &color_space_rgb::model_f32::ALEXA_WIDE_GAMUT;
    let dci_p3 = &color_space_rgb::model_f32::DCI_P3;

    let xf_xyz_to_adobe_rgb = xyz_to_rgb_matrix(adobe_rgb.white, adobe_rgb);
    let xf_xyz_to_alexa_wide = xyz_to_rgb_matrix(alexa_wide.white, alexa_wide);
    let xf_xyz_to_dci_p3 = xyz_to_rgb_matrix(dci_p3.white, dci_p3);

    // We then apply the matrix to each pixel before encoding it. In addition
    // we'll convert to RGBu8 for output
    let px_adobe_rgb_u8: Vec<RGBu8> = px_xyz
        .iter()
        .map(|p| {
            let p_adobe_rgb = xyz_to_rgb(&xf_xyz_to_adobe_rgb, *p);
            let p_adobe_rgb = adobe_rgb.encode(p_adobe_rgb);
            // convert to RGBu8. This automatically scales to [0..255] and clamps
            p_adobe_rgb.into()
        })
        .collect();

    let px_alexa_wide_u8: Vec<RGBu8> = px_xyz
        .iter()
        .map(|p| {
            let p_alexa_wide = xyz_to_rgb(&xf_xyz_to_alexa_wide, *p);
            let p_alexa_wide = alexa_wide.encode(p_alexa_wide);
            p_alexa_wide.into()
        })
        .collect();

    let px_dci_p3_u8: Vec<RGBu8> = px_xyz
        .iter()
        .map(|p| {
            let p_dci_p3 = xyz_to_rgb(&xf_xyz_to_dci_p3, *p);
            let p_dci_p3 = dci_p3.encode(p_dci_p3);
            p_dci_p3.into()
        })
        .collect();

    // write the images
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
