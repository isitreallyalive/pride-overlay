use proc_macro::TokenStream;
use quote::quote;
use syn::DeriveInput;

pub fn impl_(input: &DeriveInput) -> TokenStream {
    let name = &input.ident;
    let builder_name = quote::format_ident!("{}Builder", name);

    let new_doc = format!(
        "Create a new [{}] [Effect] with a builtin [PrideFlag][crate::PrideFlag].",
        name
    );
    let custom_doc = format!("Create a new [{}] [Effect] with a custom [Flag].", name);

    quote! {
        impl #name<'static> {
            #[doc = #new_doc]
            pub const fn new(flag: crate::PrideFlag) -> #builder_name<'static> {
                Self::builder(flag.data())
            }

            #[doc = #custom_doc]
            pub const fn custom(flag: crate::Flag<'static>) -> #builder_name<'static> {
                Self::builder(flag)
            }
        }
    }
    .into()
}
