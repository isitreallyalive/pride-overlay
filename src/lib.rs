use image::{imageops, DynamicImage, ImageBuffer, Rgba};
use std::ops::Div;

// Opacity helper
#[derive(Debug)]
pub struct Opacity(u8);

impl Opacity {
    pub fn new(opacity: u8) -> Opacity {
        Opacity(opacity)
    }

    pub fn from_percentage(percentage: f32) -> Option<Opacity> {
        if percentage >= 0. && percentage <= 100. {
            return Some(Opacity((2.55 * percentage).floor() as u8));
        }

        None
    }
}

pub(crate) type Flag<const N: usize> = [(u8, u8, u8); N];

/// Built-in flags
#[allow(non_upper_case_globals, non_snake_case)]
pub mod Flags {
    use crate::Flag;

    pub const Rainbow: Flag<6> = [
        (229, 0, 0),
        (255, 141, 0),
        (255, 238, 0),
        (2, 129, 33),
        (0, 76, 255),
        (119, 0, 136),
    ];
    pub const Transgender: Flag<5> = [
        (91, 207, 251),
        (245, 171, 185),
        (255, 255, 255),
        (245, 171, 185),
        (91, 207, 251),
    ];
    pub const Bisexual: Flag<3> = [(214, 2, 112), (155, 79, 150), (0, 56, 168)];
    pub const Pansexual: Flag<3> = [(255, 28, 141), (255, 215, 0), (26, 179, 255)];
    pub const Nonbinary: Flag<4> = [
        (252, 244, 49),
        (252, 252, 252),
        (157, 89, 210),
        (40, 40, 40),
    ];
    pub const Lesbian: Flag<5> = [
        (214, 40, 0),
        (255, 155, 86),
        (255, 255, 255),
        (212, 98, 166),
        (164, 0, 98),
    ];
    pub const Agender: Flag<7> = [
        (0, 0, 0),
        (186, 186, 186),
        (255, 255, 255),
        (186, 244, 132),
        (255, 255, 255),
        (186, 186, 186),
        (0, 0, 0),
    ];
    pub const Asexual: Flag<4> = [(0, 0, 0), (164, 164, 164), (255, 255, 255), (129, 0, 129)];
    pub const Genderqueer: Flag<3> = [(181, 127, 221), (255, 255, 255), (73, 130, 30)];
    pub const Genderfluid: Flag<5> = [
        (254, 118, 162),
        (255, 255, 255),
        (191, 18, 215),
        (0, 0, 0),
        (48, 60, 190),
    ];
    pub const Aromantic: Flag<5> = [
        (59, 167, 64),
        (168, 212, 122),
        (255, 255, 255),
        (171, 171, 171),
        (0, 0, 0),
    ];
    pub const Polyamory: Flag<3> = [(0, 0, 255), (255, 0, 0), (0, 0, 0)];
}

pub fn overlay<const N: usize>(
    image: &mut DynamicImage,
    flag: Flag<N>,
    opacity: Option<Opacity>,
) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    let mut image = image.to_rgba8();
    let (width, height) = image.dimensions();

    // Draw the flag as an image
    let mut flag_image = ImageBuffer::<Rgba<u8>, Vec<u8>>::new(width, height);
    let segment_count = flag.len();
    let segment_size = (height as f32).div(segment_count as f32).ceil() as u32;
    let a = opacity.unwrap_or(Opacity(127)).0;

    for (i, (r, g, b)) in flag.iter().enumerate() {
        let i = i as u32;

        for y in 0..segment_size {
            for x in 0..width {
                flag_image.put_pixel(x, (i * segment_size) + y, Rgba([*r, *g, *b, a]));
            }
        }
    }

    // Blit the flag over the image
    imageops::overlay(&mut image, &flag_image, 0, 0);

    image.clone()
}
