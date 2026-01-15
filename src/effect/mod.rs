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
            Image::Animated {
                width,
                height,
                frames,
                ..
            } => {
                #[cfg(feature = "rayon")]
                {
                    use rayon::prelude::*;

                    frames
                        .into_par_iter()
                        .for_each(|frame| self.process_frame(*width, *height, frame));
                }

                #[cfg(not(feature = "rayon"))]
                frames
                    .iter_mut()
                    .for_each(|frame| self.process_frame(*width, *height, frame));
            }
        }
    }

    /// Process a single frame of an animated image.
    #[cfg(feature = "gif")]
    #[doc(hidden)]
    fn process_frame(&self, width: u16, height: u16, frame: &mut Vec<u8>) {
        // directly manipulate the frame buffer to avoid cloning
        let mut img = DynamicImage::ImageRgba8(
            image::RgbaImage::from_raw(width as u32, height as u32, std::mem::take(frame)).unwrap(),
        );
        self.apply_effect(&mut img);
        *frame = img.to_rgba8().into_raw();
    }

    /// Apply the effect to a single static image.
    #[doc(hidden)]
    fn apply_effect(&self, image: &mut DynamicImage);
}
