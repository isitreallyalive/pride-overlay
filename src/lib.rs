pub mod effect;
pub mod flags;
pub mod image;
#[cfg(target_arch = "wasm32")]
mod wasm;

pub mod prelude {
    pub use crate::PrideError;
    pub use crate::effect::{Effect, overlay::Overlay};
    pub use crate::image::Image;
    pub use image::ImageFormat;
}

// todo: clear error handling
#[derive(Debug, thiserror::Error)]
pub enum PrideError {
    #[error(transparent)]
    Image(#[from] ::image::ImageError),
    #[cfg(feature = "gif")]
    #[error(transparent)]
    GifDecode(#[from] gif::DecodingError),
    #[cfg(feature = "gif")]
    #[error(transparent)]
    GifEncode(#[from] gif::EncodingError),
    #[error(transparent)]
    Io(#[from] std::io::Error),
}
