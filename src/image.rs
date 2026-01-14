#[cfg(feature = "gif")]
use image::{AnimationDecoder, Frame, codecs::gif::GifDecoder};
use image::{DynamicImage, ImageFormat};
use std::io::Cursor;

pub enum Image {
    Static(DynamicImage),
    #[cfg(feature = "gif")]
    Animated(Vec<Frame>),
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
                let frames: Vec<_> = decoder
                    .into_frames()
                    .collect::<Result<Vec<_>, _>>()?;
                println!("Decoded {} frames from GIF", frames.len());
                Image::Animated(frames)
            }
            _ => {
                // just read everything else as a static image
                Image::Static(image::load_from_memory_with_format(data, format)?)
            }
        };

        Ok(image)
    }

    pub fn to_bytes(&self, format: ImageFormat) -> Result<Vec<u8>, crate::PrideError> {
        let mut buf = Cursor::new(Vec::new());
        match self {
            Image::Static(img) => {
                img.write_to(&mut buf, format)?;
                Ok(buf.into_inner())
            }
            #[cfg(feature = "gif")]
            Image::Animated(frames) => {
                use image::codecs::gif::GifEncoder;

                // todo: make speed customisable
                let mut encoder = GifEncoder::new_with_speed(&mut buf, 1);
                for frame in frames {
                    encoder.encode_frame(frame.clone())?;
                }
                drop(encoder);

                Ok(buf.into_inner())
            }
        }
    }
}

impl From<DynamicImage> for Image {
    fn from(img: DynamicImage) -> Self {
        Image::Static(img)
    }
}
