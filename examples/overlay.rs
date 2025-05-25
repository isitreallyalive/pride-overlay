use image::ImageResult;
use pride_overlay::prelude::*;

const FLAGS: &[Flag] = &[Flag::Rainbow, Flag::Transgender, Flag::Intersex];

fn main() -> ImageResult<()> {
    for flag in FLAGS.to_owned() {
        let mut image = image::open("examples/input.webp")?;
        let effect = Overlay::builder(flag)
            .maybe_opacity(Opacity::new(0.4))
            .build();
        effect.apply(&mut image);

        image.save(format!(
            "examples/out/overlay/{}.webp",
            flag.name().to_lowercase()
        ))?;
    }

    Ok(())
}
