use image::ImageResult;
use pride_overlay::prelude::*;
use std::time::Instant;

const EFFECT: Ring = Ring::builder().build();

fn main() -> ImageResult<()> {
    let image: image::DynamicImage = image::open("examples/input.webp")?;

    for flag in PresetFlag::all().to_owned() {
        let mut image = image.clone();
        
        let start = Instant::now();
        EFFECT.apply(&mut image, flag);
        let duration = start.elapsed();

        image.save(format!(
            "examples/out/ring/{}.webp",
            flag.name().to_lowercase()
        ))?;

        println!("{}: {:?}", flag.name(), duration);
    }

    Ok(())
}
