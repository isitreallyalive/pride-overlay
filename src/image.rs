use std::io::{Read, Seek, Write};

#[cfg(feature = "gif")]
use gif::{Frame, Repeat};
use image::{DynamicImage, GenericImageView, ImageFormat};

use crate::PrideError;

pub enum Image {
    Static {
        image: DynamicImage,
        format: ImageFormat,
    },
    #[cfg(feature = "gif")]
    Gif {
        width: u16,
        height: u16,
        frames: Vec<Vec<u8>>,
        delays: Vec<u16>,
        repeat: Repeat,
    },
}

impl Image {
    /// Read image data from a byte slice.
    pub fn read<R: Read>(data: R) -> Result<Self, crate::PrideError> {
        let mut buf = Vec::new();
        data.take(usize::MAX as u64).read_to_end(&mut buf)?;

        let format = image::guess_format(&buf)?;
        let image = match format {
            #[cfg(feature = "gif")]
            ImageFormat::Gif => {
                // decode gifs into frames
                let mut decoder = {
                    use std::io::Cursor;

                    let mut opts = gif::DecodeOptions::new();
                    opts.set_color_output(gif::ColorOutput::RGBA);
                    opts.read_info(Cursor::new(&buf))?
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

                Self::Gif {
                    width,
                    height,
                    frames,
                    delays,
                    repeat: decoder.repeat(),
                }
            }
            _ => {
                // just read everything else as a static image
                Self::Static {
                    image: image::load_from_memory_with_format(&buf, format)?,
                    format,
                }
            }
        };

        Ok(image)
    }

    pub fn write<W: Write + Seek>(self, buf: &mut W) -> Result<(), PrideError> {
        match self {
            Self::Static { image, format } => {
                image.write_to(buf, format)?;
            }
            #[cfg(feature = "gif")]
            Self::Gif {
                width,
                height,
                frames,
                delays,
                repeat,
            } => {
                let mut encoder = gif::Encoder::new(buf, width, height, &[])?;
                encoder.set_repeat(repeat)?;

                for (mut data, delay) in frames.into_iter().zip(delays) {
                    let mut frame = Frame::from_rgba_speed(width, height, &mut data, 1);
                    frame.delay = delay;
                    encoder.write_frame(&frame)?;
                }
                drop(encoder);
            }
        }

        Ok(())
    }
}

impl TryFrom<DynamicImage> for Image {
    type Error = PrideError;

    fn try_from(image: DynamicImage) -> Result<Self, PrideError> {
        let format = image::guess_format(&image.buffer_like())?;
        Ok(Image::Static { image, format })
    }
}
