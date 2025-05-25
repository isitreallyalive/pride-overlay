use crate::Colour;

pub enum FlagData {
    Colours(&'static [Colour]),
    Svg(&'static [u8], &'static [Colour]),
}

proc::generate_flags! {
    Bisexual: (214, 2 , 112 :2), (155, 79, 150 :1), (0, 56, 168 :2)
    Intersex: "intersex.svg" (255, 216, 0), (121, 2, 170)
    Pansexual: (255, 33, 140), (255, 216, 0), (33, 177, 255)
    Rainbow: (229, 0, 0), (255, 141, 0), (255, 238, 0), (2, 129, 33), (0, 76, 255), (119, 0, 136)
    Transgender: (91, 206, 250), (245, 169, 184), (255, 255, 255), (245, 169, 184), (91, 206, 250)
}
