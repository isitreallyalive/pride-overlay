mod svg;
pub(crate) use svg::SvgAssetOwned;

mod flag;
pub(crate) use flag::FlagOwned;

use crate::flags::{Flags, wasm::flag::FlagData};

#[derive(Serialize, Deserialize, Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi)]
#[serde(untagged)]
pub enum Flag {
    Default(Flags),
    Custom(FlagData<'static>),
}

impl From<Flag> for FlagOwned<'static> {
    fn from(flag: Flag) -> Self {
        match flag {
            Flag::Default(f) => f.into(),
            Flag::Custom(f) => f,
        }
    }
}