use crate::parse::{Colour, Flag, FlagDefinition};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

/// Generates the Flag enum variants from flag definitions.
pub fn generate_flag_variants(flags: &[Flag]) -> impl Iterator<Item = TokenStream> + '_ {
    flags.iter().map(|flag| {
        let name = &flag.name;
        let lower_name = name.to_string().to_lowercase();
        let path = match flag.definition {
            FlagDefinition::Svg(_, _) => "",
            _ => "/readme"
        };
        let doc = format!(r#" <img src="https://raw.githubusercontent.com/isitreallyalive/pride-overlay/refs/heads/main/flags{path}/{lower_name}.svg" alt="{lower_name} flag" height="125px">"#);
        quote! {
            #[doc = #doc]
            #name
        }
    })
}

/// Generates constants for flag data and colors.
pub fn generate_flag_constants(flags: &[Flag]) -> impl Iterator<Item = TokenStream> + '_ {
    flags.iter().map(|flag| {
        let name = &flag.name;
        let colour_const = create_colour_const_name(name);

        match &flag.definition {
            FlagDefinition::Svg(path, colours) => {
                generate_svg_flag_constants(name, path, colours, &colour_const)
            }
            FlagDefinition::Colors(colours) => {
                generate_color_flag_constants(colours, &colour_const)
            }
        }
    })
}

/// Generates constants for Svg flags (with embedded image data).
pub fn generate_svg_flag_constants(
    name: &syn::Ident,
    path: &str,
    colours: &[Colour],
    colour_const: &syn::Ident,
) -> TokenStream {
    let file_path = format!("../../flags/{}", path);
    let data_const = create_data_const_name(name);
    let colour_tokens = generate_colour_tokens(colours);

    quote! {
        /// Embedded image data for the flag.
        const #data_const: &'static [u8] = include_bytes!(#file_path);

        /// Color palette for the flag.
        const #colour_const: &'static [crate::Colour] = &[
            #(#colour_tokens),*
        ];
    }
}

/// Generates constants for color-only flags.
pub fn generate_color_flag_constants(colours: &[Colour], colour_const: &syn::Ident) -> TokenStream {
    let colour_tokens = generate_colour_tokens(colours);

    quote! {
        /// Color stripes for the flag.
        const #colour_const: &'static [crate::Colour] = &[
            #(#colour_tokens),*
        ];
    }
}

/// Converts color definitions to token streams.
pub fn generate_colour_tokens(colours: &[Colour]) -> impl Iterator<Item = TokenStream> + '_ {
    colours.iter().map(|c| {
        let Colour {
            r,
            g,
            b,
            proportion,
        } = c;

        quote! { crate::Colour::builder(#r, #g, #b).proportion(#proportion).build() }
    })
}

/// Generates match arms for flag data access.
pub fn generate_flag_data_matches(flags: &[Flag]) -> impl Iterator<Item = TokenStream> + '_ {
    flags.iter().map(|flag| {
        let name = &flag.name;
        let colour_const = create_colour_const_name(name);

        match &flag.definition {
            FlagDefinition::Svg(_, _) => {
                let data_const = create_data_const_name(name);
                quote! {
                    Flag::#name => FlagData::Svg(#data_const, #colour_const)
                }
            }
            FlagDefinition::Colors(_) => {
                quote! {
                    Flag::#name => FlagData::Colours(#colour_const)
                }
            }
        }
    })
}

/// Generates match arms for flag name access.
pub fn generate_flag_name_matches(flags: &[Flag]) -> impl Iterator<Item = TokenStream> + '_ {
    flags.iter().map(|flag| {
        let name = &flag.name;
        let name_str = name.to_string();
        quote! {
            Flag::#name => #name_str
        }
    })
}

/// Creates a constant name for flag colors.
pub fn create_colour_const_name(flag_name: &syn::Ident) -> syn::Ident {
    format_ident!("{}_COLOURS", flag_name.to_string().to_uppercase())
}

/// Creates a constant name for flag data.
pub fn create_data_const_name(flag_name: &syn::Ident) -> syn::Ident {
    format_ident!("{}_DATA", flag_name.to_string().to_uppercase())
}

/// Generates all flag variants for the all() method.
pub fn generate_all_flags(flags: &[Flag]) -> impl Iterator<Item = TokenStream> + '_ {
    flags.iter().map(|flag| {
        let name = &flag.name;
        quote! {
            Flag::#name
        }
    })
}
