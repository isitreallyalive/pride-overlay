// todo: config controls

use std::borrow::Cow;
use std::io::{Cursor, Seek, Write};

use gif::{Decoder, Encoder, Frame, Repeat};
use image::{DynamicImage, RgbaImage};

use crate::{Result, image::Format};

/// A GIF image
pub struct Gif {
    frames: Vec<OwnedFrame>,
    width: u16,
    height: u16,
    repeat: Repeat,
}

/// A GIF frame with owned data
struct OwnedFrame {
    image: Cow<'static, [u8]>,
    width: u16,
    height: u16,
    delay: u16,
}

#[maybe_async::maybe_async(?Send)]
impl Format for Gif {
    async fn read(data: &[u8]) -> Result<Self> {
        // decode the gif
        let decoder = Decoder::new(Cursor::new(data))?;
        let (width, height) = (decoder.width(), decoder.height());
        let repeat = decoder.repeat();

        // collect frames
        let frames = decoder
            .into_iter()
            .filter_map(|f| f.ok())
            .map(|f| OwnedFrame {
                image: Cow::Owned(f.buffer.to_vec()),
                width: f.width,
                height: f.height,
                delay: f.delay,
            })
            .collect::<Vec<_>>();

        Ok(Self {
            width,
            height,
            repeat,
            frames,
        })
    }

    fn write<W: Write + Seek>(mut self, buf: &mut W) -> Result<()> {
        // initialise encoder
        let mut encoder = Encoder::new(buf, self.width, self.height, &[])?;
        encoder.set_repeat(self.repeat)?;

        // write processed frames
        for frame in self.frames.iter_mut().map(|f| {
            let mut frame = Frame::from_rgba_speed(f.width, f.height, f.image.to_mut(), 10);
            frame.delay = f.delay;
            frame
        }) {
            encoder.write_frame(&frame)?;
        }

        Ok(())
    }

    fn apply<A>(&mut self, apply: A) -> Result<()>
    where
        A: Fn(&mut DynamicImage) -> Result<()>,
    {
        for f in &mut self.frames {
            // gifs only support RGBA images
            let mut img = {
                let Some(rgba) = RgbaImage::from_raw(
                    f.width as u32,
                    f.height as u32,
                    std::mem::take(&mut f.image).into_owned(),
                ) else {
                    continue;
                };
                DynamicImage::ImageRgba8(rgba)
            };

            apply(&mut img)?;
            f.image = Cow::Owned(img.to_rgba8().into_raw());
        }
        Ok(())
    }
}
