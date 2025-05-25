/// Represents a colour in RGB format with a proportion.
#[derive(Builder, Clone, Copy)]
#[builder(const)]
pub struct Colour {
    #[builder(start_fn)]
    pub r: u8,
    #[builder(start_fn)]
    pub g: u8,
    #[builder(start_fn)]
    pub b: u8,
    /// The proportion of this colour in a flag.
    #[builder(default = 1)]
    pub proportion: u8,
}

impl Colour {
    /// Creates a new [Colour] from a hexadecimal value.
    pub const fn from_hex(hex: u32) -> ColourBuilder {
        let r = ((hex >> 16) & 0xFF) as u8;
        let g = ((hex >> 8) & 0xFF) as u8;
        let b = (hex & 0xFF) as u8;

        Colour::builder(r, g, b)
    }
}
