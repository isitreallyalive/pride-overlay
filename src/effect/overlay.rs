use crate::{Colour, Effect, Flag, Opacity, ScaleMode, flags::Svg};
use image::GenericImageView;
use image::{ImageBuffer, Rgba, RgbaImage, imageops::overlay};
use imageproc::{drawing::draw_filled_rect_mut, rect::Rect};
use resvg::{
    tiny_skia::{Pixmap, Transform},
    usvg::{self, Tree},
};

/// Overlay the given flag on an image.
#[derive(Builder)]
#[builder(const, start_fn(vis = "pub(crate)"))]
pub struct Overlay<'a> {
    #[builder(start_fn)]
    flag: Flag<'a>,
    #[builder(default = Opacity::HALF)]
    opacity: Opacity,
}

impl Effect for Overlay<'_> {
    fn apply(&self, image: &mut image::DynamicImage) {
        let (width, height) = image.dimensions();
        let flag_overlay = create_flag_overlay(&self.flag, width, height, &self.opacity);
        overlay(image, &flag_overlay, 0, 0);
    }
}

impl Overlay<'static> {
    /// Create a new [Overlay] [Effect] with a builtin [PrideFlag].
    pub const fn new(flag: crate::PrideFlag) -> OverlayBuilder<'static> {
        Self::builder(flag.data())
    }

    /// Create a new [Overlay] [Effect] with a custom [Flag].
    pub const fn custom(flag: crate::Flag<'static>) -> OverlayBuilder<'static> {
        Self::builder(flag)
    }
}

pub(crate) fn create_flag_overlay(
    flag: &Flag<'_>,
    width: u32,
    height: u32,
    opacity: &Opacity,
) -> RgbaImage {
    let alpha = opacity.get_raw();

    if let Some(svg) = &flag.svg {
        create_svg_flag_overlay(svg, width, height, alpha)
    } else {
        create_stripe_flag_overlay(flag.colours, width, height, alpha)
    }
}

fn create_svg_flag_overlay(svg: &Svg, width: u32, height: u32, alpha: u8) -> RgbaImage {
    let tree = Tree::from_data(svg.data, &usvg::Options::default()).unwrap();
    let mut pixmap = Pixmap::new(width, height).unwrap();

    let svg_size = tree.size();
    let scale_x = width as f32 / svg_size.width();
    let scale_y = height as f32 / svg_size.height();

    let transform = match svg.scale {
        ScaleMode::Fill => {
            let scale = scale_x.max(scale_y);
            Transform::from_scale(scale, scale).post_translate(
                (width as f32 - svg_size.width() * scale) / 2.0,
                (height as f32 - svg_size.height() * scale) / 2.0,
            )
        }
        ScaleMode::Stretch => Transform::from_scale(scale_x, scale_y),
    };

    resvg::render(&tree, transform, &mut pixmap.as_mut());

    let mut flag: RgbaImage = ImageBuffer::from_raw(width, height, pixmap.data().to_vec()).unwrap();
    flag.pixels_mut().for_each(|pixel| pixel[3] = alpha);
    flag
}

fn create_stripe_flag_overlay(
    colours: &[crate::Colour],
    width: u32,
    height: u32,
    alpha: u8,
) -> RgbaImage {
    let mut flag_image = RgbaImage::new(width, height);

    let total_proportion: f32 = colours.iter().map(|c| c.proportion as f32).sum();
    let mut y_offset = 0.0;

    for &Colour {
        r,
        g,
        b,
        proportion,
    } in colours
    {
        let stripe_height = (proportion as f32 / total_proportion) * height as f32;
        let y_start = y_offset as u32;
        let y_end = (y_offset + stripe_height as f32).min(height as f32) as u32;

        if y_end > y_start {
            let rect = Rect::at(0, y_start as i32).of_size(width, y_end - y_start);
            let colour = Rgba([r, g, b, alpha]);
            draw_filled_rect_mut(&mut flag_image, rect, colour);
        }

        y_offset += stripe_height as f32;
    }

    flag_image
}
