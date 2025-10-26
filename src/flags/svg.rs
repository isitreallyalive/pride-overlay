#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

/// How an SVG image should be scaled to fit a given area.
#[derive(Clone, Copy)]
#[cfg_attr(target_arch = "wasm32", derive(Serialize, Deserialize))]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub enum SvgScaleMode {
    /// Fit the entire SVG within the destination bounds while preserving
    /// its aspect ratio.  
    ///  
    /// This mode may leave empty space (letterboxing) if the aspect ratios differ.
    Contain,

    /// Fill the entire destination area while preserving the aspect ratio,
    /// cropping parts of the SVG if necessary.  
    Cover,

    /// Stretch the SVG to exactly fill the destination, ignoring its
    /// intrinsic aspect ratio.  
    ///  
    /// This can cause distortion, but ensures there is no empty space.
    Stretch,

    /// Render the SVG at its intrinsic size without applying any scaling.
    None,
}

/// Get information about an SVG.
pub trait SvgData {
    fn data(&self) -> &[u8];
    fn scale(&self) -> &SvgScaleMode;
}

/// An SVG image.
#[derive(bon::Builder, Clone, Copy)]
#[builder(
    const,
    builder_type(doc {
        /// Builder for the [Svg] struct.
    })
)]
pub struct Svg<'a> {
    /// The raw SVG data.
    #[builder(start_fn)]
    data: &'a [u8],
    /// How the SVG should be scaled when rendered.
    #[builder(default = SvgScaleMode::Contain)]
    scale: SvgScaleMode,
}

impl<'a> super::SvgData for Svg<'a> {
    fn data(&self) -> &[u8] {
        self.data
    }

    fn scale(&self) -> &SvgScaleMode {
        &self.scale
    }
}
