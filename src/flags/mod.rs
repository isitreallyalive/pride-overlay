use image::Rgb;

pub struct Flag<'a> {
    pub(crate) colours: &'a [Rgb<u8>],
}

const fn hex(value: u32) -> Rgb<u8> {
    Rgb([
        ((value >> 16) & 0xFF) as u8,
        ((value >> 8) & 0xFF) as u8,
        (value & 0xFF) as u8,
    ])
}

pub const RAINBOW: Flag = Flag {
    colours: &[
        hex(0xE50000), // red
        hex(0xFF8D00), // orange
        hex(0xFFEE00), // yellow
        hex(0x028121), // dark green
        hex(0x004CFF), // blue
        hex(0x770088), // dark purple
    ],
};
