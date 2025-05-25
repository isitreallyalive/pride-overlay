//! # Overview
//!
//! This crate provides various image processing utilities for pride flags, including overlaying flags onto images and drawing rings around images using flag colors.
//! It is designed to work in a `#![no_std]` environment.
//!
//! |       Input        |         [Flag::overlay]         |          [Flag::ring]          |
//! |:------------------:|:-------------------------------:|:------------------------------:|
//! | ![](https://raw.githubusercontent.com/isitreallyalive/pride-overlay/refs/heads/v2/examples/input.webp) | ![](https://raw.githubusercontent.com/isitreallyalive/pride-overlay/refs/heads/v2/examples/out/overlay/pansexual.webp) | ![](https://raw.githubusercontent.com/isitreallyalive/pride-overlay/refs/heads/v2/examples/out/ring/transgender.webp) |
//!
//! # High level API
//! Load an image with the [`image`](https://docs.rs/image) crate, and overlay [Flag::Transgender] with default opacity.
//!
//! ```rust
//! use pride_overlay::Flag;
//!
//! let mut image = image::open("path/to/image.webp")?;
//! Flag::Transgender.overlay(&mut image);
//! ```

#![no_std]
extern crate alloc;

mod colour;
mod flags;
mod opacity;

mod overlay;
mod ring;

pub use colour::Colour;
pub use flags::Flag;
pub use opacity::Opacity;

use image::DynamicImage;

impl Flag {
    /// Overlays the flag onto the given image. The image is modified in place.
    pub fn overlay(&self, image: &mut DynamicImage) {
        overlay::apply(self, image, Opacity::HALF);
    }

    /// Draws a ring around the image using the flag's colours. The image is modified in place.
    pub fn ring(&self, image: &mut DynamicImage) {
        ring::ring(self, image, Opacity::OPAQUE, None);
    }
}
