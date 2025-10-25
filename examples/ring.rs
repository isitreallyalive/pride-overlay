use image::ImageResult;
use pride_overlay::prelude::*;

const EFFECT: Ring = Ring::builder().build();

fn main() -> ImageResult<()> {
    for flag in Flags::all().to_owned() {
        let mut image = image::open("examples/input.webp")?;
        EFFECT.apply(&mut image, flag);

        image.save(format!(
            "examples/out/ring/{}.webp",
            flag.name.to_lowercase()
        ))?;
    }

    Ok(())
}
