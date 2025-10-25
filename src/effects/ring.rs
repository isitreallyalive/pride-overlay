use crate::{effects::create_flag_overlay, flags::Flag, prelude::*};
use core::f32::consts::PI;
use image::{GenericImageView, Rgba, RgbaImage, imageops::overlay};
use imageproc::{drawing::draw_antialiased_polygon_mut, pixelops::interpolate, point::Point};
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

const DEFAULT_OPACITY: Opacity = Opacity::OPAQUE;
const DEFAULT_THICKNESS: u32 = 12;

/// Effect that draws a ring around an image using pride [Flag] colors.
#[derive(bon::Builder)]
#[builder(
    const,
    builder_type(doc {
        /// Builder for the [Ring] effect.
    })
)]
pub struct Ring {
    #[builder(default = DEFAULT_OPACITY)]
    opacity: Opacity,
    #[builder(default = DEFAULT_THICKNESS)]
    thickness: u32,
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(js_name = "applyRing")]
pub fn apply_ring(image: &[u8], flag: Flags, opacity: Option<f32>, thickness: Option<i32>) -> Vec<u8> {
    let effect = Ring::builder()
        .opacity(opacity.map(Opacity::new).unwrap_or(DEFAULT_OPACITY))
        .thickness(thickness.map(|t| t.max(0) as u32).unwrap_or(DEFAULT_THICKNESS))
        .build();
    super::apply(image, flag, effect).unwrap()
}

impl Effect for Ring {
    fn apply(&self, image: &mut image::DynamicImage, flag: Flag) {
        let (width, height) = image.dimensions();
        let ring_flag = Flag::builder("", flag.colours).build();
        let mut ring_overlay = create_flag_overlay(&ring_flag, width, height, &self.opacity);

        let center = ((width / 2) as i32, (height / 2) as i32);
        let radius = (width / 2).saturating_sub(self.thickness) as i32;

        draw_circle(&mut ring_overlay, center, radius, Rgba([0, 0, 0, 0]));
        overlay(image, &ring_overlay, 0, 0);
    }
}

/// Draws a smooth circle on the image using anti-aliasing.
fn draw_circle(image: &mut RgbaImage, center: (i32, i32), radius: i32, color: Rgba<u8>) {
    const MIN_SIDES: f32 = 32.;
    const MAX_SIZES: f32 = 256.;
    const PIXELS_PER_SIDE: f32 = 4.;

    // determine the number of sides to use for a the polygon
    // that approximates the circle.
    // circumference = 2 * pi * radius
    let circumference = 2.0 * PI * radius as f32;
    let sides = (circumference / PIXELS_PER_SIDE).clamp(MIN_SIDES, MAX_SIZES) as usize;

    // compute the points of the polygon
    let mut points = Vec::with_capacity(sides);
    let mut angle = 0.;

    for _ in 0..sides {
        // angle_{i + 1} = angle_{i} + (2 * pi / sides)
        angle += 2.0 * PI / (sides as f32);

        // calculate the x and y coordinates of the point
        let (x, y) = center;
        let (dx, dy) = (radius as f32 * angle.cos(), radius as f32 * angle.sin());
        points.push(Point::new(x + dx as i32, y + dy as i32));
    }

    draw_antialiased_polygon_mut(image, &points, color, interpolate);
}
