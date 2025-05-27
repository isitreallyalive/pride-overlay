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
    let initializers = s.fields.iter().map(|Field { ident: name, .. }| name);

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

            #[wasm_bindgen]
            pub fn apply(&self, image: &[u8], width: u32, height: u32) -> Vec<u8> {
                use image::{RgbaImage, DynamicImage};

                // create a dynamic image
                let mut image = {
                    let rgba = RgbaImage::from_raw(
                        width,
                        height,
                        image.to_vec()
                    ).expect("Failed to create image from raw data");
                    DynamicImage::ImageRgba8(rgba)
                };

                // create the effect
                let effect = crate::#effect {
                    #(#initializers: self.#initializers.clone().into()),*
                };
                effect.apply(&mut image);

                // convert the image back to a vector
                image.to_rgba8().into_vec()
            }
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
