use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{Data, DataStruct, DeriveInput, Field, PathSegment, Type, TypePath, parse_quote};

/// Generate a struct that represents the effect in a format suitable for wasm.
pub fn generate(
    DeriveInput {
        ident: effect,
        data,
        ..
    }: &DeriveInput,
) -> TokenStream {
    let wasm_effect = format_ident!("{}Wasm", effect);

    let Data::Struct(s) = data else {
        unreachable!()
    };

    // generate fields for the struct
    let constructor = constructor(s);
    let struct_fields = s.fields.iter().map(
        |Field {
             ident: name, ty, ..
         }| {
            let translated_ty = translate_type(ty);
            quote! { pub #name: #translated_ty }
        },
    );

    quote! {
        #[cfg(wasm)]
        #[wasm_bindgen(js_name = #effect, js_namespace = Effect)]
        struct #wasm_effect {
            #(#struct_fields),*
        }

        #[cfg(wasm)]
        #[wasm_bindgen(js_class = #effect)]
        impl #wasm_effect {
            #constructor
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

/// Generate the constructor for the wasm struct
fn constructor(DataStruct { fields, .. }: &DataStruct) -> TokenStream {
    let params = fields.iter().map(
        |Field {
             ident: name, ty, ..
         }| {
            let translated_ty = translate_type(ty);
            quote! { #name: #translated_ty }
        },
    );
    let initializers = fields.iter().map(|Field { ident: name, .. }| name);

    quote! {
        #[wasm_bindgen(constructor)]
        pub fn new(#(#params),*) -> Self {
            Self {
                #(#initializers),*
            }
        }
    }
}
