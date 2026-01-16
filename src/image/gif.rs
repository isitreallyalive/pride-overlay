use std::io::{Cursor, Seek, Write};

use gif::{ColorOutput, Encoder, Frame, Repeat};
use image::{DynamicImage, RgbaImage};

use crate::{PrideError, image::Format};

/// A GIF image
pub struct Gif {
    frames: Vec<OwnedFrame>,
    width: u16,
    height: u16,
    repeat: Repeat,
}

impl Format for Gif {
    fn read(data: &[u8]) -> Result<Self, PrideError> {
        // decode the gif
        let decoder = {
            let mut opts = gif::DecodeOptions::new();
            opts.set_color_output(ColorOutput::RGBA);
            opts.read_info(Cursor::new(data))?
        };
        let (width, height) = (decoder.width(), decoder.height());
        let repeat = decoder.repeat();

        // collect frames
        let frames = decoder
            .into_iter()
            .filter_map(|f| f.ok())
            .map(OwnedFrame::from)
            .collect::<Vec<_>>();

        Ok(Self {
            width,
            height,
            repeat,
            frames,
        })
    }

    fn write<W: Write + Seek>(self, buf: &mut W) -> Result<(), PrideError> {
        // initialise encoder
        let mut encoder = Encoder::new(buf, self.width, self.height, &[])?;
        encoder.set_repeat(self.repeat)?;

        // write processed frames
        for frame in self
            .frames
            .into_iter()
            .filter_map(|f| Frame::try_from(f).ok())
        {
            encoder.write_frame(&frame)?;
        }

        Ok(())
    }

    fn apply<A: Fn(&mut DynamicImage)>(&mut self, apply: A) {
        for f in &mut self.frames {
            // convert to Rgba8 dynamic image
            let mut img = {
                let Some(rgba) = RgbaImage::from_raw(
                    f.width as u32,
                    f.height as u32,
                    std::mem::take(&mut f.image),
                ) else {
                    continue;
                };
                DynamicImage::ImageRgba8(rgba)
            };

            apply(&mut img);
            f.image = img.to_rgba8().into_raw();
        }
    }
}

/// A GIF frame with owned data
struct OwnedFrame {
    image: Vec<u8>,
    width: u16,
    height: u16,
    delay: u16,
}

impl From<Frame<'_>> for OwnedFrame {
    fn from(f: Frame<'_>) -> Self {
        Self {
            image: f.buffer.to_vec(),
            width: f.width,
            height: f.height,
            delay: f.delay,
        }
    }
}

impl TryFrom<OwnedFrame> for Frame<'_> {
    type Error = PrideError;

    fn try_from(mut f: OwnedFrame) -> Result<Self, PrideError> {
        let mut frame = Frame::from_rgba_speed(f.width, f.height, &mut f.image, 10);
        frame.delay = f.delay;
        Ok(frame)
    }
}
