mod flags;
mod opacity;

pub use flags::Flags;
use image::{imageops, DynamicImage, ImageBuffer, Rgba};
use imageproc::{
    drawing::{draw_filled_circle, draw_filled_rect},
    rect::Rect,
};
pub use opacity::Opacity;
use std::ops::Div;

type Image = ImageBuffer<Rgba<u8>, Vec<u8>>;

fn draw(
    image: &mut DynamicImage,
    flag: &[(u8, u8, u8)],
    opacity: Option<Opacity>,
    flag_transform: Option<Box<dyn Fn(Image, u32, u32) -> Image>>,
) -> Image {
    // get image data
    let mut image = image.to_rgba8();
    let (width, height) = image.dimensions();

    // draw the pride flag
    let mut flag_image = ImageBuffer::<Rgba<u8>, Vec<u8>>::new(width, height);
    let segment_count = flag.len();
    let segment_size = (height as f32).div(segment_count as f32).ceil() as u32;
    let a = opacity.unwrap_or(Opacity(127)).0;

    for (i, (r, g, b)) in flag.iter().enumerate() {
        let i = i as u32;
        let rect = Rect::at(0, (i * segment_size) as i32).of_size(width, segment_size);
        let colour = Rgba([*r, *g, *b, a]);

        flag_image = draw_filled_rect(&flag_image, rect, colour);
    }

    // transform the flag if a transform closure has been provided
    if let Some(transform) = flag_transform {
        flag_image = transform(flag_image, width, height);
    }

    // overlay the flag image
    imageops::overlay(&mut image, &flag_image, 0, 0);

    image.clone()
}

/// Overlay a pride flag over an image
pub fn overlay(image: &mut DynamicImage, flag: &[(u8, u8, u8)], opacity: Option<Opacity>) -> Image {
    draw(
        image,
        flag,
        opacity,
        None::<Box<dyn Fn(Image, u32, u32) -> Image>>,
    )
}

/// Overlay a pride flag ring over an image
pub fn circle(image: &mut DynamicImage, flag: &[(u8, u8, u8)], thickness: Option<u8>) -> Image {
    draw(
        image,
        flag,
        Some(Opacity(255)),
        Some(Box::new(move |img, width, height| {
            draw_filled_circle(
                &img,
                ((width / 2) as i32, (height / 2) as i32),
                (width / 2
                    - if let Some(mut thickness) = thickness {
                        if thickness > 10 {
                            thickness = 10;
                        }

                        thickness * 8
                    } else {
                        24
                    } as u32) as i32,
                Rgba([0, 0, 0, 0]),
            )
        })),
    )
}
