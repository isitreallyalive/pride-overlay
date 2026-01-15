use std::io::Cursor;

use wasm_bindgen::prelude::*;

use crate::{effect::Effect, image::Image as InnerImage, prelude::Overlay};

#[wasm_bindgen(start)]
fn init() {
    console_error_panic_hook::set_once();
}

#[wasm_bindgen(js_name = Effect)]
pub enum WasmEffect {
    Overlay,
}

#[wasm_bindgen]
pub struct Image(InnerImage);

#[wasm_bindgen]
impl Image {
    #[wasm_bindgen(constructor)]
    pub fn new(data: &[u8]) -> Result<Self, JsValue> {
        InnerImage::read(data)
            .map(|img| Self(img))
            .map_err(|e| JsValue::from_str(&e.to_string()))
    }

    pub fn apply(&mut self, effect: WasmEffect) {
        // apply the effect
        let effect: Box<dyn Effect> = Box::new(match effect {
            WasmEffect::Overlay => Overlay,
        });
        effect.apply(&mut self.0);
    }

    pub fn write(self) -> Result<Vec<u8>, JsValue> {
        let mut buf = Cursor::new(Vec::new());
        self.0
            .write(&mut buf)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        Ok(buf.into_inner())
    }
}
