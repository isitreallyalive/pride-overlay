use crate::Colour;

#[cfg(target_arch = "wasm32")]
pub(crate) mod wasm;

mod data;
pub use data::PresetFlag;

mod svg;
pub use svg::{Svg, SvgData, SvgScaleMode};

/// Get information about a pride flag.
pub trait FlagData {
    fn name(&self) -> &str;
    fn colours(&self) -> &[Colour];
    fn svg(&self) -> Option<&dyn SvgData>;
}

/// A pride flag.
#[derive(bon::Builder, Clone, Copy, Default)]
#[builder(
    const,
    builder_type(doc {
        /// Builder for the [Flag] struct.
    })
)]
pub struct Flag<'a> {
    #[builder(start_fn)]
    /// Name of the flag.
    pub(crate) name: &'a str,
    /// Colours that make up the flag.
    #[builder(start_fn)]
    pub(crate) colours: &'a [Colour],
    /// Optional SVG representation of the flag.
    pub(crate) svg: Option<svg::Svg<'a>>,
}

impl<'a> FlagData for Flag<'a> {
    fn name(&self) -> &str {
        self.name
    }

    fn colours(&self) -> &[Colour] {
        self.colours
    }

    fn svg(&self) -> Option<&dyn SvgData> {
        self.svg.as_ref().map(|svg| svg as &dyn SvgData)
    }
}
