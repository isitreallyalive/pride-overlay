use core::fmt;

/// Represents a colour in RGB format.
#[derive(Clone, Copy, PartialEq, Eq)]
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

    /// Creates a new [Colour] from a hexadecimal value.
    pub const fn from_hex(hex: u32) -> Self {
        Colour {
            r: ((hex >> 16) & 0xFF) as u8,
            g: ((hex >> 8) & 0xFF) as u8,
            b: (hex & 0xFF) as u8,
        }
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
