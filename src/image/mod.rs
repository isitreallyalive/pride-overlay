use std::io::{Seek, Write};

use image::{DynamicImage, ImageFormat};

use crate::PrideError;

#[cfg(feature = "gif")]
mod gif;
mod other;
#[cfg(feature = "webp")]
mod webp;

pub trait Format {
    fn read(data: &[u8]) -> Result<Self, PrideError>
    where
        Self: Sized;
    fn write<W: Write + Seek>(self, buf: &mut W) -> Result<(), PrideError>;
    fn apply<A: Fn(&mut DynamicImage)>(&mut self, apply: A);
}

pub enum Image {
    #[cfg(feature = "gif")]
    Gif(gif::Gif),
    #[cfg(feature = "webp")]
    Webp(webp::WebP),
    Other(other::Other),
}

impl Image {
    /// Read image data from a byte slice.
    pub fn read(data: &[u8]) -> Result<Self, crate::PrideError> {
        let format = image::guess_format(data)?;

        match format {
            #[cfg(feature = "gif")]
            ImageFormat::Gif => gif::Gif::read(data).map(Self::Gif),
            #[cfg(feature = "webp")]
            ImageFormat::WebP => webp::WebP::read(data).map(Self::Webp),
            _ => other::Other::read(data).map(Self::Other),
        }
    }

    pub fn write<W: Write + Seek>(self, buf: &mut W) -> Result<(), PrideError> {
        match self {
            #[cfg(feature = "gif")]
            Self::Gif(gif) => gif.write(buf),
            #[cfg(feature = "webp")]
            Self::Webp(webp) => webp.write(buf),
            Self::Other(other) => other.write(buf),
        }
    }
}

impl TryFrom<DynamicImage> for Image {
    type Error = PrideError;

    fn try_from(image: DynamicImage) -> Result<Self, PrideError> {
        Ok(Self::Other(other::Other::try_from(image)?))
    }
}
