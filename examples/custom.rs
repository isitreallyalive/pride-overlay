use image::ImageResult;
use pride_overlay::{Colour, Flag};

/// https://catppuccin.com/palette
const CATPPUCCIN: &[Colour] = &[
    Colour::from_hex(0xED8796), // red
    Colour::from_hex(0xF5A97F), // peach
    Colour::from_hex(0xEED49F), // yellow
    Colour::from_hex(0xA6DA95), // green
    Colour::from_hex(0x7DC4E4), // sapphire
    Colour::from_hex(0xC6A0F6), // lavender
];

fn main() -> ImageResult<()> {
    let mut image = image::open("examples/input.webp")?;
    Flag::Custom(CATPPUCCIN).overlay(&mut image, None);
    image.save("examples/out/custom.webp")?;
    Ok(())
}
