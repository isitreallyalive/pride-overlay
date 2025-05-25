use image::ImageResult;
use pride_overlay::Flag;

fn main() -> ImageResult<()> {
    let mut image = image::open("examples/input.jpg")?;
    Flag::Transgender.ring(&mut image, None, None);
    image.save("examples/ring.png")?;

    Ok(())
}
