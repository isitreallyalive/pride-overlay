use super::{Flag, Svg, SvgScaleMode};
use crate::Colour;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

// common colours
const BLACK: Colour = Colour::hex(0x000000);
const WHITE: Colour = Colour::hex(0xFFFFFF);

macro_rules! gen_flags {
    (
         $(
            $(#[doc = $doc:literal])*
            $flag:ident
        ),*$(,)?
    ) => {
        /// Built-in pride flags.
        #[cfg_attr(target_arch = "wasm32", derive(Serialize, Deserialize))]
        #[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
        #[repr(u8)]
        pub enum PresetFlag {
            $(
                #[cfg_attr(
                    not(target_arch = "wasm32"),
                    doc = concat!($($doc, "\n",)*)
                )]
                $flag,
            )*
        }

        impl PresetFlag {
            pub(crate) const fn max_discriminant() -> u8 {
                let mut max = 0;
                $(
                    if PresetFlag::$flag as u8 > max {
                        max = PresetFlag::$flag as u8;
                    }
                )*
                max
            }
        }

        impl std::str::FromStr for PresetFlag {
            type Err = ();

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                match s {
                    $(
                        stringify!($flag) => Ok(PresetFlag::$flag),
                    )*
                    _ => Err(()),
                }
            }
        }

        pastey::paste! {
            impl PresetFlag {
                pub const fn all() -> &'static [Flag<'static>] {
                    &[
                        $(
                            [<$flag:upper>],
                        )*
                    ]
                }
            }

            impl std::ops::Deref for PresetFlag {
                type Target = Flag<'static>;

                fn deref(&self) -> &Self::Target {
                    match self {
                        $(
                            PresetFlag::$flag => &[<$flag:upper>],
                        )*
                    }
                }
            }
        }
    };
}

gen_flags! {
    /// <img src="https://raw.githubusercontent.com/isitreallyalive/pride-overlay/refs/heads/main/flags/agender.svg" width="128" />
    Agender,
    /// <img src="https://raw.githubusercontent.com/isitreallyalive/pride-overlay/refs/heads/main/flags/aromantic.svg" width="128" />
    Aromantic,
    /// <img src="https://raw.githubusercontent.com/isitreallyalive/pride-overlay/refs/heads/main/flags/asexual.svg" width="128" />
    Asexual,
    /// <img src="https://raw.githubusercontent.com/isitreallyalive/pride-overlay/refs/heads/main/flags/bisexual.svg" width="128" />
    Bisexual,
    /// <img src="https://raw.githubusercontent.com/isitreallyalive/pride-overlay/refs/heads/main/flags/genderfluid.svg" width="128" />
    Genderfluid,
    /// <img src="https://raw.githubusercontent.com/isitreallyalive/pride-overlay/refs/heads/main/flags/genderqueer.svg" width="128" />
    Genderqueer,
    /// <img src="https://raw.githubusercontent.com/isitreallyalive/pride-overlay/refs/heads/main/flags/intersex.svg" width="128" />
    Intersex,
    /// <img src="https://raw.githubusercontent.com/isitreallyalive/pride-overlay/refs/heads/main/flags/lesbian.svg" width="128" />
    Lesbian,
    /// <img src="https://raw.githubusercontent.com/isitreallyalive/pride-overlay/refs/heads/main/flags/nonbinary.svg" width="128" />
    Nonbinary,
    /// <img src="https://raw.githubusercontent.com/isitreallyalive/pride-overlay/refs/heads/main/flags/pansexual.svg" width="128" />
    Pansexual,
    /// <img src="https://raw.githubusercontent.com/isitreallyalive/pride-overlay/refs/heads/main/flags/polyamory.svg" width="128" />
    Polyamory,
    /// <img src="https://raw.githubusercontent.com/isitreallyalive/pride-overlay/refs/heads/main/flags/rainbow.svg" width="128" />
    Rainbow,
    /// <img src="https://raw.githubusercontent.com/isitreallyalive/pride-overlay/refs/heads/main/flags/transgender.svg" width="128" />
    Transgender,
}

pub(crate) const AGENDER: Flag = Flag::builder(
    "Agender",
    &[
        BLACK,
        Colour::hex(0xB9B9B9), // light gray
        WHITE,
        Colour::hex(0xB8F483), // light green
        WHITE,
        Colour::hex(0xB9B9B9), // light gray
        BLACK,
    ],
)
.build();

pub(crate) const AROMANTIC: Flag = Flag::builder(
    "Aromantic",
    &[
        Colour::hex(0x3DA542), // green
        Colour::hex(0xA7D379), // lime
        WHITE,
        Colour::hex(0xA9A9A9), // gray
        BLACK,
    ],
)
.build();

pub(crate) const ASEXUAL: Flag = Flag::builder(
    "Asexual",
    &[
        BLACK,
        Colour::hex(0xA3A3A3), // gray
        WHITE,
        Colour::hex(0x800080), // dark purple
    ],
)
.build();

pub(crate) const BISEXUAL: Flag = Flag::builder(
    "Bisexual",
    &[
        Colour::hex(0xD60270), // pink
        Colour::hex(0xD60270), // pink
        Colour::hex(0x9B4F96), // purple
        Colour::hex(0x0038A8), // dark blue
        Colour::hex(0x0038A8), // dark blue
    ],
)
.build();

pub(crate) const GENDERFLUID: Flag = Flag::builder(
    "Genderfluid",
    &[
        Colour::hex(0xFF75A2), // rose
        WHITE,
        Colour::hex(0xBE18D6), // purple
        Colour::hex(0x2C2C2C), // dark gray
        Colour::hex(0x333EBD), // indigo
    ],
)
.build();

pub(crate) const GENDERQUEER: Flag = Flag::builder(
    "Genderqueer",
    &[
        Colour::hex(0xB67FDD), // lavender
        WHITE,
        Colour::hex(0x49821E), // dark green
    ],
)
.build();

pub(crate) const INTERSEX: Flag = Flag::builder(
    "Intersex",
    &[
        Colour::hex(0xFFD800), // gold,
        Colour::hex(0x7902AA), // dark purple
    ],
)
.svg(
    Svg::builder(include_bytes!("svg/intersex.svg"))
        .scale(SvgScaleMode::Cover)
        .build(),
)
.build();

pub(crate) const LESBIAN: Flag = Flag::builder(
    "Lesbian",
    &[
        Colour::hex(0xD52D00), // red
        Colour::hex(0xFF9A56), // light orange
        WHITE,
        Colour::hex(0xD362A4), // pink
        Colour::hex(0xA30262), // plum
    ],
)
.build();

pub(crate) const NONBINARY: Flag = Flag::builder(
    "Nonbinary",
    &[
        Colour::hex(0xFFF433), // yellow
        WHITE,
        Colour::hex(0x9B59D0), // purple
        Colour::hex(0x2D2D2D), // dark gray
    ],
)
.build();

pub(crate) const PANSEXUAL: Flag = Flag::builder(
    "Pansexual",
    &[
        Colour::hex(0xFF218C), // pink
        Colour::hex(0xFFD800), // gold
        Colour::hex(0x21B1FF), // blue
    ],
)
.build();

pub(crate) const POLYAMORY: Flag = Flag::builder(
    "Polyamory",
    &[
        Colour::hex(0x009FE3), // turquoise
        Colour::hex(0xE50051), // red
        Colour::hex(0x340C46), // dark purple
    ],
)
.svg(
    Svg::builder(include_bytes!("svg/polyamory.svg"))
        .scale(SvgScaleMode::Cover)
        .build(),
)
.build();

pub(crate) const RAINBOW: Flag = Flag::builder(
    "Rainbow",
    &[
        Colour::hex(0xE50000), // red
        Colour::hex(0xFF8D00), // orange
        Colour::hex(0xFFEE00), // yellow
        Colour::hex(0x028121), // dark green
        Colour::hex(0x004CFF), // blue
        Colour::hex(0x770088), // dark purple
    ],
)
.build();

pub(crate) const TRANSGENDER: Flag = Flag::builder(
    "Transgender",
    &[
        Colour::hex(0x5BCEFA), // light turquoise
        Colour::hex(0xF5A9B8), // rose
        WHITE,
        Colour::hex(0xF5A9B8), // rose
        Colour::hex(0x5BCEFA), // light turquoise
    ],
)
.build();
