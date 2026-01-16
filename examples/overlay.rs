use pride_overlay::{flags::RAINBOW, prelude::*};

use crate::helpers::run;

mod helpers;

const NAME: &str = "overlay";
const IMAGE: &[u8] = include_bytes!("data/image.webp");
const ANIM: &[u8] = include_bytes!("data/anim.webp");

fn main() -> Result<(), PrideError> {
    run(
        format!("{}-image", NAME).as_str(),
        IMAGE,
        Overlay::builder(RAINBOW).build(),
        ImageFormat::WebP,
    )?;
    run(
        format!("{}-anim", NAME).as_str(),
        ANIM,
        Overlay::builder(RAINBOW).build(),
        ImageFormat::WebP,
    )?;

    Ok(())
}
