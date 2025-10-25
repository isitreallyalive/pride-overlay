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
//! let effect = Overlay::builder(Flags::Transgender).opacity(Opacity::new(0.4)).build();
//! effect.apply(&mut image);
//! ```

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
#[macro_use]
extern crate serde;
#[cfg(target_arch = "wasm32")]
#[macro_use]
extern crate tsify;

mod colour;
#[doc(inline)]
pub use colour::Colour;

/// Built-in image effects and related types.
pub mod effects;

/// Built-in pride flags and related types.
pub mod flags;

/// Commonly used types and traits.
pub mod prelude {
    pub use crate::{
        Colour,
        effects::{Effect, Overlay, Ring},
        flags::{Flags, SvgAsset, SvgScaleMode},
    };
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(start)]
pub fn init() {
    // panic to the console
    console_error_panic_hook::set_once();
}
