use pride_overlay::{overlay, Flag, Opacity};

fn main() {
    let mut img = image::open("input.jpg").unwrap();
    overlay(&mut img, Flag::Pansexual, Opacity::from_percentage(25.))
        .save("out.png")
        .unwrap();
}
