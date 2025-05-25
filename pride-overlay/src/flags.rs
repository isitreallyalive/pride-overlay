use crate::Colour;

/// The flag's scaling mode
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
pub struct FlagData<'a> {
    #[builder(start_fn)]
    pub colours: &'a [Colour],
    #[builder(with = |data: &'static [u8], scale: ScaleMode| Svg { data, scale })]
    pub svg: Option<Svg>,
}

proc::generate_flags! {
    Agender {
        0x000000, 0xB9B9B9, 0xFFFFFF, 0xB8F483, 0xFFFFFF, 0xB9B9B9, 0x000000
    }

    Aromantic {
        0x3DA542, 0xA7D379, 0xFFFFFF, 0xA9A9A9, 0x000000
    }

    Asexual {
        0x000000, 0xA3A3A3, 0xFFFFFF, 0x800080
    }

    Bisexual {
        0xD60270:2, 0x9B4F96:1, 0x0038A8:2
    }

    Genderfluid {
        0xFF75A2, 0xFFFFFF, 0xBE18D6, 0x2C2C2C, 0x333EBD
    }

    Genderqueer {
        0xB67FDD, 0xFFFFFF, 0x49821E
    }

    Intersex {
        Fill
        0xFFD800, 0x7902AA
    }

    Lesbian {
        0xD52D00, 0xFF9A56, 0xFFFFFF, 0xD362A4, 0xA30262
    }

    Nonbinary {
        0xFFF433, 0xFFFFFF, 0x9B59D0, 0x2D2D2D
    }

    Pansexual {
        0xFF218C, 0xFFD800, 0x21B1FF
    }

    Polyamory {
        Stretch
        0x009FE3, 0xE50051, 0x340C46
    }

    Rainbow {
        0xE50000, 0xFF8D00, 0xFFEE00, 0x028121, 0x004CFF, 0x770088
    }

    Transgender {
        0x5BCEFA, 0xF5A9B8, 0xFFFFFF, 0xF5A9B8, 0x5BCEFA
    }
}
