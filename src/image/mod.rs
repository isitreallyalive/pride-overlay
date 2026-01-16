use std::io::{Seek, Write};

use image::{DynamicImage, ImageFormat};

use crate::PrideError;

#[cfg(feature = "gif")]
mod gif;
mod other;

#[cfg(any(feature = "gif"))]
pub trait Format: Sized {
    fn read(data: &[u8]) -> Result<Self, PrideError>;
    fn write<W: Write + Seek>(self, buf: &mut W) -> Result<(), PrideError>;
    fn apply<A: Fn(&mut DynamicImage)>(&mut self, apply: A);
}

pub enum Image {
    #[cfg(feature = "gif")]
    Gif(gif::Gif),
    Other(other::Other)
}

impl Image {
    /// Read image data from a byte slice.
    pub fn read(data: &[u8]) -> Result<Self, crate::PrideError> {
        let format = image::guess_format(data)?;

        match format {
            #[cfg(feature = "gif")]
            ImageFormat::Gif => gif::Gif::read(data).map(Self::Gif),
            _ => other::Other::read(data).map(Self::Other)
        }
    }

    pub fn write<W: Write + Seek>(self, buf: &mut W) -> Result<(), PrideError> {
        match self {
            #[cfg(feature = "gif")]
            Self::Gif(gif) => gif.write(buf),
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
