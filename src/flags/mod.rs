use crate::prelude::*;

mod data;
#[doc(inline)]
pub use data::*;

mod svg;
#[doc(inline)]
pub use svg::*;

/// A pride flag.
#[derive(Builder, Clone, Copy)]
#[builder(const)]
pub struct Flag<'a> {
    /// Name of the flag.
    #[builder(start_fn)]
    pub name: &'a str,
    /// Colours that make up the flag.
    #[builder(start_fn)]
    pub colours: &'a [Colour],
    pub(crate) svg: Option<SvgAsset<'a>>,
}

impl<'a> Flag<'a> {
    pub const fn all() -> &'static [Flag<'static>] {
        &[
            AGENDER,
            AROMANTIC,
            ASEXUAL,
            BISEXUAL,
            GENDERFLUID,
            GENDERQUEER,
            INTERSEX,
            LESBIAN,
            NONBINARY,
            PANSEXUAL,
            POLYAMORY,
            RAINBOW,
            TRANSGENDER,
        ]
    }
}
