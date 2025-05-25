//! # Overview
//!
//! This crate provides various image processing utilities for pride flags, including overlaying flags onto images and drawing rings around images using flag colors.
//! It is designed to work in a `#![no_std]` environment.
//!
//! |       Input        |         [Overlay::apply]         |          [Ring::apply]          |
//! |:------------------:|:--------------------------------:|:-------------------------------:|
//! | ![](https://raw.githubusercontent.com/isitreallyalive/pride-overlay/refs/heads/main/examples/input.webp) | ![](https://raw.githubusercontent.com/isitreallyalive/pride-overlay/refs/heads/main/examples/out/overlay/intersex.webp) | ![](https://raw.githubusercontent.com/isitreallyalive/pride-overlay/refs/heads/main/examples/out/ring/transgender.webp) |
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

#![no_std]
extern crate alloc;

#[macro_use]
extern crate bon;

mod colour;
mod flags;
mod opacity;

mod overlay;
mod ring;

pub mod prelude {
    pub use crate::{
        Effect, colour::Colour, flags::Flag, opacity::Opacity, overlay::Overlay, ring::Ring,
    };
}
#[doc(inline)]
pub use prelude::*;

/// An effect that can be applied to an image.
pub trait Effect {
    /// Applies the effect to the given image.
    fn apply(&self, image: &mut image::DynamicImage);
}
