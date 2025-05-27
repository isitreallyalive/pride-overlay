use proc_macro::TokenStream;
use syn::{ItemFn, token::Const};

mod effect;
mod flags;

#[proc_macro]
pub fn generate_flags(input: TokenStream) -> TokenStream {
    let parsed = flags::parse(input).expect("parse error");
    let generated = flags::generate(parsed);
    generated.into()
}

#[proc_macro_derive(Effect)]
pub fn derive_effect(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    effect::impl_(&ast).into()
}

#[proc_macro_attribute]
pub fn native_const(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input_fn: ItemFn = syn::parse(item).expect("expected a function");
    let mut const_fn = input_fn.clone();
    const_fn.sig.constness = Some(Const::default());

    quote::quote! {
        #[cfg(not(wasm))]
        #const_fn

        #[cfg(wasm)]
        #input_fn
    }
    .into()
}
