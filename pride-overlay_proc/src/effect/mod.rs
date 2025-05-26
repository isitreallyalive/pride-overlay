use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{Data, DeriveInput};

mod wasm;

pub fn impl_(input: &DeriveInput) -> TokenStream {
    let builder = builder(input);
    let wasm = wasm::generate(input);

    // reject any effect that is not a struct
    if !matches!(input.data, Data::Struct(_)) {
        return quote! {
            compile_error!("Effects must be structs.");
        };
    }

    quote! {
        #builder
        #wasm
    }
}

/// Generate the builder methods for the effect.
fn builder(DeriveInput { ident: effect, .. }: &DeriveInput) -> TokenStream {
    let builder = format_ident!("{}Builder", effect);

    // create documentation strings for the builder methods
    let new_doc = format!(
        "Create a new [{}] [Effect] with a builtin [PrideFlag][crate::PrideFlag].",
        effect
    );
    let custom_doc = format!("Create a new [{}] [Effect] with a custom [Flag].", effect);

    quote! {
        impl #effect<'static> {
            #[doc = #new_doc]
            pub const fn new(flag: crate::PrideFlag) -> #builder<'static> {
                Self::builder(flag.data())
            }

            #[doc = #custom_doc]
            pub const fn custom(flag: crate::Flag<'static>) -> #builder<'static> {
                Self::builder(flag)
            }
        }
    }
}
