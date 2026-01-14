use pride_overlay::prelude::*;

const WEBP: &[u8] = include_bytes!("input.webp");
const GIF: &[u8] = include_bytes!("input.gif");

fn main() -> Result<(), PrideError> {
    let webp = Image::read(GIF)?;

    Ok(())
}
