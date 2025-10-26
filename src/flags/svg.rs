#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

/// Scaling modes for SVG assets.
#[derive(Clone, Copy, Debug)]
#[cfg_attr(target_arch = "wasm32", derive(Serialize, Deserialize))]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub enum SvgScaleMode {
    /// Fit the whole SVG inside the destination (preserve aspect ratio).
    Contain,
    /// Fill the destination, cropping if necessary (preserve aspect ratio).
    Cover,
    /// Stretch to exactly fill the destination (ignore aspect ratio).
    Stretch,
    /// Place the SVG at its intrinsic size (no scaling).
    None,
}

pub trait SvgData {
    fn data(&self) -> &[u8];
    fn scale(&self) -> SvgScaleMode;
}

#[derive(bon::Builder, Clone, Copy)]
#[builder(const)]
pub struct Svg<'a> {
    /// SVG data.
    #[builder(start_fn)]
    data: &'a [u8],
    /// Scaling mode for the SVG.
    #[builder(default = SvgScaleMode::Contain)]
    scale: SvgScaleMode,
}

impl super::SvgData for Svg<'_> {
    fn data(&self) -> &[u8] {
        self.data
    }

    fn scale(&self) -> SvgScaleMode {
        self.scale
    }
}
