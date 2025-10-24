use std::io::Cursor;

use crate::{
    effects::{Effect as _, Overlay, Ring},
    flags::Flags,
};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn init() {
    // panic to the console
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
pub fn apply(
    #[wasm_bindgen(js_name = "image")] data: &[u8],
    effect: Effect,
    flag: Flags,
) -> Vec<u8> {
    // todo: handle errors
    let format = image::guess_format(data).unwrap();
    let mut image = image::load_from_memory_with_format(data, format).unwrap();
    match effect {
        Effect::Overlay => Overlay::builder(flag.into()).build().apply(&mut image),
        Effect::Ring => Ring::builder(flag.into()).build().apply(&mut image),
    }
    let mut bytes = Vec::new();
    image
        .write_to(&mut Cursor::new(&mut bytes), format)
        .unwrap();
    bytes
}

/// Built-in effects.
#[wasm_bindgen]
pub enum Effect {
    Overlay,
    Ring,
}
