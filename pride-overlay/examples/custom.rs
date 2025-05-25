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

const CUSTOM_FLAG: FlagData = FlagData::builder(COLOURS).build();

fn main() -> ImageResult<()> {
    // todo: FIX
    let colours = COLOURS.iter().map(|c| c.to_owned()).collect::<Vec<_>>();
    let custom_flag =
    let effect = Ring::builder(custom_flag);

    println!("{:?}",);
    Ok(())
}
