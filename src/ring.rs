use crate::{Colour, Flag, Opacity, flags::FlagData, overlay::create_flag_overlay};
use alloc::vec::Vec;
use core::f32::consts::PI;
use image::{DynamicImage, GenericImageView, Rgba, RgbaImage, imageops::overlay};
use imageproc::{drawing::draw_antialiased_polygon_mut, pixelops::interpolate, point::Point};

/// Apply a ring around the image using the flag's colours.
pub fn ring(flag: &Flag, image: &mut DynamicImage, opacity: Opacity, thickness: Option<u32>) {
    let (width, height) = image.dimensions();

    let colours = extract_flag_colours(flag.data());
    let ring_flag = Flag::Custom(colours);
    let mut ring_overlay = create_flag_overlay(&ring_flag, width, height, opacity);

    let center = ((width / 2) as i32, (height / 2) as i32);
    let offset = calculate_ring_offset(thickness);
    let inner_radius = (width / 2).saturating_sub(offset) as i32;

    draw_smooth_circle(&mut ring_overlay, center, inner_radius, Rgba([0, 0, 0, 0]));
    overlay(image, &ring_overlay, 0, 0);
}

/// Extracts the flag colours from the given [FlagData].
fn extract_flag_colours(flag_data: FlagData) -> &'static [Colour] {
    match flag_data {
        FlagData::Special(_, colours) | FlagData::Colours(colours) => colours,
    }
}

/// Calculates the offset for the ring based on the thickness.
fn calculate_ring_offset(thickness: Option<u32>) -> u32 {
    const MAX_THICKNESS: u32 = 10;
    const THICKNESS_MULTIPLIER: u32 = 8;
    const DEFAULT_OFFSET: u32 = 12;

    thickness
        .map(|t| t.min(MAX_THICKNESS) * THICKNESS_MULTIPLIER)
        .unwrap_or(DEFAULT_OFFSET)
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
