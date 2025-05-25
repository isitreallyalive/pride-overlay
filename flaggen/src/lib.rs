use codegen::*;
use parse::Definitions;
use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;

mod codegen;
mod parse;

/// Generates Flag enum and associated implementation from flag definitions.
#[proc_macro]
pub fn generate_flags(input: TokenStream) -> TokenStream {
    let Definitions { flags } = parse_macro_input!(input as Definitions);

    let flag_variants = generate_flag_variants(&flags);
    let flag_constants = generate_flag_constants(&flags);
    let flag_data_matches = generate_flag_data_matches(&flags);
    let flag_name_matches = generate_flag_name_matches(&flags);

    let generated = quote! {
        /// Represents various pride flags.
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum Flag {
            #(#flag_variants),*,
            /// A custom flag with user-provided colors.
            Custom(&'static [crate::Colour]),
        }

        #(#flag_constants)*

        impl Flag {
            /// Returns the data associated with the flag.
            pub(crate) const fn data(&self) -> FlagData {
                match self {
                    #(#flag_data_matches),*,
                    Flag::Custom(colours) => FlagData::Colours(colours),
                }
            }

            /// Returns the human-readable name of the flag.
            pub const fn name(&self) -> &'static str {
                match self {
                    #(#flag_name_matches),*,
                    Flag::Custom(_) => "Custom",
                }
            }
        }
    };

    generated.into()
}
