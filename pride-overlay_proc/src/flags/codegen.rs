use super::parse::{Colour, Flag, Flags};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::Ident;

/// Generates the code for the pride flag macro.
pub fn generate(Flags(flags): Flags) -> TokenStream {
    let consts = flags.iter().map(const_);
    let doc_variants = flags.iter().map(|v| variant(v, true));
    let variants: Vec<_> = flags.iter().map(|v| variant(v, false)).collect();
    let data_arms = flags.iter().map(data_arm);

    quote! {
        #(#consts)*

        /// A pride flag.
        #[derive(Clone, Copy)]
        pub enum PrideFlag {
            #(#doc_variants),*
        }

        impl PrideFlag {
            /// Enumerates all built-in pride flags.
            pub const fn all() -> &'static [PrideFlag] {
                &[
                    #(
                        PrideFlag::#variants,
                    )*
                ]
            }

            /// Gets the human-readable name of a pride flag.
            pub const fn name(&self) -> &'static str {
                match self {
                    #(
                        PrideFlag::#variants => stringify!(#variants),
                    )*
                }
            }

            /// Internal flag data.
            pub(crate) const fn data(&self) -> Flag<'static> {
                match self {
                    #(#data_arms),*,
                }
            }
        }
    }
}

/// Convert a [syn::Ident] to uppercase.
fn upper_ident(ident: &Ident) -> Ident {
    format_ident!("{}", ident.to_string().to_uppercase())
}

fn const_(
    Flag {
        name,
        colours,
        scale,
    }: &Flag,
) -> TokenStream {
    let name = upper_ident(name);
    let colours = colours.iter().map(|Colour { hex, proportion }| {
        quote! {
            crate::Colour::from_hex(#hex)
                .proportion(#proportion)
                .build()
        }
    });
    let svg = scale
        .as_ref()
        .map(|scale| {
            let file = format!("../flags/{}.svg", name.to_string().to_lowercase());

            quote! {
                Some((include_bytes!(#file), ScaleMode::#scale))
            }
        })
        .unwrap_or(quote! { None });

    quote! {
        const #name: Flag<'static> = Flag::builder(&[#(#colours),*])
            .maybe_svg(#svg)
            .build();
    }
}

/// Convert a flag into a variant of the PrideFlag enum.
fn variant(Flag { name, scale, .. }: &Flag, doc: bool) -> TokenStream {
    if doc {
        let str_name = name.to_string().to_lowercase();
        let path = scale.as_ref().map(|_| "").unwrap_or("/readme");
        let doc = format!(
            r#" | Flag | [Overlay][crate::Overlay] | [Ring][crate::Ring] |
 |----|----|----|
 | {} | {} | {} |"#,
            format!(
                r#"<img src="https://raw.githubusercontent.com/isitreallyalive/pride-overlay/refs/heads/main/flags{path}/{str_name}.svg" alt="{str_name} flag" height="125px">"#
            ),
            format!(
                r#"<img src="https://raw.githubusercontent.com/isitreallyalive/pride-overlay/refs/heads/main/examples/out/overlay/{str_name}.webp" alt="{str_name} overlay" height="125px">"#
            ),
            format!(
                r#"<img src="https://raw.githubusercontent.com/isitreallyalive/pride-overlay/refs/heads/main/examples/out/ring/{str_name}.webp" alt="{str_name} ring" height="125px">"#
            ),
        );

        quote! {
            #[doc = #doc]
            #name
        }
    } else {
        quote! {
            #name
        }
    }
}

/// Generate the match amrms for PrideFlag::data.
fn data_arm(Flag { name, .. }: &Flag) -> TokenStream {
    let const_ = upper_ident(name);

    quote! {
        PrideFlag::#name => #const_
    }
}
