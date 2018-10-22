# colorspace

 A crate for colorimetry in Rust.
 This crate contains types and functions for working with color. The intended
 use is to support rendering applications (I use it to manage color in a spectral pathtracer), but if you want to be able to 
 convert between spectral, XYZ, L'a'b' and RGB spaces of various flavors such as
 sRGB, ACES, DCI P3 and ALEXA Wide Gamut then this is the crate for you.
 
 Note that currently the results are not as accurate as they could be, due
 to the spectral->XYZ conversion not being implemented according to spec,
 but the results should be "good enough" for casual visual inspection. Be
 aware that future versions of the library will change some decimal places
 as the accuracy is improved.
 
 ## Examples
 ### Spectral to 8-bit, gamma encoded sRGB conversion
 ```rust
 // Definition of the sRGB color space
 use colorspace::color_space_rgb::sRGB;
 // The prelude brings in common types
 use colorspace::prelude::*;
 // Convert the spectral data for a measured MacBeth chart swatch to XYZ
 // using the CIE 1931 2-degree CMFs and a D65 illuminant
 let xyz = babel_average::spd["dark_skin"]
     .to_xyz_with_illuminant(&illuminant::D65.spd);
 // Convert the XYZ value to scene-referred (i.e. linear) sRGB
 let xf_xyz_to_srgb = xyz_to_rgb_matrix(sRGB.white, &sRGB);
 let rgb = xyz_to_rgb(&xf_xyz_to_srgb, xyz);
 // Convert the scene-referred sRGB value to an 8-bit, display-referred
 // value by applying the opto-electrical transfer function and using RGBu8's
 // From<RGBf32> impl
 let rgb: RGBu8 = (sRGB.oetf)(rgb).into();
 assert_eq!(rgb, rgbu8(115, 82, 68));
 ```
 
 ## Licence
 colorspace is licensed under Apache License, Version 2.0
 http://www.apache.org/licenses/LICENSE-2.0
 
 This crate contains some data taken from the excellent colour-science python
 library by Mansencal et al.: <https://www.colour-science.org>
 Copyright (c) 2013-2018, Colour Developers
 
 Most of the conversion algorithms are based on those published at 
 Bruce Lindbloom's site: <http://www.brucelindbloom.com>
 Copyright © 2001 - 2018 Bruce Justin Lindbloom.
 
 BabelColor color-checker data is copyright © 2004‐2012 Danny Pascale (www.babelcolor.com); used with permission.
 <http://www.babelcolor.com/index_htm_files/ColorChecker_RGB_and_spectra.xls>
 <http://www.babelcolor.com/index_htm_files/ColorChecker_RGB_and_spectra.zip>

