use std::io;

use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::{ImageData, js_sys::{Promise, Uint8Array}};

use crate::{Result, image::Format};

#[wasm_bindgen(module = "/wasm/dist/index.js")]
extern "C" {
    fn decode(data: Uint8Array) -> Promise;
}

pub struct WebP {
    data: Vec<u8>,
}

#[maybe_async::maybe_async(?Send)]
impl Format for WebP {
    async fn read(data: &[u8]) -> Result<Self> {
        let image = {
            let promise = decode(Uint8Array::from(data));
            JsFuture::from(promise)
                .await
                .map(JsValue::dyn_into::<ImageData>)
                .flatten()
                .unwrap()
        };
        web_sys::console::log_1(&image.data().to_vec().into());
        Ok(WebP { data: image.data().to_vec() })
    }

    fn write<W: io::Write + io::Seek>(self, buf: &mut W) -> Result<()> {
        buf.write_all(&self.data)?;
        Ok(())
    }

    fn apply<A>(&mut self, _apply: A) -> Result<()>
    where
        A: Fn(&mut image::DynamicImage) -> Result<()>,
    {
        Ok(())
    }
}
