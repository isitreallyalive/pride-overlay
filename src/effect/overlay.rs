use image::{GenericImage, GenericImageView, Rgba};
use rand::Rng;

use crate::effect::Effect;

pub struct Overlay;

impl Effect for Overlay {
    fn apply_effect(&self, image: &mut image::DynamicImage) {
        // todo: overlay a pride flag

        // for now, we just make it a random colour
        let (width, height) = image.dimensions();
        let mut rng = rand::rng();
        let color = Rgba([
            rng.random_range(0..=255),
            rng.random_range(0..=255),
            rng.random_range(0..=255),
            255,
        ]);

        for x in 0..width {
            for y in 0..height {
                image.put_pixel(x, y, color);
            }
        }
    }
}
