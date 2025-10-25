use image::ImageResult;
use pride_overlay::prelude::*;

const EFFECT: Overlay = Overlay::builder().build();

fn main() -> ImageResult<()> {
    for flag in Flags::all().to_owned() {
        let mut image: image::DynamicImage = image::open("examples/input.webp")?;
        EFFECT.apply(&mut image, flag);

        image.save(format!(
            "examples/out/overlay/{}.webp",
            flag.name.to_lowercase()
        ))?;
    }

    Ok(())
}
