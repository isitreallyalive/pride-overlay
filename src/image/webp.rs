// todo: don't unwrap

use std::io;

use webp::{AnimDecoder, AnimEncoder, AnimFrame, WebPConfig};

use crate::image::Format;

pub struct WebP {
    frames: Vec<OwnedFrame>,
    width: u32,
    height: u32,
}

impl Format for WebP {
    fn read(data: &[u8]) -> Result<Self, crate::PrideError> {
        // decode the animated webp
        let decoder = AnimDecoder::new(data).decode().unwrap();
        let raw_frames = decoder.get_frames(0..decoder.len()).unwrap();
        
        // determine the overall width and height of the canvas
        let (width, height) = {
            let largest_frame = raw_frames.iter().max_by_key(|f| f.width() * f.height());
            largest_frame.map(|f| (f.width(), f.height()))
                .unwrap_or((0, 0))
        };
        
        // collect frames
        let frames = raw_frames
            .into_iter()
            .map(OwnedFrame::from)
            .collect();

        Ok(Self {
            width,
            height,
            frames,
        })
    }

    fn write<W: io::Write + io::Seek>(self, buf: &mut W) -> Result<(), crate::PrideError> {
        // intialise encoder
        let conf = WebPConfig::new().unwrap();
        let mut encoder = AnimEncoder::new(self.width, self.height, &conf);

        // write processed frames
        for frame in &self.frames {
            encoder.add_frame(AnimFrame::try_from(frame)?);
        }
        
        let data = encoder.encode();
        buf.write_all(&data)?;

        Ok(())
    }

    fn apply<A: Fn(&mut image::DynamicImage)>(&mut self, apply: A) {
        for frame in &mut self.frames {
            let mut img = image::DynamicImage::ImageRgba8(
                image::ImageBuffer::from_raw(frame.width, frame.height, frame.image.clone())
                    .unwrap(),
            );
            apply(&mut img);
            frame.image = img.to_rgba8().into_raw();
        }
    }
}


struct OwnedFrame {
    image: Vec<u8>,
    width: u32,
    height: u32,
    timestamp: i32,
}

impl From<AnimFrame<'_>> for OwnedFrame {
    fn from(f: AnimFrame<'_>) -> Self {
        Self {
            image: f.get_image().to_vec(),
            width: f.width(),
            height: f.height(),
            timestamp: f.get_time_ms(),
        }
    }
}

impl<'a> TryFrom<&'a OwnedFrame> for AnimFrame<'a> {
    type Error = crate::PrideError;

    fn try_from(frame: &'a OwnedFrame) -> Result<Self, crate::PrideError> {
        Ok(AnimFrame::from_rgba(
            &frame.image,
            frame.width,
            frame.height,
            frame.timestamp,
        ))
    }
}