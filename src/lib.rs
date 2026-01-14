mod image;

pub mod prelude {
    pub use crate::PrideError;
    pub use crate::image::Image;
}

// todo: clear error handling
#[derive(Debug, thiserror::Error)]
pub enum PrideError {
    #[error(transparent)]
    Image(#[from] ::image::ImageError),
}
