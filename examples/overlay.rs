use image::ImageResult;
use pride_overlay::Flag;

fn main() -> ImageResult<()> {
    let mut image = image::open("examples/input.jpg")?;
    Flag::Transgender.overlay(&mut image);
    image.save("examples/overlay.png")?;

    Ok(())
}
