use super::{Flag as NativeFlag, FlagData};
use crate::{Colour, flags::PresetFlag};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(typescript_custom_section)]
const HEX: &'static str = r#"
type Hex = `#${string}`;
"#;

#[derive(Serialize, Deserialize, Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi)]
#[serde(untagged)]
pub enum Flag {
    Preset(PresetFlag),
    Custom(CustomFlag),
}

impl FlagData<'static> for Flag {
    fn name(&self) -> &str {
        match self {
            Flag::Preset(flag) => {
                let flag: NativeFlag = flag.into();
                flag.name
            }
            Flag::Custom(flag) => flag.name(),
        }
    }

    fn colours(&self) -> Box<[Colour]> {
        match self {
            Flag::Preset(flag) => {
                let flag: NativeFlag = flag.into();
                flag.colours()
            }
            Flag::Custom(flag) => flag.colours(),
        }
    }

    fn svg(&self) -> Option<Box<dyn super::SvgData>> {
        match self {
            Flag::Preset(flag) => {
                let flag: NativeFlag = flag.into();
                flag.svg()
            }
            Flag::Custom(flag) => flag.svg(),
        }
    }
}

#[derive(Default, Serialize, Deserialize, Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct CustomFlag {
    pub(crate) name: String,
    #[tsify(type = "Hex[]")]
    pub(crate) colours: Box<[Colour]>,
    #[tsify(optional)]
    pub(crate) svg: Option<Svg>,
}

impl FlagData<'static> for CustomFlag {
    fn name(&self) -> &str {
        &self.name
    }

    fn colours(&self) -> Box<[Colour]> {
        self.colours.clone()
    }

    fn svg(&self) -> Option<Box<dyn super::SvgData>> {
        self.svg
            .as_ref()
            .map(|svg| Box::new(svg.clone()) as Box<dyn super::SvgData>)
    }
}

#[derive(Clone, Serialize, Deserialize, Tsify)]
pub struct Svg {
    data: Box<[u8]>,
    scale: super::SvgScaleMode,
}

impl super::SvgData for Svg {
    fn data(&self) -> &[u8] {
        &self.data
    }

    fn scale(&self) -> super::SvgScaleMode {
        self.scale
    }
}
