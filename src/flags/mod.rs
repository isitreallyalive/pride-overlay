#[cfg(target_arch = "wasm32")]
use std::borrow::Cow;

use crate::prelude::*;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

mod data;
use data::*;

mod svg;
#[doc(inline)]
pub use svg::*;

/// A pride flag.
#[derive(bon::Builder, Clone, Copy)]
#[builder(const)]
pub struct Flag<'a> {
    /// Name of the flag.
    #[builder(start_fn)]
    pub name: &'a str,
    /// Colours that make up the flag.
    #[builder(start_fn)]
    pub(crate) colours: &'a [Colour],
    pub(crate) svg: Option<SvgAsset<'a>>,
}

#[cfg(not(target_arch = "wasm32"))]
pub(crate) type FlagOwned<'a> = Flag<'a>;

#[cfg(target_arch = "wasm32")]
pub struct FlagOwned<'a> {
    pub(crate) colours: Cow<'a, [Colour]>,
    pub(crate) svg: Option<SvgAssetOwned<'a>>,
}

impl<'a> From<&Flag<'a>> for Flag<'a> {
    fn from(flag: &Flag<'a>) -> Self {
        *flag
    }
}


#[cfg(target_arch = "wasm32")]
impl<'a> From<Flag<'a>> for FlagOwned<'a> {
    fn from(flag: Flag<'a>) -> Self {
        Self {
            colours: Cow::Owned(flag.colours.to_vec()),
            svg: flag.svg.map(|svg| svg.into()),
        }
    }
}

#[cfg(target_arch = "wasm32")]
impl<'a> From<&'a Flag<'a>> for FlagOwned<'a> {
    fn from(flag: &'a Flag<'a>) -> Self {
        Self {
            colours: Cow::Borrowed(flag.colours),
            svg: flag.svg.as_ref().map(|svg| SvgAssetOwned {
                data: Cow::Borrowed(svg.data),
                scale: svg.scale,
            }),
        }
    }
}

macro_rules! gen_flags {
    (
         $(
            $(#[doc = $doc:literal])*
            $flag:ident
        ),*$(,)?
    ) => {
        /// Built-in pride flags.
        #[derive(Clone, Copy)]
        #[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
        pub enum Flags {
            $(
                #[cfg_attr(
                    not(target_arch = "wasm32"),
                    doc = concat!($($doc, "\n",)*)
                )]
                $flag,
            )*
        }

        pastey::paste! {
            impl Flags {
                pub const fn all() -> &'static [Flag<'static>] {
                    &[
                        $(
                            [<$flag:upper>],
                        )*
                    ]
                }
            }

            impl<'a> From<Flags> for FlagOwned<'a> {
                fn from(flag: Flags) -> Self {
                    match flag {
                        $(
                            Flags::$flag => [<$flag:upper>].into(),
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
