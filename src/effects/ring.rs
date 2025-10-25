use crate::flags::FlagOwned;
use crate::{effects::overlay_flag, prelude::*};
use core::f32::consts::PI;
use image::{GenericImageView, Rgba, RgbaImage, imageops::overlay};
use imageproc::{drawing::draw_antialiased_polygon_mut, pixelops::interpolate, point::Point};
#[cfg(target_arch = "wasm32")]
use std::marker::PhantomData;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

const DEFAULT_OPACITY: f32 = 1.;
const DEFAULT_THICKNESS: f32 = 0.1;

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
    #[builder(default = DEFAULT_OPACITY, with = |percent: f32| percent.clamp(0., 1.))]
    opacity: f32,
    /// Thickness of the ring as a percentage of the image width, from 0.0 to 1.0.
    ///
    /// You probably want this to be fairly small!
    #[builder(default = DEFAULT_THICKNESS, with = |percent: f32| percent.clamp(0., 1.))]
    thickness: f32,
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(js_name = "applyRing")]
pub fn apply_ring(
    image: &[u8],
    flag: crate::flags::wasm::Flag,
    opacity: Option<f32>,
    thickness: Option<f32>,
) -> Vec<u8> {
    let effect = Ring::builder()
        .opacity(opacity.unwrap_or(DEFAULT_OPACITY))
        .thickness(thickness.unwrap_or(DEFAULT_THICKNESS))
        .build();
    super::apply(image, flag, effect).unwrap()
}

impl Effect for Ring {
    fn apply<'a, F>(&self, image: &mut image::DynamicImage, flag: F)
    where
        F: Into<FlagOwned<'a>>,
    {
        if self.opacity == 0. {
            // no-op for zero opacity
        } else if self.thickness == 1. {
            // full thickness is just an overlay
            let effect = Overlay::builder().opacity(self.opacity).build();
            effect.apply(image, flag)
        } else {
            let flag = flag.into();
            let (width, height) = image.dimensions();
            #[cfg(not(target_arch = "wasm32"))]
            let ring_flag = FlagOwned {
                name: flag.name,
                colours: flag.colours,
                svg: None,
            };
            #[cfg(target_arch = "wasm32")]
            let ring_flag = FlagOwned {
                colours: flag.colours,
                svg: None,
                _marker: PhantomData,
            };
            let mut ring_overlay = overlay_flag(ring_flag, width, height, self.opacity);

            let center = ((width / 2) as i32, (height / 2) as i32);
            let radius =
                (width / 2).saturating_sub(((width / 2) as f32 * self.thickness) as u32) as i32;

            draw_circle(&mut ring_overlay, center, radius, Rgba([0, 0, 0, 0]));
            overlay(image, &ring_overlay, 0, 0);
        }
    }
}

/// Draws a smooth circle on the image using anti-aliasing.
fn draw_circle(image: &mut RgbaImage, center: (i32, i32), radius: i32, color: Rgba<u8>) {
    const MIN_SIDES: f32 = 32.;
    const MAX_SIDES: f32 = 256.;
    const PIXELS_PER_SIDE: f32 = 4.;

    // determine the number of sides to use for a the polygon
    // that approximates the circle.
    // circumference = 2 * pi * radius
    let circumference = 2.0 * PI * radius as f32;
    let sides = (circumference / PIXELS_PER_SIDE).clamp(MIN_SIDES, MAX_SIDES) as usize;

    // compute the points of the polygon
    let points: Vec<Point<i32>> = (0..sides)
        .map(|i| {
            let theta = 2.0 * PI * (i as f32) / (sides as f32);
            let (x, y) = center;
            let dx = radius as f32 * theta.cos();
            let dy = radius as f32 * theta.sin();
            Point::new(x + dx.round() as i32, y + dy.round() as i32)
        })
        .collect();

    draw_antialiased_polygon_mut(image, &points, color, interpolate);
}
