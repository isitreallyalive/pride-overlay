use std::io;

use image::{DynamicImage, GenericImageView, ImageFormat};

use crate::image::Format;

pub struct Other {
    data: DynamicImage,
    format: ImageFormat
}

impl Format for Other {
    fn read(data: &[u8]) -> Result<Self, crate::PrideError> {
        let format = image::guess_format(data)?;
        let img = image::load_from_memory_with_format(data, format)?;
        Ok(Self { data: img, format })
    }

    fn write<W: io::Write + io::Seek>(self, buf: &mut W) -> Result<(), crate::PrideError> {
        self.data.write_to(buf, self.format)?;
        Ok(())
    }

    fn apply<A: Fn(&mut DynamicImage)>(&mut self, apply: A) {
        apply(&mut self.data);
    }
}

impl TryFrom<DynamicImage> for Other {
    type Error = crate::PrideError;

    fn try_from(image: DynamicImage) -> Result<Self, crate::PrideError> {
        let format = image::guess_format(&image.buffer_like())?;
        Ok(Self {
            data: image,
            format
        })
    }
}