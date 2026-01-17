use std::io;

use wasm_bindgen::prelude::*;
use web_sys::js_sys::{Promise};

use crate::{Result, image::Format};

#[wasm_bindgen(module = "/wasm/dist/index.js")]
extern "C" {
    fn decode();
}

pub struct WebP {}

impl Format for WebP {
    fn read(data: &[u8]) -> Result<Self> {
        decode();
        Ok(Self {})
    }

    fn write<W: io::Write + io::Seek>(self, _buf: &mut W) -> Result<()> {
        Ok(())
    }

    fn apply<A>(&mut self, _apply: A) -> Result<()>
        where
            A: Fn(&mut image::DynamicImage) -> Result<()> {
                Ok(())
    }
}