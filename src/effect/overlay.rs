use image::{GenericImageView, Pixel, RgbaImage, imageops::overlay};
use imageproc::{drawing::draw_filled_rect_mut, rect::Rect};

use crate::{effect::Effect, flags::Flag};

#[derive(bon::Builder)]
#[builder(const)]
pub struct Overlay<'a> {
    #[builder(start_fn)]
    flag: Flag<'a>,
    #[builder(default = 0.5, with = |p: f32| p.clamp(0., 1.))]
    opacity: f32,
}

impl Effect for Overlay<'_> {
    fn apply_effect(&self, image: &mut image::DynamicImage) {
        if self.opacity == 0. {
            // no-op
            return;
        }

        // draw the flag
        let (width, height) = image.dimensions();
        let mut flag = RgbaImage::new(width, height);
        let count = self.flag.colours.len() as u32;
        let stripe_height = height / count;
        let mut start = 0;

        for (i, colour) in self.flag.colours.iter().enumerate() {
            // calculate the start and end y-coordinates for this stripe
            let end = if i as u32 == count - 1 {
                height
            } else {
                start + stripe_height
            };

            if end > start {
                let rect = Rect::at(0, start as i32).of_size(width, end - start);
                let mut colour = colour.to_rgba();
                colour.0[3] = (self.opacity * u8::MAX as f32) as u8;
                draw_filled_rect_mut(&mut flag, rect, colour);
            }

            start = end;
        }

        // overlay
        overlay(image, &flag, 0, 0);
    }
}
