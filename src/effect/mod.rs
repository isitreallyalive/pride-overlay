use image::DynamicImage;

use crate::{
    PrideError,
    image::{Format, Image},
};

pub mod overlay;

/// An effect that can be applied to an image.
pub trait Effect {
    /// Apply the effect to the given [Image].
    fn apply(&self, image: &mut Image) -> Result<(), PrideError> {
        match image {
            #[cfg(feature = "gif")]
            Image::Gif(gif) => gif.apply(|f| self.apply_effect(f)),
            #[cfg(feature = "webp")]
            Image::Webp(webp) => webp.apply(|f| self.apply_effect(f)),
            Image::Other(other) => other.apply(|f| self.apply_effect(f)),
        }
    }

    /// Apply the effect to a single static image.
    #[doc(hidden)]
    fn apply_effect(&self, image: &mut DynamicImage) -> Result<(), PrideError>;
}
