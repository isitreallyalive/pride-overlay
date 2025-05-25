use codegen::generate;
use parse::parse;
use proc_macro::TokenStream;

mod codegen;
mod parse;

#[proc_macro]
pub fn generate_flags(input: TokenStream) -> TokenStream {
    let parsed = parse(input).expect("parse error");
    let generated = generate(parsed);
    generated.into()
}
