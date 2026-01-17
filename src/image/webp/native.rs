// todo: don't unwrap
// todo: config controls

use std::borrow::Cow;
use std::io;

use image::DynamicImage;
use webp::{AnimDecoder, AnimEncoder, AnimFrame, WebPConfig};

use crate::{Result, image::Format};

/// A WebP image
pub struct WebP {
    frames: Vec<OwnedFrame>,
    width: u32,
    height: u32,
}

/// A WebP frame with owned data
struct OwnedFrame {
    image: Cow<'static, [u8]>,
    width: u32,
    height: u32,
    timestamp: i32,
}

impl Format for WebP {
    fn read(data: &[u8]) -> Result<Self> {
        // decode the animated webp
        let decoder = AnimDecoder::new(data).decode().unwrap();
        let raw_frames = decoder.get_frames(0..decoder.len()).unwrap();

        // determine the overall width and height of the canvas
        let (width, height) = {
            let largest_frame = raw_frames.iter().max_by_key(|f| f.width() * f.height());
            largest_frame
                .map(|f| (f.width(), f.height()))
                .unwrap_or((0, 0))
        };

        // collect frames
        let frames = raw_frames
            .into_iter()
            .map(|f| OwnedFrame {
                image: Cow::Owned(f.get_image().to_vec()),
                width: f.width(),
                height: f.height(),
                timestamp: f.get_time_ms(),
            })
            .collect();

        Ok(Self {
            width,
            height,
            frames,
        })
    }

    fn write<W: io::Write + io::Seek>(self, buf: &mut W) -> Result<()> {
        // intialise encoder
        let conf = WebPConfig::new().unwrap();
        let mut encoder = AnimEncoder::new(self.width, self.height, &conf);

        // write processed frames
        for f in &self.frames {
            let frame = AnimFrame::from_rgba(&f.image, f.width, f.height, f.timestamp);
            encoder.add_frame(frame);
        }

        let data = encoder.encode();
        buf.write_all(&data)?;

        Ok(())
    }

    fn apply<A>(&mut self, apply: A) -> Result<()>
    where
        A: Fn(&mut DynamicImage) -> Result<()>,
    {
        for f in &mut self.frames {
            let frame = AnimFrame::from_rgba(&f.image, f.width, f.height, f.timestamp);
            let mut img: DynamicImage = (&frame).into();
            apply(&mut img)?;
            f.image = Cow::Owned(img.to_rgba8().into_raw());
        }

        Ok(())
    }
}
