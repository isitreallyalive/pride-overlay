use crate::Colour;

#[cfg(target_arch = "wasm32")]
pub(crate) mod wasm;

mod data;
pub use data::PresetFlag;

mod svg;
pub use svg::{Svg, SvgData, SvgScaleMode};

/// A pride flag.
pub trait FlagData<'a> {
    fn name(&self) -> &str;
    fn colours(&self) -> Box<[Colour]>;
    fn svg(&self) -> Option<Box<dyn SvgData + 'a>>;
}

#[derive(bon::Builder, Clone, Copy, Default)]
#[builder(const)]
pub struct Flag<'a> {
    #[builder(start_fn)]
    /// Name of the flag.
    pub(crate) name: &'a str,
    /// Colours that make up the flag.
    #[builder(start_fn)]
    pub(crate) colours: &'a [Colour],
    pub(crate) svg: Option<svg::Svg<'a>>,
}

impl<'a> FlagData<'a> for Flag<'a> {
    fn name(&self) -> &str {
        self.name
    }

    fn colours(&self) -> Box<[Colour]> {
        self.colours.to_vec().into_boxed_slice()
    }

    fn svg(&self) -> Option<Box<dyn SvgData + 'a>> {
        self.svg
            .clone()
            .map(|svg| Box::new(svg) as Box<dyn SvgData>)
    }
}
