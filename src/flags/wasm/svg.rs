use crate::flags::SvgAsset as SvgAssetBorrowed;
use crate::prelude::*;
use std::marker::PhantomData;

#[derive(Serialize, Deserialize, Tsify)]
pub(crate) struct SvgAsset<'a> {
    pub(crate) data: Box<[u8]>,
    pub(crate) scale: SvgScaleMode,
    #[serde(skip)]
    _marker: PhantomData<&'a ()>,
}

impl From<SvgAssetBorrowed<'_>> for SvgAsset<'_> {
    fn from(asset: SvgAssetBorrowed<'_>) -> Self {
        SvgAsset {
            data: asset.data.to_vec().into_boxed_slice(),
            scale: asset.scale,
            _marker: PhantomData,
        }
    }
}

pub(crate) type SvgAssetOwned<'a> = SvgAsset<'a>;
