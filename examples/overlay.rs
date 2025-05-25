use image::ImageResult;
use pride_overlay::prelude::*;

fn main() -> ImageResult<()> {
    for flag in Flag::all().to_owned() {
        let mut image = image::open("examples/input.webp")?;
        let effect = Overlay::builder(flag).build();
        effect.apply(&mut image);

        image.save(format!(
            "examples/out/overlay/{}.webp",
            flag.name().to_lowercase()
        ))?;
    }

    Ok(())
}
