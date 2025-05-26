pub use overlay::*;
pub use ring::*;

mod overlay;
mod ring;

/// An effect that can be applied to an image.
pub trait Effect {
    /// Applies the effect to the given image.
    fn apply(&self, image: &mut image::DynamicImage);
}
