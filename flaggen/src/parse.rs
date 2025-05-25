use syn::{
    Ident, LitInt, LitStr, Result, Token,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
};

pub struct Definitions {
    pub flags: Vec<Flag>,
}

pub struct Flag {
    pub name: syn::Ident,
    pub definition: FlagDefinition,
}

pub enum FlagDefinition {
    Colors(Vec<Color>),
    Path(String),
}

pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Parse for Color {
    fn parse(input: ParseStream) -> Result<Self> {
        let content;
        syn::parenthesized!(content in input);

        let r: LitInt = content.parse()?;
        content.parse::<Token![,]>()?;
        let g: LitInt = content.parse()?;
        content.parse::<Token![,]>()?;
        let b: LitInt = content.parse()?;

        Ok(Color {
            r: r.base10_parse()?,
            g: g.base10_parse()?,
            b: b.base10_parse()?,
        })
    }
}

impl Parse for Definitions {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut flags = Vec::new();

        while !input.is_empty() {
            let name: Ident = input.parse()?;
            input.parse::<Token![:]>()?;

            let definition = if input.peek(LitStr) {
                let path: LitStr = input.parse()?;
                FlagDefinition::Path(path.value())
            } else {
                let colours: Punctuated<Color, Token![,]> =
                    Punctuated::parse_separated_nonempty(input)?;
                FlagDefinition::Colors(colours.into_iter().collect())
            };

            flags.push(Flag { name, definition });
        }

        Ok(Definitions { flags })
    }
}
