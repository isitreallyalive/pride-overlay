use strum_macros::{Display, EnumIter, EnumString};

/// Built-in flags
#[derive(Display, EnumString, EnumIter)]
pub enum Flags {
    Rainbow,
    Transgender,
    Bisexual,
    Pansexual,
    Nonbinary,
    Lesbian,
    Agender,
    Asexual,
    Genderqueer,
    Genderfluid,
    Aromantic,
    Polyamory,
}

impl From<Flags> for Vec<(u8, u8, u8)> {
    fn from(val: Flags) -> Self {
        match val {
            Flags::Rainbow => vec![
                (229, 0, 0),
                (255, 141, 0),
                (255, 238, 0),
                (2, 129, 33),
                (0, 76, 255),
                (119, 0, 136),
            ],
            Flags::Transgender => vec![
                (91, 207, 251),
                (245, 171, 185),
                (255, 255, 255),
                (245, 171, 185),
                (91, 207, 251),
            ],
            Flags::Bisexual => vec![(214, 2, 112), (155, 79, 150), (0, 56, 168)],
            Flags::Pansexual => vec![(255, 28, 141), (255, 215, 0), (26, 179, 255)],
            Flags::Nonbinary => vec![
                (252, 244, 49),
                (252, 252, 252),
                (157, 89, 210),
                (40, 40, 40),
            ],
            Flags::Lesbian => vec![
                (214, 40, 0),
                (255, 155, 86),
                (255, 255, 255),
                (212, 98, 166),
                (164, 0, 98),
            ],
            Flags::Agender => vec![
                (0, 0, 0),
                (186, 186, 186),
                (255, 255, 255),
                (186, 244, 132),
                (255, 255, 255),
                (186, 186, 186),
                (0, 0, 0),
            ],
            Flags::Asexual => vec![(0, 0, 0), (164, 164, 164), (255, 255, 255), (129, 0, 129)],
            Flags::Genderqueer => vec![(181, 127, 221), (255, 255, 255), (73, 130, 30)],
            Flags::Genderfluid => vec![
                (254, 118, 162),
                (255, 255, 255),
                (191, 18, 215),
                (0, 0, 0),
                (48, 60, 190),
            ],
            Flags::Aromantic => vec![
                (59, 167, 64),
                (168, 212, 122),
                (255, 255, 255),
                (171, 171, 171),
                (0, 0, 0),
            ],
            Flags::Polyamory => vec![(0, 0, 255), (255, 0, 0), (0, 0, 0)],
        }
    }
}
