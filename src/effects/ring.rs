use crate::{effects::create_flag_overlay, flags::Flag, prelude::*};
use core::f32::consts::PI;
use image::{GenericImageView, Rgba, RgbaImage, imageops::overlay};
use imageproc::{drawing::draw_antialiased_polygon_mut, pixelops::interpolate, point::Point};

/// Effect that draws a ring around an image using pride [Flag] colors.
#[derive(bon::Builder)]
#[builder(
    const,
    builder_type(doc {
        /// Builder for the [Ring] effect.
    }),
    start_fn(vis = "pub(crate)", name = "_builder")
)]
pub struct Ring<'a> {
    #[builder(start_fn)]
    flag: Flag<'a>,
    #[builder(default = Opacity::OPAQUE)]
    opacity: Opacity,
    #[builder(default = 12)]
    thickness: u32,
}

impl Effect for Ring<'_> {
    fn apply(&self, image: &mut image::DynamicImage) {
        let (width, height) = image.dimensions();

        let ring_flag = Flag::builder("", self.flag.colours).build();
        let mut ring_overlay = create_flag_overlay(&ring_flag, width, height, &self.opacity);

        let center = ((width / 2) as i32, (height / 2) as i32);
        let radius = (width / 2).saturating_sub(self.thickness) as i32;

        draw_circle(&mut ring_overlay, center, radius, Rgba([0, 0, 0, 0]));
        overlay(image, &ring_overlay, 0, 0);
    }
}

impl<'a> Ring<'a> {
    /// Create a new [Ring] [Effect] with a custom [Flag].
    pub const fn builder(flag: Flag<'a>) -> RingBuilder<'a> {
        Self::_builder(flag)
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
