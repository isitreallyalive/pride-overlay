use pride_overlay::prelude::*;

use crate::helpers::run;

mod helpers;

const NAME: &str = "overlay";
const WEBP: &[u8] = include_bytes!("data/input.webp");
const GIF: &[u8] = include_bytes!("data/input.gif");

fn main() -> Result<(), PrideError> {
    run(NAME, WEBP, Overlay, ImageFormat::WebP)?;
    run(NAME, GIF, Overlay, ImageFormat::Gif)?;

    Ok(())
}
