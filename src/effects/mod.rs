use crate::flags::Flag;
#[cfg(target_arch = "wasm32")]
use crate::flags::Flags;

mod overlay;
pub use overlay::*;

mod ring;
pub use ring::*;

pub trait Effect {
    fn apply(&self, image: &mut image::DynamicImage, flag: Flag);
}

/// Apply an effect to an image.
#[cfg(target_arch = "wasm32")]
pub fn apply<E: Effect>(
    data: &[u8],
    flag: Flags,
    effect: E,
) -> Result<Vec<u8>, image::ImageError> {
    use std::io::Cursor;

    let format = image::guess_format(data)?;
    let mut image = image::load_from_memory_with_format(data, format)?;
    effect.apply(&mut image, flag.into());
    let mut output: Vec<u8> = Vec::new();
    image.write_to(&mut Cursor::new(&mut output), format)?;
    Ok(output)
}
