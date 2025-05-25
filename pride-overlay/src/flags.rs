use crate::Colour;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Svg {
    pub data: &'static [u8],
    pub scale_mode: ScaleMode,
}

pub enum FlagData {
    /// Represents a flag with SVG data and its associated colours.
    Svg(Svg, &'static [Colour]),
    /// Represents a flag with only colour stripes.
    Colours(&'static [Colour]),
}

/// The flag's scaling mode
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ScaleMode {
    /// Maintain aspect ratio, may crop parts of the flag
    Fill,
    /// Stretch to exact dimensions, may distort
    #[default]
    Stretch,
}

proc::generate_flags! {
    Agender: (0, 0, 0), (185, 185, 185), (255, 255, 255), (184, 244, 131), (255, 255, 255), (185, 185, 185), (0, 0, 0)
    Aromantic: (61, 165, 66), (167, 211, 121), (255, 255, 255), (169, 169, 169), (0, 0, 0)
    Asexual: (0, 0, 0), (163, 163, 163), (255, 255, 255), (128, 0, 128)
    Bisexual: (214, 2 , 112 :2), (155, 79, 150 :1), (0, 56, 168 :2)
    Genderfluid: (255, 117, 162), (255, 255, 255), (190, 24, 214), (44, 44, 44), (51, 62, 189)
    Genderqueer: (182, 127, 221), (255, 255, 255), (73, 130, 30)
    Intersex: Fill "intersex.svg" (255, 216, 0), (121, 2, 170)
    Lesbian: (213, 45, 0),  (255, 154, 86), (255, 255, 255), (211, 98, 164), (163, 2, 98)
    Nonbinary: (255, 244, 51), (255, 255, 255), (155, 89, 208), (45, 45, 45)
    Pansexual: (255, 33, 140), (255, 216, 0), (33, 177, 255)
    Polyamory: Stretch "polyamory.svg" (0, 159, 227), (229, 0, 81), (52, 12, 70)
    Rainbow: (229, 0, 0), (255, 141, 0), (255, 238, 0), (2, 129, 33), (0, 76, 255), (119, 0, 136)
    Transgender: (91, 206, 250), (245, 169, 184), (255, 255, 255), (245, 169, 184), (91, 206, 250)
}
