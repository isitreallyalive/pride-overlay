use std::{env, io::Cursor, path::PathBuf};

use pride_overlay::prelude::*;

pub fn run<E: Effect>(
    name: &str,
    data: &[u8],
    effect: E,
    format: ImageFormat,
) -> Result<(), PrideError> {
    println!("applying effect to {:?}", format);

    // apply effect
    let mut image = Image::read(data)?;
    effect.apply(&mut image);

    // write output
    let mut buf = Cursor::new(Vec::new());
    image.write(&mut buf)?;
    let mut out_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("examples")
        .join(name);
    out_path.set_extension(format.extensions_str()[0]);
    std::fs::write(&out_path, buf.into_inner()).expect("couldn't write output");

    println!(
        "written {}",
        out_path
            .file_name()
            .expect("no file name")
            .to_string_lossy()
    );

    Ok(())
}
