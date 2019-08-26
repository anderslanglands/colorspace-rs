import colour

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

cmfs = colour.CMFS[u'cie_2_1931']
illum = colour.ILLUMINANTS_SDS[u'D65']
babelcolor_sds = colour.characterisation.COLOURCHECKERS_SDS[u'BabelColor Average']
dark_skin_sd = babelcolor_sds[u'dark skin']
xyz = colour.sd_to_XYZ(dark_skin_sd, cmfs, illum)
print(xyz/ 100)
rgb = colour.XYZ_to_sRGB(xyz / 100)
print(rgb)

print('let ref_xyz = HashMap::new();')
print('let ref_srgb = HashMap::new();')
for name in name_map.keys():
    sd = babelcolor_sds[name]
    xyz = colour.sd_to_XYZ(sd, cmfs, illum) / 100
    srgb = colour.XYZ_to_sRGB(xyz)
    print('ref_xyz.insert("%s", xyz(%f, %f, %f));' % (name_map[name], xyz[0], xyz[1], xyz[2]))
    print('ref_srgb.insert("%s", rgbf32(%f, %f, %f));' % (name_map[name], srgb[0], srgb[1], srgb[2]))