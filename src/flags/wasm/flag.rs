use crate::{
    Colour,
    flags::{Flag, wasm::svg::SvgAsset},
};
use std::marker::PhantomData;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(typescript_custom_section)]
const HEX: &'static str = r#"
type Hex = `#${string}`;
"#;

#[derive(Serialize, Deserialize, Tsify)]
pub struct FlagData<'a> {
    #[tsify(type = "Hex[]")]
    pub(crate) colours: Box<[Colour]>,
    #[tsify(optional)]
    pub(crate) svg: Option<SvgAsset<'static>>,
    #[serde(skip)]
    pub(crate) _marker: PhantomData<&'a ()>,
}

impl From<Flag<'_>> for FlagOwned<'_> {
    fn from(flag: Flag<'_>) -> Self {
        Self {
            colours: flag.colours.to_vec().into_boxed_slice(),
            svg: flag.svg.map(|svg| svg.into()),
            _marker: PhantomData,
        }
    }
}

pub type FlagOwned<'a> = FlagData<'a>;
