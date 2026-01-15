#[cfg(feature = "gif")]
use gif::{Frame, Repeat};
use image::{DynamicImage, ImageFormat};
use std::io::Cursor;

pub enum Image {
    Static(DynamicImage),
    #[cfg(feature = "gif")]
    Animated {
        width: u16,
        height: u16,
        frames: Vec<Vec<u8>>,
        delays: Vec<u16>,
        repeat: Repeat,
    },
}

impl Image {
    /// Read image data from a byte slice.
    pub fn read(data: &[u8]) -> Result<Self, crate::PrideError> {
        let format = image::guess_format(data)?;
        let image = match format {
            #[cfg(feature = "gif")]
            ImageFormat::Gif => {
                // decode gifs into frames
                let mut decoder = {
                    let mut opts = gif::DecodeOptions::new();
                    opts.set_color_output(gif::ColorOutput::RGBA);
                    opts.read_info(data)?
                };
                let width = decoder.width();
                let height = decoder.height();
                let mut frames = Vec::new();
                let mut delays = Vec::new();

                while let Ok(Some(frame)) = decoder.read_next_frame() {
                    // convert frame to full canvas size
                    let mut canvas = vec![0u8; (width as usize) * (height as usize) * 4];

                    // copy frame data to the correct position in the canvas
                    for y in 0..frame.height {
                        for x in 0..frame.width {
                            let frame_idx =
                                ((y as usize) * (frame.width as usize) + (x as usize)) * 4;
                            let canvas_x = frame.left + x;
                            let canvas_y = frame.top + y;
                            let canvas_idx =
                                ((canvas_y as usize) * (width as usize) + (canvas_x as usize)) * 4;

                            canvas[canvas_idx..canvas_idx + 4]
                                .copy_from_slice(&frame.buffer[frame_idx..frame_idx + 4]);
                        }
                    }

                    frames.push(canvas);
                    delays.push(frame.delay);
                }

                Self::Animated {
                    width,
                    height,
                    frames,
                    delays,
                    repeat: decoder.repeat(),
                }
            }
            _ => {
                // just read everything else as a static image
                Self::Static(image::load_from_memory_with_format(data, format)?)
            }
        };

        Ok(image)
    }

    pub fn to_bytes(self, format: ImageFormat) -> Result<Vec<u8>, crate::PrideError> {
        let mut buf = Cursor::new(Vec::new());
        match self {
            Self::Static(img) => {
                img.write_to(&mut buf, format)?;
                Ok(buf.into_inner())
            }
            #[cfg(feature = "gif")]
            Self::Animated {
                width,
                height,
                frames,
                delays,
                repeat,
            } => {
                let mut encoder = gif::Encoder::new(&mut buf, width, height, &[])?;
                encoder.set_repeat(repeat)?;

                for (mut data, delay) in frames.into_iter().zip(delays) {
                    let mut frame = Frame::from_rgba_speed(width, height, &mut data, 1);
                    frame.delay = delay;
                    encoder.write_frame(&frame)?;
                }
                drop(encoder);

                Ok(buf.into_inner())
            }
        }
    }
}

impl From<DynamicImage> for Image {
    fn from(img: DynamicImage) -> Self {
        Image::Static(img)
    }
}
