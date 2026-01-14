use std::path::PathBuf;

use pride_overlay::prelude::*;

const WEBP: &[u8] = include_bytes!("input.webp");
const GIF: &[u8] = include_bytes!("input.gif");

const OVERLAY: Overlay = Overlay;

fn main() -> Result<(), PrideError> {
    let mut gif = Image::read(GIF)?;
    OVERLAY.apply(&mut gif);

    let examples = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("examples");
    let gif_out = gif.to_bytes(ImageFormat::Gif)?;
    std::fs::write(examples.join("out.gif"), gif_out).unwrap();

    Ok(())
}
