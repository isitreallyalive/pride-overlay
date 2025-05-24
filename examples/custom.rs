use pride_overlay::{Colour, Flag};

const CUSTOM: &[Colour] = &[
    Colour::new(255, 0, 0), // red
    Colour::new(0, 255, 0), // green
    Colour::new(0, 0, 255), // blue
];

fn main() {
    println!("{:?}", Flag::Custom(CUSTOM.to_vec()));
}
