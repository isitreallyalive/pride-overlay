use crate::{Colour, Effect, Flag, Opacity, flags::FlagData, overlay::create_flag_overlay};
use alloc::vec::Vec;
use core::f32::consts::PI;
use image::{GenericImageView, Rgba, RgbaImage, imageops::overlay};
use imageproc::{drawing::draw_antialiased_polygon_mut, pixelops::interpolate, point::Point};

#[derive(Builder)]
#[builder(const)]
pub struct Ring {
    #[builder(start_fn)]
    flag: Flag,
    #[builder(default = Opacity::HALF)]
    opacity: Opacity,
    #[builder(default = 12)]
    thickness: u32,
}

impl Effect for Ring {
    fn apply(&self, image: &mut image::DynamicImage) {
        let (width, height) = image.dimensions();

        let colours = extract_flag_colours(self.flag.data());
        let ring_flag = Flag::Custom(colours);
        let mut ring_overlay = create_flag_overlay(ring_flag, width, height, self.opacity);

        let center = ((width / 2) as i32, (height / 2) as i32);
        let offset = self.thickness.min(10) * 8;
        let inner_radius = (width / 2).saturating_sub(offset) as i32;

        draw_smooth_circle(&mut ring_overlay, center, inner_radius, Rgba([0, 0, 0, 0]));
        overlay(image, &ring_overlay, 0, 0);
    }
}

/// Extracts the flag colours from the given [FlagData].
fn extract_flag_colours(flag_data: FlagData) -> &'static [Colour] {
    match flag_data {
        FlagData::Special(_, colours) | FlagData::Colours(colours) => colours,
    }
}

/// Draws a smooth circle on the image using anti-aliasing.
fn draw_smooth_circle(image: &mut RgbaImage, center: (i32, i32), radius: i32, color: Rgba<u8>) {
    const MIN_SIDES: f32 = 32.;
    const MAX_SIZES: f32 = 256.;
    const PIXELS_PER_SIDE: f32 = 4.;

    // determine the number of sides to use for a the polygon
    // that approximates the circle.
    // circumference = 2 * pi * radius
    let circumference = 2.0 * PI * radius as f32;
    let sides = (circumference / PIXELS_PER_SIDE)
        .max(MIN_SIDES)
        .min(MAX_SIZES) as usize;

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
