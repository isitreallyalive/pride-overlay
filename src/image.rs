#[cfg(feature = "gif")]
use image::{AnimationDecoder, codecs::gif::GifDecoder};
use image::{DynamicImage, ImageFormat};
#[cfg(feature = "gif")]
use std::io::Cursor;

pub enum Image {
    Static(DynamicImage),
    #[cfg(feature = "gif")]
    Animated(Vec<DynamicImage>),
}

impl Image {
    /// Read image data from a byte slice.
    pub fn read(data: &[u8]) -> Result<Image, crate::PrideError> {
        let format = image::guess_format(data)?;
        let image = match format {
            #[cfg(feature = "gif")]
            ImageFormat::Gif => {
                // decode gifs into frames
                let decoder = GifDecoder::new(Cursor::new(data))?;
                let frames = decoder
                    .into_frames()
                    .collect::<Result<Vec<_>, _>>()?
                    .into_iter()
                    .map(|frame| DynamicImage::ImageRgba8(frame.into_buffer()))
                    .collect::<Vec<_>>();
                Image::Animated(frames)
            }
            _ => {
                // just read everything else as a static image
                Image::Static(image::load_from_memory_with_format(data, format)?)
            }
        };

        Ok(image)
    }
}

impl From<DynamicImage> for Image {
    fn from(img: DynamicImage) -> Self {
        Image::Static(img)
    }
}