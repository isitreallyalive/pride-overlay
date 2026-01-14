use std::sync::Mutex;

use image::{GenericImage, GenericImageView, Rgba};
use rand::Rng;
use rayon::prelude::*;

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

        let image = Mutex::new(image);
        (0..width).into_par_iter().for_each(|x| {
            (0..height).for_each(|y| {
                let mut image = image.lock().unwrap();
                image.put_pixel(x, y, color);
            });
        });
    }
}
