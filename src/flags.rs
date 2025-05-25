use crate::Colour;

pub enum FlagData {
    Colours(&'static [Colour]),
    Special(&'static [u8], &'static [Colour]),
}

flaggen::generate_flags! {
    Rainbow: (229, 0, 0), (255, 141, 0), (255, 238, 0), (2, 129, 33), (0, 76, 255), (119, 0, 136)
    Transgender: (91, 206, 250), (245, 169, 184), (255, 255, 255), (245, 169, 184), (91, 206, 250)

    Intersex: "intersex.svg" (255, 216, 0), (121, 2, 170)
}
