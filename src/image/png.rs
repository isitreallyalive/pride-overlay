use std::io::{self, Cursor};

use image::{DynamicImage, ImageBuffer, Rgba, imageops::overlay};
use imageproc::{drawing::draw_filled_rect_mut, rect::Rect};
use png::{AnimationControl, BitDepth, ColorType, Decoder, DisposeOp, Encoder, Info, OutputInfo};

use crate::{Result, image::Format};

pub struct Png {
    frames: Vec<Frame>,
    width: u32,
    height: u32,
    color_type: ColorType,
    bit_depth: BitDepth,
    animation: Option<AnimationControl>,
}

struct Frame {
    image: DynamicImage,
    delay: Option<(u16, u16)>,
}

#[maybe_async::maybe_async(?Send)]
impl Format for Png {
    async fn read(data: &[u8]) -> Result<Self> {
        // decode png data
        let mut reader = Decoder::new(Cursor::new(data)).read_info()?;
        let Info {
            width,
            height,
            color_type,
            bit_depth,
            animation_control: animation,
            ..
        } = *reader.info();

        // output buffer and canvas
        let mut buf = vec![0; reader.output_buffer_size().unwrap_or(0)];
        let mut canvas = animation.map(|_| {
            DynamicImage::new(width, height, {
                use image::ColorType as Img;

                match color_type {
                    ColorType::Grayscale => Img::L8,
                    ColorType::Rgb => Img::Rgb8,
                    ColorType::Indexed => Img::Rgba8,
                    ColorType::GrayscaleAlpha => Img::La8,
                    ColorType::Rgba => Img::Rgba8,
                }
            })
        });
        let mut prev_canvas = canvas.clone();

        // collect frames
        let mut frames = Vec::with_capacity(animation.map_or(1, |ac| ac.num_frames as usize));

        while let Ok(OutputInfo { .. }) = reader.next_frame(&mut buf) {
            let control = reader.info().frame_control;

            if let Some((control, canvas_img)) = control.zip(canvas.as_mut()) {
                // apng
                let Some(frame_img) = buf_to_image(
                    control.width,
                    control.height,
                    buf[..(control.width * control.height) as usize * color_type.samples()]
                        .to_vec(),
                    color_type,
                ) else {
                    continue;
                };

                // handle dispose previous
                if control.dispose_op == DisposeOp::Previous {
                    prev_canvas = Some(canvas_img.clone());
                }

                // composite frame onto canvas
                overlay(
                    canvas_img,
                    &frame_img,
                    control.x_offset as i64,
                    control.y_offset as i64,
                );

                frames.push(Frame {
                    image: canvas_img.clone(),
                    delay: Some((control.delay_num, control.delay_den)),
                });

                // handle dispose operation after frame
                match control.dispose_op {
                    DisposeOp::None => {}
                    // clear to background (transparent)
                    DisposeOp::Background => draw_filled_rect_mut(
                        canvas_img,
                        Rect::at(control.x_offset as i32, control.y_offset as i32)
                            .of_size(control.width, control.height),
                        Rgba([0, 0, 0, 0]),
                    ),
                    // clear to previous
                    DisposeOp::Previous => canvas = prev_canvas.clone(),
                }
            } else {
                // png
                let Some(image) = buf_to_image(width, height, buf.clone(), color_type) else {
                    continue;
                };
                frames.push(Frame { image, delay: None });
            }
        }

        Ok(Self {
            width,
            height,
            frames,
            color_type,
            bit_depth,
            animation,
        })
    }

    fn write<W: io::Write + io::Seek>(self, buf: &mut W) -> Result<()> {
        // configure encoder
        let mut encoder = Encoder::new(buf, self.width, self.height);
        encoder.set_color(self.color_type);
        encoder.set_depth(self.bit_depth);

        if let Some(a) = self.animation {
            encoder.set_animated(a.num_frames, a.num_plays)?;
        }

        // start writing
        let mut writer = encoder.write_header()?;

        for frame in self.frames {
            // set delay
            frame
                .delay
                .and_then(|(num, den)| writer.set_frame_delay(num, den).ok());

            // write the frame
            let data = match self.color_type {
                ColorType::Grayscale => frame.image.to_luma8().into_raw(),
                ColorType::Rgb => frame.image.to_rgb8().into_raw(),
                ColorType::Indexed => frame.image.to_rgba8().into_raw(),
                ColorType::GrayscaleAlpha => frame.image.to_luma_alpha8().into_raw(),
                ColorType::Rgba => frame.image.to_rgba8().into_raw(),
            };
            writer.write_image_data(&data)?;
        }

        writer.finish()?;

        Ok(())
    }

    fn apply<A>(&mut self, apply: A) -> Result<()>
    where
        A: Fn(&mut image::DynamicImage) -> Result<()>,
    {
        for frame in &mut self.frames {
            apply(&mut frame.image)?;
        }
        Ok(())
    }
}

fn buf_to_image(
    width: u32,
    height: u32,
    buf: Vec<u8>,
    color_type: ColorType,
) -> Option<DynamicImage> {
    match color_type {
        ColorType::Grayscale => {
            ImageBuffer::from_raw(width, height, buf).map(DynamicImage::ImageLuma8)
        }
        ColorType::Rgb => ImageBuffer::from_raw(width, height, buf).map(DynamicImage::ImageRgb8),
        ColorType::Indexed => {
            ImageBuffer::from_raw(width, height, buf).map(DynamicImage::ImageRgba8)
        }
        ColorType::GrayscaleAlpha => {
            ImageBuffer::from_raw(width, height, buf).map(DynamicImage::ImageLumaA8)
        }
        ColorType::Rgba => ImageBuffer::from_raw(width, height, buf).map(DynamicImage::ImageRgba8),
    }
}
