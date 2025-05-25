use parse::{Definitions, FlagDefinition};
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::parse_macro_input;

mod parse;

#[proc_macro]
pub fn generate_flags(input: TokenStream) -> TokenStream {
    let Definitions { flags } = parse_macro_input!(input as Definitions);

    // generate Flag enum variants
    let flag_variants = flags.iter().map(|flag| {
        let name = &flag.name;
        quote! { #name }
    });

    let flag_consts = flags.iter().map(|flag| {
        let name = &flag.name;
        let const_name = format_ident!("{}_DATA", name.to_string().to_uppercase());

        match &flag.definition {
            FlagDefinition::Path(path) => {
                let path = format!("../flags/{path}");
                quote! {
                    const #const_name: &'static [u8] = include_bytes!(#path);
                }
            }
            FlagDefinition::Colors(colours) => {
                let colours = colours.iter().map(|c| {
                    let r = c.r;
                    let g = c.g;
                    let b = c.b;
                    quote! { crate::Colour::new(#r, #g, #b) }
                });

                quote! {
                    const #const_name: &'static [crate::Colour] = &[
                        #(#colours),*
                    ];
                }
            }
        }
    });

    let flag_data_matches = flags.iter().map(|flag| {
        let name = &flag.name;
        let const_name = format_ident!("{}_DATA", name.to_string().to_uppercase());

        match &flag.definition {
            FlagDefinition::Path(_) => {
                quote! {
                    Flag::#name => FlagData::Image(#const_name)
                }
            }
            FlagDefinition::Colors(_) => {
                quote! {
                    Flag::#name => FlagData::Colours(#const_name)
                }
            }
        }
    });

    let flag_name_matches = flags.iter().map(|flag| {
        let name = &flag.name;
        let name_str = name.to_string();
        quote! {
            Flag::#name => #name_str
        }
    });

    let generated = quote! {
        pub enum Flag {
            #(#flag_variants),*,
            Custom(&'static [crate::Colour]),
        }

        #(#flag_consts)*

        impl Flag {
            /// Returns the data associated with the flag.
            pub(crate) const fn data(&self) -> FlagData {
                match self {
                    #(#flag_data_matches),*,
                    Flag::Custom(colours) => FlagData::Colours(colours),
                }
            }

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
