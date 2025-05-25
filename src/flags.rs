use crate::Colour;

pub(crate) enum FlagData {
    Colours(&'static [Colour]),
    Special(&'static [u8], &'static [Colour]),
}

flaggen::generate_flags! {
    Rainbow: (229, 0, 0), (255, 141, 0), (255, 238, 0), (2, 129, 33), (0, 76, 255), (119, 0, 136)
    Transgender: (91, 207, 251), (245, 171, 185), (255, 255, 255), (245, 171, 185), (91, 207, 251)

    Intersex: "intersex.webp" (254, 216, 0), (121, 3, 170)
}
