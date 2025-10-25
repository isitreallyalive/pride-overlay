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

/// An SVG asset to be used when rendering a pride flag.
#[derive(Clone, Copy)]
pub struct SvgAsset<'a> {
    pub(crate) data: &'a [u8],
    pub(crate) scale: SvgScaleMode,
}

impl<'a> SvgAsset<'a> {
    /// Create a new [SvgAsset] from raw SVG data and a [SvgScaleMode].
    pub const fn new(data: &'a [u8], scale: SvgScaleMode) -> Self {
        SvgAsset { data, scale }
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub(crate) type SvgAssetOwned<'a> = SvgAsset<'a>;
