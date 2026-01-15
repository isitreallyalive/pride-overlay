use image::{GenericImageView, Pixel};
use imageproc::{drawing::draw_filled_rect_mut, rect::Rect};

use crate::{effect::Effect, flags::Flag};

#[derive(bon::Builder)]
#[builder(const)]
pub struct Overlay<'a> {
    #[builder(start_fn)]
    flag: Flag<'a>,
}

impl Effect for Overlay<'_> {
    fn apply_effect(&self, image: &mut image::DynamicImage) {
        let (width, height) = image.dimensions();
        let count = self.flag.colours.len() as u32;

        for (colour, i) in self.flag.colours.iter().zip(0..count) {
            // calculate the start and end y-coordinates for this stripe
            let start = (i * height) / count;
            let end = ((i + 1) * height) / count;

            if end > start {
                let rect = Rect::at(0, start as i32).of_size(width, end - start);
                draw_filled_rect_mut(image, rect, colour.to_rgba());
            }
        }
    }
}
