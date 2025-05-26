use proc_macro2::TokenStream;
use quote::format_ident;
use quote::quote;
use syn::PathSegment;
use syn::Type;
use syn::TypePath;
use syn::parse_quote;
use syn::{Data, DeriveInput, Field};

/// Generate a struct that represents the effect in a format suitable for wasm.
pub fn generate(
    DeriveInput {
        ident: effect,
        data,
        ..
    }: &DeriveInput,
) -> TokenStream {
    let wasm_effect = format_ident!("{}Wasm", effect);

    // extract the fields on the rust struct
    let fields = if let Data::Struct(s) = data {
        s.fields.iter().map(
            |Field {
                 ident: name, ty, ..
             }| {
                let ty = translate_type(&ty);
                quote! {
                    pub #name: #ty
                }
            },
        )
    } else {
        unreachable!()
    };

    quote! {
        #[cfg(wasm)]
        #[wasm_bindgen(js_name = #effect, js_namespace = Effect)]
        struct #wasm_effect {
            #(#fields),*
        }

        #[cfg(wasm)]
        #[wasm_bindgen(js_class = #effect)]
        impl #wasm_effect {
        }
    }
}

/// Translate types that are not directly compatible with wasm.
fn translate_type(ty: &Type) -> Type {
    if let Type::Path(TypePath { path, .. }) = ty {
        if let Some(PathSegment { ident: name, .. }) = path.segments.last() {
            match name.to_string().as_str() {
                "Opacity" => return parse_quote!(f32),
                "Flag" => return parse_quote!(crate::PrideFlag),
                _ => {}
            }
        }
    }

    ty.clone()
}
