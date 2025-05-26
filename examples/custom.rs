use image::ImageResult;
use pride_overlay::prelude::*;

/// https://catppuccin.com/palette
const COLOURS: &[Colour] = &[
    Colour::from_hex(0xED8796).build(),               // red
    Colour::from_hex(0xF5A97F).proportion(2).build(), // peach
    Colour::from_hex(0xEED49F).build(),               // yellow
    Colour::from_hex(0xA6DA95).build(),               // green
    Colour::from_hex(0x7DC4E4).build(),               // sapphire
    Colour::from_hex(0xC6A0F6).build(),               // lavender
];

const fn make_effect() -> Ring<'static> {
    let flag = Flag::builder(COLOURS).build();
    Ring::custom(flag).build()
}

const EFFECT: Ring = make_effect();

fn main() -> ImageResult<()> {
    //! effects can be created at compile or runtime
    #[allow(unused_variables)]
    let effect = make_effect();

    let mut image = image::open("examples/input.webp")?;
    EFFECT.apply(&mut image);
    image.save("examples/out/custom.webp")?;

    Ok(())
}
