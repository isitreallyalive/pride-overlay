use pride_overlay::{Result, prelude::*};

use crate::helpers::run;

mod helpers;

const NAME: &str = "overlay";
const WEBP: &[u8] = include_bytes!("data/image.webp");
const WEBP_ANIM: &[u8] = include_bytes!("data/anim.webp");
const PNG: &[u8] = include_bytes!("data/image.png");
const APNG: &[u8] = include_bytes!("data/anim.png");

fn main() -> Result<()> {
    run(
        format!("{}-image", NAME).as_str(),
        WEBP,
        Overlay::builder(flags::RAINBOW).build(),
        ImageFormat::WebP,
    )?;
    run(
        format!("{}-anim", NAME).as_str(),
        WEBP_ANIM,
        Overlay::builder(flags::RAINBOW).build(),
        ImageFormat::WebP,
    )?;
    run(
        format!("{}-image", NAME).as_str(),
        PNG,
        Overlay::builder(flags::RAINBOW).build(),
        ImageFormat::Png,
    )?;
    run(
        format!("{}-anim", NAME).as_str(),
        APNG,
        Overlay::builder(flags::RAINBOW).build(),
        ImageFormat::Png,
    )?;

    Ok(())
}
