use core::fmt;

/// Represents a colour in RGB format.
#[derive(Clone, Copy)]
pub struct Colour {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Colour {
    /// Creates a new [Colour] from the given RGB values.
    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Colour { r, g, b }
    }
}

impl fmt::Display for Colour {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {})", self.r, self.g, self.b)
    }
}

impl fmt::Debug for Colour {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // pass through to Display
        write!(f, "{self}")
    }
}
