use std::io::{Seek, Write};

use image::{DynamicImage, ImageFormat};

use crate::{Result, image::other::Other};

#[cfg(feature = "gif")]
mod gif;
mod other;
#[cfg(feature = "png")]
mod png;
#[cfg(feature = "webp")]
mod webp;

pub trait Format {
    fn read(data: &[u8]) -> Result<Self>
    where
        Self: Sized;

    fn write<W: Write + Seek>(self, buf: &mut W) -> Result<()>;

    fn apply<A>(&mut self, apply: A) -> Result<()>
    where
        A: Fn(&mut DynamicImage) -> Result<()>;
}

pub enum Image {
    #[cfg(feature = "gif")]
    Gif(gif::Gif),
    #[cfg(feature = "webp")]
    Webp(webp::WebP),
    #[cfg(feature = "png")]
    Png(png::Png),
    Other(Other),
}

impl Image {
    /// Read image data from a byte slice.
    pub fn read(data: &[u8]) -> Result<Self> {
        let format = image::guess_format(data)?;

        match format {
            #[cfg(feature = "gif")]
            ImageFormat::Gif => gif::Gif::read(data).map(Self::Gif),
            #[cfg(feature = "webp")]
            ImageFormat::WebP => webp::WebP::read(data).map(Self::Webp),
            #[cfg(feature = "png")]
            ImageFormat::Png => png::Png::read(data).map(Self::Png),
            _ => Other::read(data).map(Self::Other),
        }
    }

    pub fn write<W: Write + Seek>(self, buf: &mut W) -> Result<()> {
        match self {
            #[cfg(feature = "gif")]
            Self::Gif(gif) => gif.write(buf),
            #[cfg(feature = "webp")]
            Self::Webp(webp) => webp.write(buf),
            #[cfg(feature = "png")]
            Self::Png(png) => png.write(buf),
            Self::Other(other) => other.write(buf),
        }
    }
}

impl TryFrom<DynamicImage> for Image {
    type Error = crate::PrideError;

    fn try_from(image: DynamicImage) -> Result<Self> {
        Ok(Self::Other(Other::try_from(image)?))
    }
}
