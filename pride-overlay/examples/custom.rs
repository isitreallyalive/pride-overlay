use image::ImageResult;
use pride_overlay::prelude::*;

// https://catppuccin.com/palette
// effects can be constant!
const EFFECT: Overlay = Overlay::builder(Flag::Custom(&[
    Colour::from_hex(0xED8796).build(),               // red
    Colour::from_hex(0xF5A97F).proportion(2).build(), // peach
    Colour::from_hex(0xEED49F).build(),               // yellow
    Colour::from_hex(0xA6DA95).build(),               // green
    Colour::from_hex(0x7DC4E4).build(),               // sapphire
    Colour::from_hex(0xC6A0F6).build(),               // lavender
]))
.opacity(Opacity::new(0.8))
.build();

fn main() -> ImageResult<()> {
    let mut image = image::open("pride-overlay/examples/input.webp")?;
    EFFECT.apply(&mut image);
    image.save("pride-overlay/examples/out/custom.webp")?;
    Ok(())
}
