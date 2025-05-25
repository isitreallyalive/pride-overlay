use image::ImageResult;
use pride_overlay::{Flag, Opacity};

const FLAGS: &[Flag] = &[Flag::Rainbow, Flag::Transgender, Flag::Intersex];

fn main() -> ImageResult<()> {
    for flag in FLAGS {
        let mut image = image::open("examples/input.webp")?;
        flag.overlay(&mut image, Opacity::new(0.4));
        image.save(format!(
            "examples/out/overlay/{}.webp",
            flag.name().to_lowercase()
        ))?;
    }

    Ok(())
}
