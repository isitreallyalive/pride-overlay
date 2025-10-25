use crate::flags::FlagOwned;
#[cfg(target_arch = "wasm32")]
use crate::flags::{wasm::Flag};

mod overlay;
pub use overlay::*;

mod ring;
pub use ring::*;

pub trait Effect {
    fn apply<'a, F: Into<FlagOwned<'a>>>(&self, image: &mut image::DynamicImage, flag: F);
}

/// Apply an effect to an image.
#[cfg(target_arch = "wasm32")]
pub fn apply<E: Effect>(data: &[u8], flag: Flag, effect: E) -> Result<Vec<u8>, image::ImageError> {
    use std::io::Cursor;

    let format = image::guess_format(data)?;
    let mut image = image::load_from_memory_with_format(data, format)?;
    let flag: FlagOwned = flag.into();
    effect.apply(&mut image, flag);
    let mut output: Vec<u8> = Vec::new();
    image.write_to(&mut Cursor::new(&mut output), format)?;
    Ok(output)
}
