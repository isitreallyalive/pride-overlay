use super::{FlagData, PresetFlag, SvgData};
use crate::Colour;
use wasm_bindgen::prelude::*;

/// Type alias for hexadecimal colour literals in TypeScript bindings.
#[wasm_bindgen(typescript_custom_section)]
const HEX: &'static str = r#"
type Hex = `#${string}`;
"#;

#[derive(Serialize, Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi)]
#[serde(untagged)]
pub enum Flag {
    Preset(PresetFlag),
    Custom(CustomFlag),
}

impl<'de> serde::Deserialize<'de> for Flag {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct FlagVisitor;

        impl<'de> serde::de::Visitor<'de> for FlagVisitor {
            type Value = Flag;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a preset flag index or name")
            }

            fn visit_u8<E>(self, num: u8) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                let max = PresetFlag::max_discriminant();
                if num <= max {
                    Ok(Flag::Preset(unsafe {
                        std::mem::transmute::<u8, PresetFlag>(num)
                    }))
                } else {
                    Err(E::custom("invalid preset index"))
                }
            }

            fn visit_u64<E>(self, num: u64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                if num <= u8::MAX as u64 {
                    self.visit_u8(num as u8)
                } else {
                    Err(E::custom("invalid preset index"))
                }
            }

            fn visit_i64<E>(self, num: i64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                if num >= 0 && num <= u8::MAX as i64 {
                    self.visit_u8(num as u8)
                } else {
                    Err(E::custom("invalid preset index"))
                }
            }

            fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                s.parse::<PresetFlag>()
                    .map(Flag::Preset)
                    .map_err(|_| E::custom("invalid preset name"))
            }
        }

        deserializer.deserialize_any(FlagVisitor)
    }
}

impl FlagData for Flag {
    fn name(&self) -> &str {
        match self {
            Flag::Preset(flag) => flag.name(),
            Flag::Custom(flag) => flag.name(),
        }
    }

    fn colours(&self) -> &[Colour] {
        match self {
            Flag::Preset(flag) => flag.colours(),
            Flag::Custom(flag) => flag.colours(),
        }
    }

    fn svg(&self) -> Option<&dyn SvgData> {
        match self {
            Flag::Preset(flag) => flag.svg(),
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

impl FlagData for CustomFlag {
    fn name(&self) -> &str {
        &self.name
    }

    fn colours(&self) -> &[Colour] {
        self.colours.as_ref()
    }

    fn svg(&self) -> Option<&dyn SvgData> {
        self.svg.as_ref().map(|svg| svg as &dyn SvgData)
    }
}

#[derive(Serialize, Deserialize, Tsify)]
pub struct Svg {
    data: Box<[u8]>,
    scale: super::SvgScaleMode,
}

impl SvgData for Svg {
    fn data(&self) -> &[u8] {
        &self.data
    }

    fn scale(&self) -> &super::SvgScaleMode {
        &self.scale
    }
}
