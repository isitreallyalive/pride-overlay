use crate::prelude::*;
use image::GenericImageView;
use image::{ImageBuffer, Rgba, RgbaImage, imageops::overlay};
use imageproc::{drawing::draw_filled_rect_mut, rect::Rect};
use resvg::{
    tiny_skia::{Pixmap, Transform},
    usvg::{self, Tree},
};

/// Effect that overlays a pride [Flag] onto an image.
#[derive(Builder)]
#[builder(
    const,
    builder_type(doc {
        /// Builder for the [Overlay] effect.
    }),
    start_fn(vis = "pub(crate)", name = "_builder")
)]
pub struct Overlay<'a> {
    #[builder(start_fn)]
    flag: Flag<'a>,
    #[builder(default = Opacity::HALF)]
    opacity: Opacity,
}

impl<'a> Effect for Overlay<'a> {
    fn apply(&self, image: &mut image::DynamicImage) {
        let (width, height) = image.dimensions();
        let flag_overlay = create_flag_overlay(&self.flag, width, height, &self.opacity);
        overlay(image, &flag_overlay, 0, 0);
    }
}

impl<'a> Overlay<'a> {
    /// Create a new [Overlay] [Effect] with a [Flag].
    pub const fn builder(flag: Flag<'a>) -> OverlayBuilder<'a> {
        Self::_builder(flag)
    }
}

pub(crate) fn create_flag_overlay<'a>(
    flag: &Flag<'a>,
    width: u32,
    height: u32,
    opacity: &Opacity,
) -> RgbaImage {
    let alpha = opacity.get_raw();

    match flag.svg {
        Some(svg) => overlay_svg(svg, width, height, alpha),
        None => overlay_colours(flag.colours, width, height, alpha),
    }
}

fn overlay_svg(
    SvgAsset { data, mode, .. }: SvgAsset,
    width: u32,
    height: u32,
    alpha: u8,
) -> RgbaImage {
    let tree = Tree::from_data(data, &usvg::Options::default()).unwrap();
    let mut pixmap = Pixmap::new(width, height).unwrap();

    let size = tree.size();
    let scale_x = width as f32 / size.width();
    let scale_y = height as f32 / size.height();

    let transform = match mode {
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
