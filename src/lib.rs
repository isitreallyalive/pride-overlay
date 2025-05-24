pub use colour::Colour;
pub use flags::Flag;

use image::{DynamicImage, Rgba, RgbaImage, imageops::overlay};
use imageproc::{drawing::draw_filled_rect_mut, rect::Rect};

mod colour;
mod flags;

impl Flag {
    pub fn overlay(&self, image: &mut DynamicImage) {
        let image_rgba = image.to_rgba8();
        let (width, height) = image_rgba.dimensions();

        // draw the pride flag
        let mut flag = RgbaImage::new(width, height);
        let colours = self.colours();
        let colour_size = height / colours.len() as u32;

        for (i, Colour { r, g, b }) in colours.iter().enumerate() {
            let rect = Rect::at(0, (i as u32 * colour_size) as i32).of_size(width, colour_size);
            // todo: custom opacity
            let colour = Rgba([*r, *g, *b, 127]);
            draw_filled_rect_mut(&mut flag, rect, colour);
        }

        overlay(image, &flag, 0, 0);
    }
}
