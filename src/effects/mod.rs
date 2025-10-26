use crate::flags::FlagData;

mod overlay;
pub use overlay::*;

mod ring;
pub use ring::*;

#[cfg(target_arch = "wasm32")]
mod wasm;

/// An effect that can be applied to an image.
pub trait Effect {
    fn apply<F: FlagData>(&self, image: &mut image::DynamicImage, flag: F);
}
