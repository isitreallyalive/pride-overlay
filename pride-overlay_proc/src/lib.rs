use proc_macro::TokenStream;

mod flags;

#[proc_macro]
pub fn generate_flags(input: TokenStream) -> TokenStream {
    let parsed = flags::parse(input).expect("parse error");
    let generated = flags::generate(parsed);
    generated.into()
}
