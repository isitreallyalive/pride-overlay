//! # Overview
//!
//! This crate provides functionality to overlay pride flags onto images.
//! You can either [Overlay](effects::Overlay) a flag directly onto an image with adjustable opacity,
//! or draw a [Ring](effects::Ring) around the image using the flag's colours,
//! or create your own custom effects using the provided [Flag](flags::Flag) type and [Effect](effects::Effect) trait.
//!
//! | Input | Overlay | Ring |
//! |:-----:|:-------:|:----:|
//! | ![](https://raw.githubusercontent.com/isitreallyalive/pride-overlay/refs/heads/main/examples/input.webp) | ![](https://raw.githubusercontent.com/isitreallyalive/pride-overlay/refs/heads/main/examples/out/overlay/intersex.webp) | ![](https://raw.githubusercontent.com/isitreallyalive/pride-overlay/refs/heads/main/examples/out/ring/transgender.webp) |
//!
//! # High level API
//! Load an image with the [`image`](https://docs.rs/image) crate, and [Overlay](effects::Overlay) the [Transgender](flags::Flags::Transgender) flag with 40% [Opacity].
//!
//! ```rust
//! use pride_overlay::prelude::*;
//!
//! let mut image = image::open("path/to/image.webp")?;
//! let effect = Overlay::builder(TRANSGENDER).opacity(Opacity::new(0.4)).build();
//! effect.apply(&mut image);
//! ```

#[cfg(target_arch = "wasm32")]
mod wasm;

/// Built-in image effects and related types.
pub mod effects;

/// Built-in pride flags and related types.
pub mod flags;

mod opacity;
pub use opacity::Opacity;

/// Commonly used types and traits.
pub mod prelude {
    pub use crate::{
        Colour, Opacity,
        effects::{Effect, Overlay, Ring},
        flags::{Flags, SvgAsset, SvgScaleMode},
    };
}

/// Represents a colour in RGB format.
#[derive(Clone, Copy)]
pub struct Colour(pub(crate) u8, pub(crate) u8, pub(crate) u8);

impl Colour {
    pub const fn hex(hex: u32) -> Colour {
        let r = ((hex >> 16) & 0xFF) as u8;
        let g = ((hex >> 8) & 0xFF) as u8;
        let b = (hex & 0xFF) as u8;
        Colour(r, g, b)
    }
}
