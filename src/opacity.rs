/// The opacity of a pixel (0-255)
pub struct Opacity(pub u8);

impl Opacity {
    pub fn new(opacity: u8) -> Opacity {
        Opacity(opacity)
    }

    pub fn from_percentage(percentage: f32) -> Option<Opacity> {
        if percentage >= 0. && percentage <= 100. {
            return Some(Opacity((2.55 * percentage).floor() as u8));
        }

        None
    }
}
