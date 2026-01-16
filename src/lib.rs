pub mod effect;
pub mod flags;
pub mod image;
#[cfg(target_arch = "wasm32")]
mod wasm;

pub mod prelude {
    pub use crate::PrideError;
    pub use crate::effect::{Effect, overlay::Overlay};
    pub use crate::flags;
    pub use crate::image::Image;
    pub use image::ImageFormat;
}

pub type Result<T, E = PrideError> = std::result::Result<T, E>;

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
    #[cfg(feature = "png")]
    #[error(transparent)]
    PngDecode(#[from] png::DecodingError),
    #[cfg(feature = "png")]
    #[error(transparent)]
    PngEncode(#[from] png::EncodingError),
    #[error(transparent)]
    Io(#[from] std::io::Error),
}
