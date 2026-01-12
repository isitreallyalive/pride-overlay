use std::path::{PathBuf};

use image::ImageResult;
use pride_overlay::prelude::*;

/// Apply the given effect and flag to the image at the given path, saving the result to an "out" directory.
pub fn run(
    path: impl Into<PathBuf>,
    effect: impl Effect,
    flag: Flag,
    out_path: impl Into<PathBuf>,
) -> ImageResult<()> {
    // apply
    let path = PathBuf::from("examples").join(path.into());
    let mut image = image::open(path)?;
    effect.apply(&mut image, flag);

    // save
    let out_path = PathBuf::from("examples/out").join(out_path.into());
    std::fs::create_dir_all(out_path.parent().unwrap())?;
    image.save(out_path)?;

    Ok(())
}
