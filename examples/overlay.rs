use image::ImageResult;
use pride_overlay::prelude::*;

use crate::helpers::run;

mod helpers;

const EFFECT: Overlay = Overlay::builder().build();

fn main() -> ImageResult<()> {
    for flag in PresetFlag::all().to_owned() {
        let flag_name = flag.name().to_lowercase();

        {
            let out = format!("overlay/{}.webp", flag_name);
            println!("processing {out}...");
            run("input.webp", EFFECT, flag, out)?;
        }

        {
            let out = format!("overlay/{}.gif", flag_name);
            println!("processing {out}...");
            run("input.gif", EFFECT, flag, out)?;
        }
    }

    Ok(())
}
