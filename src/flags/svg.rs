/// Scaling modes for SVG assets.
#[derive(Clone, Copy, Debug)]
pub enum SvgScaleMode {
    /// Fit the whole SVG inside the destination (preserve aspect ratio).
    Contain,
    /// Fill the destination, cropping if necessary (preserve aspect ratio).
    Cover,
    /// Stretch to exactly fill the destination (ignore aspect ratio).
    Stretch,
    /// Place the SVG at its intrinsic size (no scaling).
    None,
}

/// An SVG asset to be used when rendering a pride flag.
#[derive(Clone, Copy)]
pub struct SvgAsset<'a> {
    pub data: &'a [u8],
    pub mode: SvgScaleMode,
}

impl<'a> SvgAsset<'a> {
    /// Create a new [SvgAsset] from raw SVG data and a [SvgScaleMode].
    pub const fn new(bytes: &'a [u8], mode: SvgScaleMode) -> Self {
        SvgAsset { data: bytes, mode }
    }
}
