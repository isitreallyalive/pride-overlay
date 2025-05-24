/// Generates an enum `Flag` with variants for each flag, and constants for the colours.
macro_rules! generate_flags {
    ($($name:ident : $(($r:expr, $g:expr, $b:expr)),+$(;)?);+) => {
        pub enum Flag {
            $(
                $name,
            )+
            Custom(&'static [$crate::Colour]),
        }

        $(
            #[allow(non_upper_case_globals)]
            const $name: &[$crate::Colour] = &[$($crate::Colour::new($r, $g, $b)),+];
        )+

        impl Flag {
            /// The name of the flag.
            pub const fn name(&self) -> &str {
                match self {
                    $(Flag::$name => stringify!($name),)+
                    Flag::Custom(_) => "Custom",
                }
            }

            /// The colours of the flag.
            pub fn colours(&self) -> &[$crate::Colour] {
                match self {
                    $(Flag::$name => $name,)+
                    Flag::Custom(colours) => colours,
                }
            }
        }

        impl core::fmt::Display for Flag {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                write!(f, "{}: [", self.name())?;

                for (i, colour) in self.colours().iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", colour)?;
                }

                write!(f, "]")
            }
        }

        impl core::fmt::Debug for Flag {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                // pass through to Display
                write!(f, "{self}")
            }
        }
    };
}

generate_flags! {
    Rainbow: (229, 0, 0), (255, 141, 0), (255, 238, 0), (2, 129, 33), (0, 76, 255), (119, 0, 136);
    Transgender: (91, 207, 251), (245, 171, 185), (255, 255, 255), (245, 171, 185), (91, 207, 251);
    // todo: add more flags
}
