use image::ImageResult;
use pride_overlay::prelude::*;

fn main() -> ImageResult<()> {
    for flag in PrideFlag::all().to_owned() {
        let mut image = image::open("pride-overlay/examples/input.webp")?;
        let effect = Ring::builder(flag).build();
        effect.apply(&mut image);

        image.save(format!(
            "pride-overlay/examples/out/ring/{}.webp",
            flag.name().to_lowercase()
        ))?;
    }

    Ok(())
}
