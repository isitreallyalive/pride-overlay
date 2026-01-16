use std::io;

use image::{DynamicImage, GenericImageView, ImageFormat};

use crate::{Result, image::Format};

/// Any image that doesn't have a special handler.
pub struct Other {
    image: DynamicImage,
    format: ImageFormat,
}

impl Format for Other {
    fn read(data: &[u8]) -> Result<Self> {
        let format = image::guess_format(data)?;
        let img = image::load_from_memory_with_format(data, format)?;
        Ok(Self { image: img, format })
    }

    fn write<W: io::Write + io::Seek>(self, buf: &mut W) -> Result<()> {
        self.image.write_to(buf, self.format)?;
        Ok(())
    }

    fn apply<A>(&mut self, apply: A) -> Result<()>
    where
        A: Fn(&mut DynamicImage) -> Result<()>,
    {
        apply(&mut self.image)
    }
}

impl TryFrom<DynamicImage> for Other {
    type Error = crate::PrideError;

    fn try_from(image: DynamicImage) -> Result<Self> {
        let format = image::guess_format(&image.buffer_like())?;
        Ok(Self { image, format })
    }
}
