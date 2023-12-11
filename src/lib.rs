use image::{imageops, DynamicImage, GenericImageView};

// Include all of the flags
pub enum Flag {
    Agender,
    Ally,
    Aromantic,
    Asexual,
    Bisexual,
    Demisexual,
    Genderfluid,
    Genderqueer,
    Homosexual,
    Intersex,
    Lesbian,
    Nonbinary,
    Pansexual,
    Polyamory,
    Polysexual,
    Progress,
    Transgender,
}

static AGENDER: &'static [u8] = include_bytes!("agender.png");
static ALLY: &'static [u8] = include_bytes!("ally.png");
static AROMANTIC: &'static [u8] = include_bytes!("aromantic.png");
static ASEXUAL: &'static [u8] = include_bytes!("asexual.png");
static BISEXUAL: &'static [u8] = include_bytes!("bisexual.png");
static DEMISEXUAL: &'static [u8] = include_bytes!("demisexual.png");
static GENDERFLUID: &'static [u8] = include_bytes!("genderfluid.png");
static GENDERQUEER: &'static [u8] = include_bytes!("genderqueer.png");
static HOMOSEXUAL: &'static [u8] = include_bytes!("homosexual.png");
static INTERSEX: &'static [u8] = include_bytes!("intersex.png");
static LESBIAN: &'static [u8] = include_bytes!("lesbian.png");
static NONBINARY: &'static [u8] = include_bytes!("nonbinary.png");
static PANSEXUAL: &'static [u8] = include_bytes!("pansexual.png");
static POLYAMORY: &'static [u8] = include_bytes!("polyamory.png");
static POLYSEXUAL: &'static [u8] = include_bytes!("polysexual.png");
static PROGRESS: &'static [u8] = include_bytes!("progress.png");
static TRANSGENDER: &'static [u8] = include_bytes!("transgender.png");

// Opacity helper
#[derive(Debug)]
pub struct Opacity(u8);

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

// Load a flag from memory
fn load_flag(buffer: &[u8]) -> DynamicImage {
    image::load_from_memory(buffer).unwrap()
}

pub fn overlay(image: &mut DynamicImage, flag: Flag, opacity: Option<Opacity>) -> DynamicImage {
    let (width, height) = image.dimensions();
    let parsed_opacity = opacity.unwrap_or(Opacity(127));

    let mut flag = match flag {
        Flag::Agender => load_flag(AGENDER),
        Flag::Ally => load_flag(ALLY),
        Flag::Aromantic => load_flag(AROMANTIC),
        Flag::Asexual => load_flag(ASEXUAL),
        Flag::Bisexual => load_flag(BISEXUAL),
        Flag::Demisexual => load_flag(DEMISEXUAL),
        Flag::Genderfluid => load_flag(GENDERFLUID),
        Flag::Genderqueer => load_flag(GENDERQUEER),
        Flag::Homosexual => load_flag(HOMOSEXUAL),
        Flag::Intersex => load_flag(INTERSEX),
        Flag::Lesbian => load_flag(LESBIAN),
        Flag::Nonbinary => load_flag(NONBINARY),
        Flag::Pansexual => load_flag(PANSEXUAL),
        Flag::Polyamory => load_flag(POLYAMORY),
        Flag::Polysexual => load_flag(POLYSEXUAL),
        Flag::Progress => load_flag(PROGRESS),
        Flag::Transgender => load_flag(TRANSGENDER),
    }
    .resize_exact(width, height, imageops::FilterType::Nearest)
    .to_rgba8();

    for p in flag.pixels_mut() {
        p[3] = parsed_opacity.0;
    }

    imageops::overlay(image, &flag, 0, 0);

    image.clone()
}
