use super::*;
use crate::flags::wasm::Flag;
use std::io::Cursor;
use wasm_bindgen::prelude::*;

fn apply<E: Effect>(data: &[u8], flag: Flag, effect: E) -> Result<Vec<u8>, image::ImageError> {
    let format = image::guess_format(data)?;
    let mut image = image::load_from_memory_with_format(data, format)?;
    effect.apply(&mut image, flag);
    let mut output: Vec<u8> = Vec::new();
    image.write_to(&mut Cursor::new(&mut output), format)?;
    Ok(output)
}

#[wasm_bindgen(js_name = "applyOverlay")]
pub fn apply_overlay(image: &[u8], flag: Flag, opacity: Option<f32>) -> Vec<u8> {
    let effect = Overlay::builder()
        .opacity(opacity.unwrap_or(Overlay::DEFAULT_OPACITY))
        .build();
    apply(image, flag, effect).unwrap()
}

#[wasm_bindgen(js_name = "applyRing")]
pub fn apply_ring(
    image: &[u8],
    flag: Flag,
    opacity: Option<f32>,
    thickness: Option<f32>,
) -> Vec<u8> {
    let effect = Ring::builder()
        .opacity(opacity.unwrap_or(Ring::DEFAULT_OPACITY))
        .thickness(thickness.unwrap_or(Ring::DEFAULT_THICKNESS))
        .build();
    apply(image, flag, effect).unwrap()
}
