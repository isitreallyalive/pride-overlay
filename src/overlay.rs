use crate::flags::FlagData;
use crate::{Flag, Opacity};
use image::GenericImageView;
use image::{DynamicImage, ImageBuffer, Rgba, RgbaImage, imageops::overlay};
use imageproc::{drawing::draw_filled_rect_mut, rect::Rect};
use resvg::{
    tiny_skia::{Pixmap, Transform},
    usvg::{self, Tree},
};

pub fn apply(flag: &Flag, image: &mut DynamicImage, opacity: Opacity) {
    let (width, height) = image.dimensions();
    let flag_overlay = create_flag_overlay(flag, width, height, opacity);
    overlay(image, &flag_overlay, 0, 0);
}

pub(crate) fn create_flag_overlay(
    flag: &Flag,
    width: u32,
    height: u32,
    opacity: Opacity,
) -> RgbaImage {
    let alpha = opacity.get_raw();

    match flag.data() {
        FlagData::Special(data, _) => create_special_flag_overlay(data, width, height, alpha),
        FlagData::Colours(colours) => create_stripe_flag_overlay(colours, width, height, alpha),
    }
}

fn create_special_flag_overlay(data: &[u8], width: u32, height: u32, alpha: u8) -> RgbaImage {
    let tree = Tree::from_data(data, &usvg::Options::default()).unwrap();
    let mut pixmap = Pixmap::new(width, height).unwrap();

    let svg_size = tree.size();
    let scale_x = width as f32 / svg_size.width();
    let scale_y = height as f32 / svg_size.height();
    let scale = scale_x.max(scale_y);
    let transform = Transform::from_scale(scale, scale).post_translate(
        (width as f32 - svg_size.width() * scale) / 2.0,
        (height as f32 - svg_size.height() * scale) / 2.0,
    );

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
    let num_stripes = colours.len() as u32;

    for (i, &crate::Colour { r, g, b }) in colours.iter().enumerate() {
        let y_start = (i as u32 * height) / num_stripes;
        let y_end = ((i as u32 + 1) * height) / num_stripes;
        let stripe_height = y_end - y_start;

        let rect = Rect::at(0, y_start as i32).of_size(width, stripe_height);
        let colour = Rgba([r, g, b, alpha]);
        draw_filled_rect_mut(&mut flag_image, rect, colour);
    }

    flag_image
}
