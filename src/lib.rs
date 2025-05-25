#![no_std]

pub use colour::Colour;
pub use flags::Flag;
pub use opacity::Opacity;

use flags::FlagData;
use image::{DynamicImage, GenericImageView, Rgba, RgbaImage, imageops::overlay};
use imageproc::{
    drawing::{draw_filled_circle_mut, draw_filled_rect_mut},
    rect::Rect,
};

mod colour;
mod flags;
mod opacity;

/// Creates a flag overlay image with the specified dimensions and opacity
fn create_flag_overlay(flag: &Flag, width: u32, height: u32, opacity: Opacity) -> RgbaImage {
    let alpha = opacity.get_raw();

    match flag.data() {
        FlagData::Special(data, _) => create_special_flag_overlay(data, width, height, alpha),
        FlagData::Colours(colours) => create_stripe_flag_overlay(colours, width, height, alpha),
    }
}

/// Creates an overlay from special flag image data
fn create_special_flag_overlay(data: &[u8], width: u32, height: u32, alpha: u8) -> RgbaImage {
    let mut flag_image = image::load_from_memory(data)
        .expect("Failed to load flag image data")
        .resize_to_fill(width, height, image::imageops::FilterType::Nearest)
        .to_rgba8();

    // Apply opacity to all pixels
    flag_image.pixels_mut().for_each(|pixel| {
        pixel[3] = alpha;
    });

    flag_image
}

/// Creates a horizontal stripe flag overlay from colors
fn create_stripe_flag_overlay(colours: &[Colour], width: u32, height: u32, alpha: u8) -> RgbaImage {
    let mut flag_image = RgbaImage::new(width, height);
    let stripe_height = height / colours.len() as u32;

    for (i, &Colour { r, g, b }) in colours.iter().enumerate() {
        let y_start = (i as u32 * stripe_height) as i32;
        let rect = Rect::at(0, y_start).of_size(width, stripe_height);
        let colour = Rgba([r, g, b, alpha]);
        draw_filled_rect_mut(&mut flag_image, rect, colour);
    }

    flag_image
}

/// Calculates the ring thickness offset in pixels
fn calculate_ring_offset(thickness: Option<u32>) -> u32 {
    const MAX_THICKNESS: u32 = 10;
    const THICKNESS_MULTIPLIER: u32 = 8;
    const DEFAULT_OFFSET: u32 = 12;

    thickness
        .map(|t| t.min(MAX_THICKNESS) * THICKNESS_MULTIPLIER)
        .unwrap_or(DEFAULT_OFFSET)
}

/// Extracts colors from flag data, regardless of variant
fn extract_flag_colours(flag_data: FlagData) -> &'static [Colour] {
    match flag_data {
        FlagData::Special(_, colours) | FlagData::Colours(colours) => colours,
    }
}

impl Flag {
    /// Overlays the flag onto the given image. The image is modified in place.
    ///
    /// # Arguments
    /// * `image` - The target image to overlay the flag onto
    /// * `opacity` - Controls the transparency of the flag overlay. Uses 50% if [None].
    pub fn overlay(&self, image: &mut DynamicImage, opacity: Option<Opacity>) {
        let (width, height) = image.dimensions();
        let opacity = opacity.unwrap_or_default();
        let flag_overlay = create_flag_overlay(self, width, height, opacity);

        overlay(image, &flag_overlay, 0, 0);
    }

    /// Draws a ring around the image using the flag's colours. The image is modified in place.
    ///
    /// # Arguments
    /// * `image` - The target image to draw the ring on
    /// * `opacity` - Controls the transparency of the ring. Uses 100% if [None].
    /// * `thickness` - Ring thickness (1-10, multiplied by 8). Uses 12 pixels if [None].
    ///
    /// # Ring Calculation
    /// - Offset = min(thickness, 10) * 8 (capped at 80 pixels) or 12 (default)
    /// - Inner radius = (image_width / 2) - offset
    pub fn ring(&self, image: &mut DynamicImage, opacity: Option<Opacity>, thickness: Option<u32>) {
        let (width, height) = image.dimensions();
        let opacity = opacity.unwrap_or(Opacity::OPAQUE);

        // create ring overlay using flag colours
        let colours = extract_flag_colours(self.data());
        let ring_flag = Flag::Custom(colours);
        let mut ring_overlay = create_flag_overlay(&ring_flag, width, height, opacity);

        // cut out the inner circle to create a ring effect
        let center = ((width / 2) as i32, (height / 2) as i32);
        let offset = calculate_ring_offset(thickness);
        let inner_radius = (width / 2).saturating_sub(offset) as i32;

        draw_filled_circle_mut(&mut ring_overlay, center, inner_radius, Rgba([0, 0, 0, 0]));
        overlay(image, &ring_overlay, 0, 0);
    }
}
