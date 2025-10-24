use image::ImageResult;
use pride_overlay::{flags::Flag, prelude::*};

/// https://catppuccin.com/palette
const COLOURS: &[Colour] = &[
    Colour::hex(0xED8796), // red
    Colour::hex(0xF5A97F), // peach
    Colour::hex(0xF5A97F), // peach
    Colour::hex(0xEED49F), // yellow
    Colour::hex(0xA6DA95), // green
    Colour::hex(0x7DC4E4), // sapphire
    Colour::hex(0xC6A0F6), // lavender
];

const fn make_effect() -> Ring<'static> {
    let flag = Flag::builder("Catppuccin", COLOURS).build();
    Ring::builder(flag).build()
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
