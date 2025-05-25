use core::fmt;

/// Represents an opacity value, ranging from 0% (fully transparent) to 100% (fully opaque).
#[derive(Clone, Copy)]
pub struct Opacity(u8);

impl Opacity {
    /// Fully transparent (0%)
    pub const TRANSPARENT: Self = Self(0);
    /// Half transparent (50%)
    pub const HALF: Self = Self(128);
    /// Fully opaque (100%)
    pub const OPAQUE: Self = Self(255);

    /// Creates an [Opacity] from a percentage value.
    ///
    /// This percentage is clamped between 0.0 and 1.0, then mapped to a [u8].
    pub const fn new(percentage: f32) -> Self {
        // clamp between 0% and 100%
        let clamped = percentage.clamp(0.0, 1.);
        // turn into a u8 value between 0 and 255
        Self((clamped * u8::MAX as f32) as u8)
    }

    /// Returns the [Opacity] as a percentage value
    pub const fn get(self) -> f32 {
        (self.0 as f32 / u8::MAX as f32) * 100.
    }

    /// Creates an [Opacity] with the given raw [u8] value.
    ///
    /// This function returns an [Option] for ease of consumption, it will never be [None].
    pub const fn raw(opacity: u8) -> Option<Self> {
        Some(Self(opacity))
    }

    /// Returns the raw [u8] value of the [Opacity].
    pub const fn get_raw(&self) -> u8 {
        self.0
    }
}

impl fmt::Display for Opacity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}%", self.get())
    }
}

impl fmt::Debug for Opacity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // pass through to Display
        write!(f, "{self}")
    }
}
