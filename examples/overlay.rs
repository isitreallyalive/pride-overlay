use pride_overlay::{flags::RAINBOW, prelude::*};

use crate::helpers::run;

mod helpers;

const NAME: &str = "overlay";
const WEBP: &[u8] = include_bytes!("data/input.webp");
const WEBP_ANIM: &[u8] = include_bytes!("data/input-anim.webp");
const GIF: &[u8] = include_bytes!("data/input.gif");

fn main() -> Result<(), PrideError> {
    run(
        NAME,
        WEBP,
        Overlay::builder(RAINBOW).build(),
        ImageFormat::WebP,
    )?;
    run(
        "overlay-anim",
        WEBP_ANIM,
        Overlay::builder(RAINBOW).build(),
        ImageFormat::WebP,
    )?;
    run(
        NAME,
        GIF,
        Overlay::builder(RAINBOW).build(),
        ImageFormat::Gif,
    )?;

    Ok(())
}
