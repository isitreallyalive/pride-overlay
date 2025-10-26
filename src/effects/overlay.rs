use super::Effect;
use crate::Colour;
#[cfg(target_arch = "wasm32")]
use crate::flags::wasm;
use crate::flags::{FlagData, SvgData, SvgScaleMode};
use image::GenericImageView;
use image::{ImageBuffer, Rgba, RgbaImage, imageops::overlay};
use imageproc::{drawing::draw_filled_rect_mut, rect::Rect};
use resvg::{
    tiny_skia::{Pixmap, Transform},
    usvg::{self, Tree},
};
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

const DEFAULT_OPACITY: f32 = 0.5;

/// Effect that overlays a pride [Flag] onto an image.
#[derive(bon::Builder)]
#[builder(
    const,
    builder_type(doc {
        /// Builder for the [Overlay] effect.
    })
)]
pub struct Overlay {
    /// Opacity of the overlay, from 0.0 (transparent) to 1.0 (opaque).
    #[builder(default = DEFAULT_OPACITY, with = |percent: f32| percent.clamp(0., 1.))]
    opacity: f32,
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(js_name = "applyOverlay")]
pub fn apply_overlay(image: &[u8], flag: wasm::Flag, opacity: Option<f32>) -> Vec<u8> {
    let effect = Overlay::builder()
        .opacity(opacity.unwrap_or(DEFAULT_OPACITY))
        .build();
    super::apply(image, flag, effect).unwrap()
}

impl Effect for Overlay {
    fn apply<'a, F>(&self, image: &mut image::DynamicImage, flag: F)
    where
        F: FlagData<'a>,
    {
        if self.opacity == 0. {
            // no-op for zero opacity
        } else {
            let (width, height) = image.dimensions();
            let flag_overlay = overlay_flag(flag, width, height, self.opacity);
            overlay(image, &flag_overlay, 0, 0);
        }
    }
}

pub(crate) fn overlay_flag<'a, F: FlagData<'a>>(
    flag: F,
    width: u32,
    height: u32,
    opacity: f32,
) -> RgbaImage {
    let alpha = (opacity * u8::MAX as f32) as u8;

    match flag.svg() {
        Some(svg) => overlay_svg(svg, width, height, alpha),
        None => overlay_colours(&flag.colours(), width, height, alpha),
    }
}

fn overlay_svg<'a>(svg: Box<dyn SvgData + 'a>, width: u32, height: u32, alpha: u8) -> RgbaImage {
    #[cfg(target_arch = "wasm32")]
    let tree = Tree::from_data(&svg.data(), &usvg::Options::default()).unwrap();
    #[cfg(not(target_arch = "wasm32"))]
    let tree = Tree::from_data(&svg.data(), &usvg::Options::default()).unwrap();
    let mut pixmap = Pixmap::new(width, height).unwrap();

    let size = tree.size();
    let scale_x = width as f32 / size.width();
    let scale_y = height as f32 / size.height();

    let transform = match svg.scale() {
        SvgScaleMode::Contain => {
            let scale = scale_x.min(scale_y);
            Transform::from_scale(scale, scale).post_translate(
                (width as f32 - size.width() * scale) / 2.0,
                (height as f32 - size.height() * scale) / 2.0,
            )
        }
        SvgScaleMode::Cover => {
            let scale = scale_x.max(scale_y);
            Transform::from_scale(scale, scale).post_translate(
                (width as f32 - size.width() * scale) / 2.0,
                (height as f32 - size.height() * scale) / 2.0,
            )
        }
        SvgScaleMode::Stretch => Transform::from_scale(scale_x, scale_y),
        SvgScaleMode::None => Transform::identity(),
    };

    resvg::render(&tree, transform, &mut pixmap.as_mut());

    let mut flag: RgbaImage = ImageBuffer::from_raw(width, height, pixmap.data().to_vec()).unwrap();
    flag.pixels_mut().for_each(|pixel| pixel[3] = alpha);
    flag
}

fn overlay_colours(colours: &[Colour], width: u32, height: u32, alpha: u8) -> RgbaImage {
    let mut flag_image = RgbaImage::new(width, height);
    let count = colours.len() as u32;

    for (i, &Colour(r, g, b)) in colours.iter().enumerate() {
        let i = i as u32;
        let y_start = (i * height) / count;
        let y_end = ((i + 1) * height) / count;

        if y_end > y_start {
            let rect = Rect::at(0, y_start as i32).of_size(width, y_end - y_start);
            let colour = Rgba([r, g, b, alpha]);
            draw_filled_rect_mut(&mut flag_image, rect, colour);
        }
    }

    flag_image
}
