use image::ImageResult;
use pride_overlay::prelude::*;

fn main() -> ImageResult<()> {
    for flag in PrideFlag::all().to_owned() {
        let mut image = image::open("examples/input.webp")?;
        let effect = Ring::new(flag).build();
        effect.apply(&mut image);

        image.save(format!(
            "examples/out/ring/{}.webp",
            flag.name().to_lowercase()
        ))?;
    }

    Ok(())
}
