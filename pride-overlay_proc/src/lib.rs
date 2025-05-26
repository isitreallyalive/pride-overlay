use proc_macro::TokenStream;

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
    effect::impl_(&ast)
}
