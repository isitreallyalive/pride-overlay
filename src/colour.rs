/// Represents a colour in RGB format.
#[derive(Clone, Copy)]
pub struct Colour(pub(crate) u8, pub(crate) u8, pub(crate) u8);

impl Colour {
    /// Create a [Colour] from RGB components.
    pub const fn rgb(r: u8, g: u8, b: u8) -> Colour {
        Colour(r, g, b)
    }

    /// Create a [Colour] from a hexadecimal RGB value (e.g., `0xRRGGBB`).
    pub const fn hex(hex: u32) -> Colour {
        let r = ((hex >> 16) & 0xFF) as u8;
        let g = ((hex >> 8) & 0xFF) as u8;
        let b = (hex & 0xFF) as u8;
        Colour(r, g, b)
    }
}

// deserialize hex strings
#[cfg(target_arch = "wasm32")]
impl<'de> serde::Deserialize<'de> for Colour {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let s = s.trim_start_matches('#');
        let hex = u32::from_str_radix(s, 16).map_err(serde::de::Error::custom)?;
        Ok(Self::hex(hex))
    }
}

#[cfg(target_arch = "wasm32")]
impl serde::Serialize for Colour {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&format!("#{:02X}{:02X}{:02X}", self.0, self.1, self.2))
    }
}
