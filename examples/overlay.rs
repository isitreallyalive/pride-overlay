use image::ImageResult;
use pride_overlay::{Flag, Opacity};

fn main() -> ImageResult<()> {
    let mut image = image::open("examples/input.jpg")?;
    Flag::Transgender.overlay(&mut image, Some(Opacity::percent(0.4)));
    image.save("examples/overlay.png")?;

    Ok(())
}
