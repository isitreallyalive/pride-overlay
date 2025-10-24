mod overlay;
pub use overlay::*;

mod ring;
pub use ring::*;

/// An effect that can be applied to an image.
pub trait Effect {
    /// Applies the effect to the given image.
    fn apply(&self, image: &mut image::DynamicImage);
}
