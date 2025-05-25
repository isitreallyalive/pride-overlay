#![no_std]

pub use colour::Colour;
pub use flags::Flag;
pub use opacity::Opacity;

use flags::FlagData;
use image::{DynamicImage, Rgba, RgbaImage, imageops::overlay};
use imageproc::{
    drawing::{draw_filled_circle_mut, draw_filled_rect_mut},
    rect::Rect,
};

mod colour;
mod flags;
mod opacity;

fn draw<F>(flag: &Flag, image: &mut DynamicImage, opacity: Opacity, mut flag_transform: Option<F>)
where
    F: for<'a> FnMut(&'a mut RgbaImage, u32, u32),
{
    let image_rgba = image.to_rgba8();
    let (width, height) = image_rgba.dimensions();

    // draw flag
    let a = opacity.get_raw();
    let mut flag_image = match flag.data() {
        FlagData::Colours(colours) => {
            let mut flag_image = RgbaImage::new(width, height);

            let colour_size = height / colours.len() as u32;

            colours
                .iter()
                .enumerate()
                .for_each(|(i, &Colour { r, g, b })| {
                    let y_start = (i as u32 * colour_size) as i32;
                    let rect = Rect::at(0, y_start).of_size(width, colour_size);
                    let colour = Rgba([r, g, b, a]);
                    draw_filled_rect_mut(&mut flag_image, rect, colour);
                });

            flag_image
        }
        FlagData::Image(data) => {
            let mut flag_image = image::load_from_memory(data)
                .expect("failed to load flag image")
                .resize_to_fill(width, height, image::imageops::FilterType::Nearest)
                .to_rgba8();

            for pixel in flag_image.pixels_mut() {
                pixel[3] = a;
            }

            flag_image
        }
    };

    // transform flag
    if let Some(ref mut trans_fn) = flag_transform {
        trans_fn(&mut flag_image, width, height);
    }

    overlay(image, &flag_image, 0, 0);
}

impl Flag {
    /// Overlays the flag onto the given image. The image is modified in place.
    ///
    /// #### Implementation Details
    /// The `opacity` parameter controls the transparency of the flag overlay.
    /// If [None], the default opacity of 50% is used.
    pub fn overlay(&self, image: &mut DynamicImage, opacity: Option<Opacity>) {
        draw(
            self,
            image,
            opacity.unwrap_or_default(),
            None::<fn(&mut RgbaImage, u32, u32)>,
        )
    }

    /// Draws a ring around the image using the flag's colours. The image is modified in place.
    ///
    /// #### Implementation Details
    /// The `opacity` parameter controls the transparency of the ring.
    /// If [None], the default opacity of 100% is used.
    ///
    /// The ring thickness is calculated as an offset from the image edge:
    /// - When thickness is specified: offset = min(thickness, 10) * 8 (capped at 80 pixels)
    /// - When thickness is [None]: offset = 12 pixels (default)
    /// - Inner radius = (image_width / 2) - offset
    pub fn ring(&self, image: &mut DynamicImage, opacity: Option<Opacity>, thickness: Option<u32>) {
        let opacity = opacity.unwrap_or(Opacity::OPAQUE);

        let transform = |image: &mut RgbaImage, width: u32, height: u32| {
            let center = ((width / 2) as i32, (height / 2) as i32);
            let offset = thickness
                .map(|thickness| thickness.min(10) * 8)
                .unwrap_or(12);
            let radius = (width / 2).saturating_sub(offset) as i32;
            draw_filled_circle_mut(image, center, radius, Rgba([0, 0, 0, 0]));
        };

        draw(self, image, opacity, Some(transform))
    }
}
