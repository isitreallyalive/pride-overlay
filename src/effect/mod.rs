use image::DynamicImage;

use crate::image::Image;

pub mod overlay;

/// An effect that can be applied to an image.
pub trait Effect: Sync {
    /// Apply the effect to the given [Image].
    fn apply(&self, image: &mut Image) {
        match image {
            Image::Static(img) => self.apply_effect(img),
            #[cfg(feature = "gif")]
            Image::Animated(frames) => {
                #[cfg(feature = "rayon")]
                {
                    use rayon::prelude::*;

                    frames
                        .par_iter_mut()
                        .for_each(|frame| self.process_frame(frame));
                }

                #[cfg(not(feature = "rayon"))]
                frames
                    .iter_mut()
                    .for_each(|frame| self.process_frame(frame));
            }
        }
    }

    /// Process a single frame of an animated image.
    #[cfg(feature = "gif")]
    #[doc(hidden)]
    fn process_frame(&self, frame: &mut image::Frame) {
        // directly manipulate the frame buffer to avoid cloning
        let buf = frame.buffer_mut();
        let mut img = DynamicImage::ImageRgba8(std::mem::take(buf));
        self.apply_effect(&mut img);
        *buf = img.to_rgba8();
    }

    /// Apply the effect to a single static image.
    #[doc(hidden)]
    fn apply_effect(&self, image: &mut DynamicImage);
}
