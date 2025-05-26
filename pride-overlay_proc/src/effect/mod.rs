use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::DeriveInput;

pub fn impl_(input: &DeriveInput) -> TokenStream {
    let builder = builder(input);
    let wasm = wasm(input);

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

fn wasm(DeriveInput { ident: effect, .. }: &DeriveInput) -> TokenStream {
    let wasm_effect = format_ident!("{}Wasm", effect);

    quote! {
        #[cfg(wasm)]
        #[wasm_bindgen(js_name = #effect)]
        struct #wasm_effect {}
    }
}
