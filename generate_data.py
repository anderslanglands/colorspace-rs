import colour
from colour import *
import numpy as np

from colour.utilities import (dot_matrix, dot_vector, from_range_1,
                              row_as_diagonal, to_domain_1)
name_map = {
    u'black 2 (1.5 D)': 'black_20',
    u'blue': 'blue',
    u'blue flower': 'blue_flower',
    u'blue sky': 'blue_sky',
    u'bluish green': 'bluish_green',
    u'cyan': 'cyan',
    u'dark skin': 'dark_skin',
    u'foliage': 'foliage',
    u'green': 'green',
    u'light skin': 'light_skin',
    u'magenta': 'magenta',
    u'moderate red': 'moderate_red',
    u'neutral 3.5 (1.05 D)': 'neutral_35',
    u'neutral 5 (.70 D)': 'neutral_50',
    u'neutral 6.5 (.44 D)': 'neutral_65',
    u'neutral 8 (.23 D)': 'neutral_80',
    u'orange': 'orange',
    u'orange yellow': 'orange_yellow',
    u'purple': 'purple',
    u'purplish blue': 'purplish_blue',
    u'red': 'red',
    u'white 9.5 (.05 D)': 'white_95',
    u'yellow': 'yellow',
    u'yellow green': 'yellow_green'
}


def generate_colorchecker_xyz(name, t, checker, cmfs, ill):
    print('pub static ref %s: HashMap<String, %s> = hashmap! {' % (name, t))
    for swatch_name in checker.keys():
        sd = checker[swatch_name]
        xyz = sd_to_XYZ(sd, cmfs, ill)
        print('    "%s".into() => xyz(%.24f, %.24f, %.24f),' %
              (name_map[swatch_name], xyz[0], xyz[1], xyz[2]))
    print('};')


# Align cmfs to standard practise shape
cmfs = CMFS['CIE 1931 2 Degree Standard Observer'].clone().align(
    SpectralShape(360, 780, 1))
ill_d65 = colour.ILLUMINANTS_SDS['D65'].clone().align(
    SpectralShape(360, 780, 1))

generate_colorchecker_xyz('XYZ_D65', 'XYZf64',
                          colour.COLOURCHECKERS_SDS['BabelColor Average'],
                          cmfs, ill_d65)

# Generate spectral checkers
print('pub static ref VSPD: HashMap<String, VSPD> = hashmap! {')
for swatch_name in colour.COLOURCHECKERS_SDS['BabelColor Average'].keys():
    sd = colour.COLOURCHECKERS_SDS['BabelColor Average'][swatch_name]
    print('    "%s".into() => vspd! {' % name_map[swatch_name])

    for w, v in zip(sd.wavelengths, sd.values):
        print('        %f => %f,' % (w, v))
    print('    },')

print('};')


def generate_colorchecker_rgb(name, model, t, checker, cmfs, ill):
    print(
        'pub static ref %s_SCENE_REFERRED: HashMap<String, %s> = hashmap! {' %
        (name, t))
    xyz_wp = colour.models.rgb.RGB_COLOURSPACES['sRGB'].whitepoint
    for swatch_name in checker.keys():
        sd = checker[swatch_name]
        xyz = sd_to_XYZ(sd, cmfs, ill)
        rgb = colour.XYZ_to_RGB(xyz / 100.0, xyz_wp,
                                model.whitepoint, model.XYZ_to_RGB_matrix)
        print('    "%s".into() => rgbf(%.24f, %.24f, %.24f),' %
              (name_map[swatch_name], rgb[0], rgb[1], rgb[2]))
    print('};\n')

    print('pub static ref %s_ENCODED: HashMap<String, %s> = hashmap! {' %
          (name, t))
    for swatch_name in checker.keys():
        sd = checker[swatch_name]
        xyz = sd_to_XYZ(sd, cmfs, ill)
        rgb = colour.XYZ_to_RGB(xyz / 100.0, xyz_wp,
                                model.whitepoint, model.XYZ_to_RGB_matrix)
        rgb = model.encoding_cctf(rgb)
        print('    "%s".into() => rgbf(%.24f, %.24f, %.24f),' %
              (name_map[swatch_name], rgb[0], rgb[1], rgb[2]))
    print('};\n')

generate_colorchecker_rgb('SRGB', colour.models.rgb.RGB_COLOURSPACES['sRGB'],
                          'RGBf64',
                          colour.COLOURCHECKERS_SDS['BabelColor Average'],
                          cmfs, ill_d65)

generate_colorchecker_rgb('ITUR_BT709', colour.models.rgb.RGB_COLOURSPACES['ITU-R BT.709'],
                          'RGBf64',
                          colour.COLOURCHECKERS_SDS['BabelColor Average'],
                          cmfs, ill_d65)

generate_colorchecker_rgb('ALEXA_WIDE_GAMUT', colour.models.rgb.RGB_COLOURSPACES['ALEXA Wide Gamut'],
                          'RGBf64',
                          colour.COLOURCHECKERS_SDS['BabelColor Average'],
                          cmfs, ill_d65)

generate_colorchecker_rgb('ACES_CC', colour.models.rgb.RGB_COLOURSPACES['ACEScc'],
                          'RGBf64',
                          colour.COLOURCHECKERS_SDS['BabelColor Average'],
                          cmfs, ill_d65)

generate_colorchecker_rgb('ACES_CCT', colour.models.rgb.RGB_COLOURSPACES['ACEScct'],
                          'RGBf64',
                          colour.COLOURCHECKERS_SDS['BabelColor Average'],
                          cmfs, ill_d65)

generate_colorchecker_rgb('ACES_CG', colour.models.rgb.RGB_COLOURSPACES['ACEScg'],
                          'RGBf64',
                          colour.COLOURCHECKERS_SDS['BabelColor Average'],
                          cmfs, ill_d65)

generate_colorchecker_rgb('ACES', colour.models.rgb.RGB_COLOURSPACES['aces'],
                          'RGBf64',
                          colour.COLOURCHECKERS_SDS['BabelColor Average'],
                          cmfs, ill_d65)

generate_colorchecker_rgb('ACES_PROXY', colour.models.rgb.RGB_COLOURSPACES['ACESproxy'],
                          'RGBf64',
                          colour.COLOURCHECKERS_SDS['BabelColor Average'],
                          cmfs, ill_d65)

generate_colorchecker_rgb('DCI_P3_P', colour.models.rgb.RGB_COLOURSPACES['DCI-P3+'],
                          'RGBf64',
                          colour.COLOURCHECKERS_SDS['BabelColor Average'],
                          cmfs, ill_d65)

generate_colorchecker_rgb('DCI_P3', colour.models.rgb.RGB_COLOURSPACES['DCI-P3'],
                          'RGBf64',
                          colour.COLOURCHECKERS_SDS['BabelColor Average'],
                          cmfs, ill_d65)

generate_colorchecker_rgb('PRO_PHOTO_RGB', colour.models.rgb.RGB_COLOURSPACES['ProPhoto RGB'],
                          'RGBf64',
                          colour.COLOURCHECKERS_SDS['BabelColor Average'],
                          cmfs, ill_d65)

generate_colorchecker_rgb('BETA_RGB', colour.models.rgb.RGB_COLOURSPACES['Beta RGB'],
                          'RGBf64',
                          colour.COLOURCHECKERS_SDS['BabelColor Average'],
                          cmfs, ill_d65)

generate_colorchecker_rgb('RED_COLOR', colour.models.rgb.RGB_COLOURSPACES['REDcolor'],
                          'RGBf64',
                          colour.COLOURCHECKERS_SDS['BabelColor Average'],
                          cmfs, ill_d65)

generate_colorchecker_rgb('RED_COLOR2', colour.models.rgb.RGB_COLOURSPACES['REDcolor2'],
                          'RGBf64',
                          colour.COLOURCHECKERS_SDS['BabelColor Average'],
                          cmfs, ill_d65)

generate_colorchecker_rgb('RED_COLOR3', colour.models.rgb.RGB_COLOURSPACES['REDcolor3'],
                          'RGBf64',
                          colour.COLOURCHECKERS_SDS['BabelColor Average'],
                          cmfs, ill_d65)

generate_colorchecker_rgb('RED_COLOR4', colour.models.rgb.RGB_COLOURSPACES['REDcolor4'],
                          'RGBf64',
                          colour.COLOURCHECKERS_SDS['BabelColor Average'],
                          cmfs, ill_d65)

generate_colorchecker_rgb('DRAGON_COLOR', colour.models.rgb.RGB_COLOURSPACES['DRAGONcolor'],
                          'RGBf64',
                          colour.COLOURCHECKERS_SDS['BabelColor Average'],
                          cmfs, ill_d65)

generate_colorchecker_rgb('DRAGON_COLOR2', colour.models.rgb.RGB_COLOURSPACES['DRAGONcolor2'],
                          'RGBf64',
                          colour.COLOURCHECKERS_SDS['BabelColor Average'],
                          cmfs, ill_d65)

generate_colorchecker_rgb('SHARP_RGB', colour.models.rgb.RGB_COLOURSPACES['Sharp RGB'],
                          'RGBf64',
                          colour.COLOURCHECKERS_SDS['BabelColor Average'],
                          cmfs, ill_d65)

generate_colorchecker_rgb('ITUR_BT2020', colour.models.rgb.RGB_COLOURSPACES['ITU-R BT.2020'],
                          'RGBf64',
                          colour.COLOURCHECKERS_SDS['BabelColor Average'],
                          cmfs, ill_d65)

generate_colorchecker_rgb('ADOBE_RGB_1998', colour.models.rgb.RGB_COLOURSPACES['Adobe RGB (1998)'],
                          'RGBf64',
                          colour.COLOURCHECKERS_SDS['BabelColor Average'],
                          cmfs, ill_d65)

print(colour.models.rgb.RGB_COLOURSPACES.keys())

wp_srgb = colour.models.rgb.RGB_COLOURSPACES['sRGB'].whitepoint
wp_srgb_xyz = xyY_to_XYZ(xy_to_xyY(wp_srgb))
wp_aces = colour.models.rgb.RGB_COLOURSPACES['aces'].whitepoint
wp_aces_xyz = xyY_to_XYZ(xy_to_xyY(wp_aces))
print("srgb whitepoint ", wp_srgb, wp_srgb_xyz)
print("aces whitepoint ", wp_aces, wp_aces_xyz)
print(colour.models.rgb.RGB_COLOURSPACES['aces'].XYZ_to_RGB_matrix)

cat_mtx = colour.adaptation.chromatic_adaptation_matrix_VonKries(wp_srgb_xyz, wp_aces_xyz, transform='CAT02')
print(cat_mtx)

cat_mtx = colour.adaptation.chromatic_adaptation_matrix_VonKries(wp_srgb_xyz, wp_aces_xyz, transform='Bradford')
print(cat_mtx)

# Generate data for SPD type
spd_shape = SpectralShape(380, 770, 10)
W = colour.colorimetry.tristimulus.tristimulus_weighting_factors_ASTME202211(
    cmfs,
    ill_d65,
    spd_shape,
)

W = colour.colorimetry.tristimulus.adjust_tristimulus_weighting_factors_ASTME30815(W, SpectralShape(cmfs.shape.start, cmfs.shape.end, spd_shape.interval), spd_shape)
print('pub static ref W_X: SPD = SPD::new([')
for w in W[:,0]:
    print('    %.24f,' % w)
print(']);')

print('pub static ref W_Y: SPD = SPD::new([')
for w in W[:,1]:
    print('    %.24f,' % w)
print(']);')

print('pub static ref W_Z: SPD = SPD::new([')
for w in W[:,2]:
    print('    %.24f,' % w)
print(']);')

print('pub static ref BABELCOLOR: HashMap<String, SPD> = hashmap! {')
for swatch_name in colour.COLOURCHECKERS_SDS['BabelColor Average'].keys():
    sd = colour.COLOURCHECKERS_SDS['BabelColor Average'][swatch_name].clone().align(spd_shape)
    print('    "%s".into() => SPD::new([' % name_map[swatch_name])

    for v in sd.values:
        print('        %.24f,' % v )
    print('    ]),')

print('};')

def chromatic_adaptation_matrix_VonKries(XYZ_w, XYZ_wr, transform='CAT02'):
    M = CHROMATIC_ADAPTATION_TRANSFORMS.get(transform)

    print('XYZ_w', XYZ_w)
    print('XYZ_wr', XYZ_wr)

    if M is None:
        raise KeyError(
            '"{0}" chromatic adaptation transform is not defined! Supported '
            'methods: "{1}".'.format(transform,
                                     CHROMATIC_ADAPTATION_TRANSFORMS.keys()))

    print('M', M)
    print('M inv', np.linalg.inv(M))

    rgb_w = np.einsum('...i,...ij->...j', XYZ_w, np.transpose(M))
    rgb_wr = np.einsum('...i,...ij->...j', XYZ_wr, np.transpose(M))

    print('rgb_w: ', rgb_w)
    print('rgb_wr: ', rgb_wr)

    D = rgb_wr / rgb_w
    print('D: ', D)

    D = row_as_diagonal(D)

    M_CAT = dot_matrix(np.linalg.inv(M), D)
    M_CAT = dot_matrix(M_CAT, M)

    return M_CAT

M_cat02 = chromatic_adaptation_matrix_VonKries(wp_srgb_xyz, wp_aces_xyz)
print(M_cat02)


model_src = colour.models.rgb.RGB_COLOURSPACES['sRGB']
model_dst =colour.models.rgb.RGB_COLOURSPACES['aces']

print(
    'pub static ref ACES_FROM_SRGB: HashMap<String, RGBf64> = hashmap! {')
xyz_wp = colour.models.rgb.RGB_COLOURSPACES['sRGB'].whitepoint
for swatch_name in colour.COLOURCHECKERS_SDS['BabelColor Average'].keys():
    sd = colour.COLOURCHECKERS_SDS['BabelColor Average'][swatch_name]
    xyz = sd_to_XYZ(sd, cmfs, ill_d65)
    rgb_srgb = colour.XYZ_to_RGB(xyz / 100.0, xyz_wp,
                            model_src.whitepoint, model_src.XYZ_to_RGB_matrix)

    rgb_aces = colour.RGB_to_RGB(rgb_srgb, model_src, model_dst)
    print('    "%s".into() => rgbf(%.24f, %.24f, %.24f),' %
            (name_map[swatch_name], rgb_aces[0], rgb_aces[1], rgb_aces[2]))
print('};\n')

# Whitepoints
d65_xyz = xy_to_XYZ([0.31270, 0.32900])
print('d65_xyz: ', d65_xyz)
