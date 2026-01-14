pub mod effect;
pub mod image;

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
}
