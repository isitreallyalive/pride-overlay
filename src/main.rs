use pride_overlay::{overlay, Flags, Opacity};

fn main() {
    let mut img = image::open("input.jpg").unwrap();
    overlay(&mut img, Flags::Rainbow, Opacity::from_percentage(60.))
        .save("out.png")
        .unwrap();
}
