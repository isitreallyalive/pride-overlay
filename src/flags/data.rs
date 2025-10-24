use crate::{flags::Flag, prelude::*};

// common colours
const BLACK: Colour = Colour::hex(0x000000);
const WHITE: Colour = Colour::hex(0xFFFFFF);

pub(crate) const AGENDER: Flag<'static> = Flag::builder(
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

pub(crate) const AROMANTIC: Flag<'static> = Flag::builder(
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

pub(crate) const ASEXUAL: Flag<'static> = Flag::builder(
    "Asexual",
    &[
        BLACK,
        Colour::hex(0xA3A3A3), // gray
        WHITE,
        Colour::hex(0x800080), // dark purple
    ],
)
.build();

pub(crate) const BISEXUAL: Flag<'static> = Flag::builder(
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

pub(crate) const GENDERFLUID: Flag<'static> = Flag::builder(
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

pub(crate) const GENDERQUEER: Flag<'static> = Flag::builder(
    "Genderqueer",
    &[
        Colour::hex(0xB67FDD), // lavender
        WHITE,
        Colour::hex(0x49821E), // dark green
    ],
)
.build();

pub(crate) const INTERSEX: Flag<'static> = Flag::builder(
    "Intersex",
    &[
        Colour::hex(0xFFD800), // gold,
        Colour::hex(0x7902AA), // dark purple
    ],
)
.svg(SvgAsset::new(
    include_bytes!("svg/intersex.svg"),
    SvgScaleMode::Cover,
))
.build();

pub(crate) const LESBIAN: Flag<'static> = Flag::builder(
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

pub(crate) const NONBINARY: Flag<'static> = Flag::builder(
    "Nonbinary",
    &[
        Colour::hex(0xFFF433), // yellow
        WHITE,
        Colour::hex(0x9B59D0), // purple
        Colour::hex(0x2D2D2D), // dark gray
    ],
)
.build();

pub(crate) const PANSEXUAL: Flag<'static> = Flag::builder(
    "Pansexual",
    &[
        Colour::hex(0xFF218C), // pink
        Colour::hex(0xFFD800), // gold
        Colour::hex(0x21B1FF), // blue
    ],
)
.build();

pub(crate) const POLYAMORY: Flag<'static> = Flag::builder(
    "Polyamory",
    &[
        Colour::hex(0x009FE3), // turquoise
        Colour::hex(0xE50051), // red
        Colour::hex(0x340C46), // dark purple
    ],
)
.svg(SvgAsset::new(
    include_bytes!("svg/polyamory.svg"),
    SvgScaleMode::Stretch,
))
.build();

pub(crate) const RAINBOW: Flag<'static> = Flag::builder(
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

pub(crate) const TRANSGENDER: Flag<'static> = Flag::builder(
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
