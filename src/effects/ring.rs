#[cfg(not(target_arch = "wasm32"))]
use crate::flags::Flag;
#[cfg(target_arch = "wasm32")]
use crate::flags::wasm::CustomFlag;
use crate::{effects::overlay_flag, prelude::*};
use core::f32::consts::PI;
use image::{GenericImageView, Rgba, RgbaImage, imageops::overlay};
use imageproc::{drawing::draw_antialiased_polygon_mut, pixelops::interpolate, point::Point};

/// Effect that draws a ring around an image using pride [Flag] colors.
#[derive(bon::Builder)]
#[builder(
    const,
    builder_type(doc {
        /// Builder for the [Ring] effect.
    })
)]
pub struct Ring {
    /// Opacity of the ring, from 0.0 (transparent) to 1.0 (opaque).
    #[builder(default = Ring::DEFAULT_OPACITY, with = |percent: f32| percent.clamp(0., 1.))]
    opacity: f32,
    /// Thickness of the ring as a percentage of the image width, from 0.0 to 1.0.
    ///
    /// You probably want this to be fairly small!
    #[builder(default = Ring::DEFAULT_THICKNESS, with = |percent: f32| percent.clamp(0., 1.))]
    thickness: f32,
}

impl Ring {
    /// Default opacity for the [Ring] effect.
    pub const DEFAULT_OPACITY: f32 = 1.;

    /// Default thickness for the [Ring] effect.
    pub const DEFAULT_THICKNESS: f32 = 0.1;
}

impl Effect for Ring {
    fn apply<F>(&self, image: &mut image::DynamicImage, flag: F)
    where
        F: FlagData,
    {
        if self.opacity == 0. {
            // no-op for zero opacity
        } else if self.thickness >= 0.99 {
            // full thickness is just an overlay
            let effect = Overlay::builder().opacity(self.opacity).build();
            effect.apply(image, flag)
        } else {
            let (width, height) = image.dimensions();
            #[cfg(not(target_arch = "wasm32"))]
            let ring_flag = Flag {
                name: flag.name(),
                colours: flag.colours(),
                ..Default::default()
            };
            #[cfg(target_arch = "wasm32")]
            let ring_flag = CustomFlag {
                colours: flag.colours().into(),
                ..Default::default()
            };
            let mut ring_overlay = overlay_flag(ring_flag, width, height, self.opacity);

            let center = ((width / 2) as i32, (height / 2) as i32);
            let radius =
                (width / 2).saturating_sub(((width / 2) as f32 * self.thickness) as u32) as i32;

            draw_circle(&mut ring_overlay, center, radius as f32, Rgba([0, 0, 0, 0]));
            overlay(image, &ring_overlay, 0, 0);
        }
    }
}

/// Draws a smooth circle on the image using anti-aliasing.
fn draw_circle(image: &mut RgbaImage, center: (i32, i32), radius: f32, color: Rgba<u8>) {
    const MIN_SIDES: f32 = 32.;
    const MAX_SIDES: f32 = 256.;
    const PIXELS_PER_SIDE: f32 = 4.;

    // determine the number of sides to use for a the polygon
    // that approximates the circle.
    let circumference = 2.0 * PI * radius;
    let sides = (circumference / PIXELS_PER_SIDE).clamp(MIN_SIDES, MAX_SIDES);

    // compute the points of the polygon
    let points: Vec<Point<i32>> = (0..(sides as usize))
        .map(|i| {
            let theta = 2.0 * PI * (i as f32) / sides;
            let (x, y) = center;
            let dx = radius * theta.cos();
            let dy = radius * theta.sin();
            Point::new(x + dx.round() as i32, y + dy.round() as i32)
        })
        .collect();

    draw_antialiased_polygon_mut(image, &points, color, interpolate);
}
