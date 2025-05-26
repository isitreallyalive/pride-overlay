//! # Overview
//!
//! This crate provides various image processing utilities for pride flags, including overlaying flags onto images and drawing rings around images using flag colors.
//!
//! |       Input        |         [Overlay::apply]         |          [Ring::apply]          |
//! |:------------------:|:--------------------------------:|:-------------------------------:|
//! | ![](https://raw.githubusercontent.com/isitreallyalive/pride-overlay/refs/heads/main/pride-overlay/examples/input.webp) | ![](https://raw.githubusercontent.com/isitreallyalive/pride-overlay/refs/heads/main/pride-overlay/examples/out/overlay/intersex.webp) | ![](https://raw.githubusercontent.com/isitreallyalive/pride-overlay/refs/heads/main/pride-overlay/examples/out/ring/transgender.webp) |
//!
//! # High level API
//! Load an image with the [`image`](https://docs.rs/image) crate, and overlay [Flag::Transgender] with 40% opacity.
//!
//! ```rust
//! use pride_overlay::prelude::*;
//!
//! let mut image = image::open("path/to/image.webp")?;
//! let effect = Overlay::builder(Flag::Transgender).opacity(Opacity::new(0.4)).build();
//! effect.apply(&mut image);
//! ```

#[macro_use]
extern crate bon;

mod colour;
mod effect;
mod flags;

mod opacity;
pub mod prelude {
    pub use crate::{
        colour::Colour,
        effect::{Effect, Overlay, Ring},
        flags::{Flag, PrideFlag, ScaleMode},
        opacity::Opacity,
    };
}
#[doc(inline)]
pub use prelude::*;
