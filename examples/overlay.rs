use image::ImageResult;
use pride_overlay::{Flag, Opacity};

fn main() -> ImageResult<()> {
    let mut image = image::open("examples/input.webp")?;
    Flag::Transgender.overlay(&mut image, Some(Opacity::new(0.4)));
    image.save("examples/overlay.webp")?;

    Ok(())
}
