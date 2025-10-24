use crate::Colour;

/// How to scale the SVG when rendering.
#[derive(Clone, Copy, Default)]
pub enum ScaleMode {
    /// Maintain aspect ratio, may crop parts of the flag
    Fill,
    /// Stretch to exact dimensions, may distort
    #[default]
    Stretch,
}

/// Data used to render an SVG.
#[derive(Clone, Copy)]
pub struct Svg {
    pub data: &'static [u8],
    pub scale: ScaleMode,
}

/// Data used to render a pride flag.
///
/// If `svg` is [Some], it will be used during [Overlay].
#[derive(Builder, Clone, Copy)]
#[builder(const)]
pub struct Flag<'a> {
    #[builder(start_fn)]
    pub colours: &'a [Colour],
    #[builder(with = |data: &'static [u8], scale: ScaleMode| Svg { data, scale })]
    pub svg: Option<Svg>,
}

const AGENDER: Flag<'static> = Flag::builder(&[
    Colour::from_hex(0x000000).build(),
    Colour::from_hex(0xB9B9B9).build(),
    Colour::from_hex(0xFFFFFF).build(),
    Colour::from_hex(0xB8F483).build(),
    Colour::from_hex(0xFFFFFF).build(),
    Colour::from_hex(0xB9B9B9).build(),
    Colour::from_hex(0x000000).build(),
])
.build();

const AROMANTIC: Flag<'static> = Flag::builder(&[
    Colour::from_hex(0x3DA542).build(),
    Colour::from_hex(0xA7D379).build(),
    Colour::from_hex(0xFFFFFF).build(),
    Colour::from_hex(0xA9A9A9).build(),
    Colour::from_hex(0x000000).build(),
])
.build();

const ASEXUAL: Flag<'static> = Flag::builder(&[
    Colour::from_hex(0x000000).build(),
    Colour::from_hex(0xA3A3A3).build(),
    Colour::from_hex(0xFFFFFF).build(),
    Colour::from_hex(0x800080).build(),
])
.build();

const BISEXUAL: Flag<'static> = Flag::builder(&[
    Colour::from_hex(0xD60270).proportion(2).build(),
    Colour::from_hex(0x9B4F96).build(),
    Colour::from_hex(0x0038A8).proportion(2).build(),
])
.build();

const GENDERFLUID: Flag<'static> = Flag::builder(&[
    Colour::from_hex(0xFF75A2).build(),
    Colour::from_hex(0xFFFFFF).build(),
    Colour::from_hex(0xBE18D6).build(),
    Colour::from_hex(0x2C2C2C).build(),
    Colour::from_hex(0x333EBD).build(),
])
.build();

const GENDERQUEER: Flag<'static> = Flag::builder(&[
    Colour::from_hex(0xB67FDD).build(),
    Colour::from_hex(0xFFFFFF).build(),
    Colour::from_hex(0x49821E).build(),
])
.build();

const INTERSEX: Flag<'static> = Flag::builder(&[
    Colour::from_hex(0xFFD800).build(),
    Colour::from_hex(0x7902AA).build(),
])
.maybe_svg(Some((
    include_bytes!("../flags/intersex.svg"),
    ScaleMode::Fill,
)))
.build();

const LESBIAN: Flag<'static> = Flag::builder(&[
    Colour::from_hex(0xD52D00).build(),
    Colour::from_hex(0xFF9A56).build(),
    Colour::from_hex(0xFFFFFF).build(),
    Colour::from_hex(0xD362A4).build(),
    Colour::from_hex(0xA30262).build(),
])
.build();

const NONBINARY: Flag<'static> = Flag::builder(&[
    Colour::from_hex(0xFFF433).build(),
    Colour::from_hex(0xFFFFFF).build(),
    Colour::from_hex(0x9B59D0).build(),
    Colour::from_hex(0x2D2D2D).build(),
])
.build();

const PANSEXUAL: Flag<'static> = Flag::builder(&[
    Colour::from_hex(0xFF218C).build(),
    Colour::from_hex(0xFFD800).build(),
    Colour::from_hex(0x21B1FF).build(),
])
.build();

const POLYAMORY: Flag<'static> = Flag::builder(&[
    Colour::from_hex(0x009FE3).build(),
    Colour::from_hex(0xE50051).build(),
    Colour::from_hex(0x340C46).build(),
])
.maybe_svg(Some((
    include_bytes!("../flags/polyamory.svg"),
    ScaleMode::Stretch,
)))
.build();

const RAINBOW: Flag<'static> = Flag::builder(&[
    Colour::from_hex(0xE50000).build(),
    Colour::from_hex(0xFF8D00).build(),
    Colour::from_hex(0xFFEE00).build(),
    Colour::from_hex(0x028121).build(),
    Colour::from_hex(0x004CFF).build(),
    Colour::from_hex(0x770088).build(),
])
.build();

const TRANSGENDER: Flag<'static> = Flag::builder(&[
    Colour::from_hex(0x5BCEFA).build(),
    Colour::from_hex(0xF5A9B8).build(),
    Colour::from_hex(0xFFFFFF).build(),
    Colour::from_hex(0xF5A9B8).build(),
    Colour::from_hex(0x5BCEFA).build(),
])
.build();

/// A pride flag.
#[derive(Clone, Copy)]
pub enum PrideFlag {
    Agender,
    Aromantic,
    Asexual,
    Bisexual,
    Genderfluid,
    Genderqueer,
    Intersex,
    Lesbian,
    Nonbinary,
    Pansexual,
    Polyamory,
    Rainbow,
    Transgender,
}

impl PrideFlag {
    /// Enumerates all built-in pride flags.
    pub const fn all() -> &'static [PrideFlag] {
        &[
            PrideFlag::Agender,
            PrideFlag::Aromantic,
            PrideFlag::Asexual,
            PrideFlag::Bisexual,
            PrideFlag::Genderfluid,
            PrideFlag::Genderqueer,
            PrideFlag::Intersex,
            PrideFlag::Lesbian,
            PrideFlag::Nonbinary,
            PrideFlag::Pansexual,
            PrideFlag::Polyamory,
            PrideFlag::Rainbow,
            PrideFlag::Transgender,
        ]
    }

    /// Gets the human-readable name of a pride flag.
    pub const fn name(&self) -> &'static str {
        match self {
            PrideFlag::Agender => "Agender",
            PrideFlag::Aromantic => "Aromantic",
            PrideFlag::Asexual => "Asexual",
            PrideFlag::Bisexual => "Bisexual",
            PrideFlag::Genderfluid => "Genderfluid",
            PrideFlag::Genderqueer => "Genderqueer",
            PrideFlag::Intersex => "Intersex",
            PrideFlag::Lesbian => "Lesbian",
            PrideFlag::Nonbinary => "Nonbinary",
            PrideFlag::Pansexual => "Pansexual",
            PrideFlag::Polyamory => "Polyamory",
            PrideFlag::Rainbow => "Rainbow",
            PrideFlag::Transgender => "Transgender",
        }
    }

    /// Internal flag data.
    pub(crate) const fn data(&self) -> Flag<'static> {
        match self {
            PrideFlag::Agender => AGENDER,
            PrideFlag::Aromantic => AROMANTIC,
            PrideFlag::Asexual => ASEXUAL,
            PrideFlag::Bisexual => BISEXUAL,
            PrideFlag::Genderfluid => GENDERFLUID,
            PrideFlag::Genderqueer => GENDERQUEER,
            PrideFlag::Intersex => INTERSEX,
            PrideFlag::Lesbian => LESBIAN,
            PrideFlag::Nonbinary => NONBINARY,
            PrideFlag::Pansexual => PANSEXUAL,
            PrideFlag::Polyamory => POLYAMORY,
            PrideFlag::Rainbow => RAINBOW,
            PrideFlag::Transgender => TRANSGENDER,
        }
    }
}
