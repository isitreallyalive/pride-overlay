use std::io::{Cursor, Seek, Write};

use gif::{ColorOutput, Encoder, Repeat};
use image::{DynamicImage, RgbaImage};

use crate::{PrideError, image::Format};

// todo: only keep necessary data
pub struct Gif {
    width: u16,
    height: u16,
    repeat: Repeat,
    frames: Vec<GifFrame>,
}

struct GifFrame {
    data: Vec<u8>,
    delay: u16,
}

impl Format for Gif {
    fn read(data: &[u8]) -> Result<Self, PrideError> {
        let decoder = {
            let mut opts = gif::DecodeOptions::new();
            opts.set_color_output(ColorOutput::RGBA);
            opts.read_info(Cursor::new(data))?
        };
        let (width, height) = (decoder.width(), decoder.height());
        let repeat = decoder.repeat();
        let frames = decoder
            .into_iter()
            .filter_map(|f| f.ok())
            .map(|f| {
                let mut canvas = vec![0; (width as usize) * (height as usize) * 4];

                // copy frame data to the correct position in the canvas
                for y in 0..f.height {
                    for x in 0..f.width {
                        let frame_idx = ((y as usize) * (f.width as usize) + (x as usize)) * 4;
                        let canvas_x = f.left + x;
                        let canvas_y = f.top + y;
                        let canvas_idx =
                            ((canvas_y as usize) * (width as usize) + (canvas_x as usize)) * 4;

                        canvas[canvas_idx..canvas_idx + 4]
                            .copy_from_slice(&f.buffer[frame_idx..frame_idx + 4]);
                    }
                }

                GifFrame {
                    data: canvas,
                    delay: f.delay,
                }
            })
            .collect::<Vec<_>>();
        Ok(Self {
            width,
            height,
            repeat,
            frames,
        })
    }

    fn write<W: Write + Seek>(mut self, buf: &mut W) -> Result<(), PrideError> {
        let mut encoder = Encoder::new(buf, self.width, self.height, &[])?;
        encoder.set_repeat(self.repeat)?;

        let frames = self
            .frames
            .iter_mut()
            .map(|f| {
                let mut frame = gif::Frame::from_rgba_speed(self.width, self.height, &mut f.data, 10);
                frame.delay = f.delay;
                frame
            });
        for frame in frames {
            encoder.write_frame(&frame)?;
        }

        Ok(())
    }

    fn apply<A: Fn(&mut DynamicImage)>(&mut self, apply: A) {
        for frame in &mut self.frames {
            let mut img = {
                let Some(rgba) = RgbaImage::from_raw(
                    self.width as u32,
                    self.height as u32,
                    std::mem::take(&mut frame.data),
                ) else {
                    continue;
                };
                DynamicImage::ImageRgba8(rgba)
            };

            apply(&mut img);
            frame.data = img.to_rgba8().into_raw();
        }
    }
}
