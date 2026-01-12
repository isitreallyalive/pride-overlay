use image::ImageResult;
use pride_overlay::{flags::Flag, prelude::*};

use crate::helpers::run;

mod helpers;

/// https://catppuccin.com/palette
const CATPPUCCIN: Flag = Flag::builder(
    "Catppuccin",
    &[
        Colour::hex(0xED8796), // red
        Colour::hex(0xF5A97F), // peach
        Colour::hex(0xF5A97F), // peach
        Colour::hex(0xEED49F), // yellow
        Colour::hex(0xA6DA95), // green
        Colour::hex(0x7DC4E4), // sapphire
        Colour::hex(0xC6A0F6), // lavender
    ],
)
.build();

const EFFECT: Ring = Ring::builder().opacity(0.8).build();

fn main() -> ImageResult<()> {
    run("input.webp", EFFECT, CATPPUCCIN, "custom.webp")?;
    run("input.gif", EFFECT, CATPPUCCIN, "custom.gif")
}
