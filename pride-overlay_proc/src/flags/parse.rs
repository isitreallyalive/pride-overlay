use proc_macro::TokenStream;
use syn::{
    Ident, LitInt, Result, Token, braced,
    parse::{Parse, ParseStream},
};

/// Parses the input token stream into [Flags].
pub fn parse(input: TokenStream) -> Result<Flags> {
    syn::parse(input)
}

pub struct Flags(pub Vec<Flag>);

pub struct Flag {
    pub name: Ident,
    pub colours: Vec<Colour>,
    pub scale: Option<Ident>,
}

impl Parse for Flags {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut flags = Vec::new();

        while !input.is_empty() {
            // parse the name
            let name = input.parse()?;

            // parse the braced contents
            let contents;
            braced!(contents in input);

            // parse colours
            let mut colours = Vec::new();

            // is there an svg?
            let scale = contents
                .peek(Ident)
                .then(|| contents.parse().ok())
                .flatten();

            while contents.peek(LitInt) {
                let colour = contents.parse()?;
                // consume comma
                if contents.peek(Token![,]) {
                    contents.parse::<Token![,]>()?;
                }
                colours.push(colour);
            }

            flags.push(Flag {
                name,
                colours,
                scale,
            });
        }

        Ok(Self(flags))
    }
}

pub struct Colour {
    pub hex: u32,
    pub proportion: u8,
}

impl Parse for Colour {
    fn parse(input: ParseStream) -> Result<Self> {
        let hex: LitInt = input.parse()?;

        // Check if there's a proportion specified (optional)
        let proportion = if input.peek(Token![:]) {
            input.parse::<Token![:]>()?;
            let prop: LitInt = input.parse()?;
            prop.base10_parse()?
        } else {
            1 // default proportion
        };

        Ok(Colour {
            hex: hex.base10_parse()?,
            proportion,
        })
    }
}
