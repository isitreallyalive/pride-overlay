/// Generates an enum `Flag` with variants for each flag, and constants for the colours.
macro_rules! generate_flags {
    ($($name:ident : $(($r:expr, $g:expr, $b:expr)),+$(;)?);+) => {
        pub enum Flag {
            $(
                $name,
            )+
            Custom(Vec<$crate::Colour>),
        }

        $(
            #[allow(non_upper_case_globals)]
            const $name: &[$crate::Colour] = &[$($crate::Colour::new($r, $g, $b)),+];
        )+

        impl Flag {
            /// The colours of the flag.
            pub fn colours(&self) -> &[$crate::Colour] {
                match self {
                    $(Flag::$name => $name,)+
                    Flag::Custom(colours) => colours,
                }
            }
        }

        impl std::fmt::Display for Flag {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                // name
                match self {
                    $(Flag::$name => write!(f, "{}", stringify!($name)),)+
                    Flag::Custom(_) => write!(f, "Custom"),
                }?;

                // start colours
                write!(f, ": [")?;

                // colours
                for (i, colour) in self.colours().iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", colour)?;
                }

                // end colours
                write!(f, "]")
            }
        }

        impl std::fmt::Debug for Flag {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                // pass through to Display
                write!(f, "{}", self)
            }
        }
    };
}

generate_flags! {
    Rainbow: (229, 0, 0), (255, 141, 0), (255, 238, 0), (2, 129, 33), (0, 76, 255), (119, 0, 136);
    Transgender: (91, 207, 251), (245, 171, 185), (255, 255, 255), (245, 171, 185), (91, 207, 251);
    // todo: add more flags
}
