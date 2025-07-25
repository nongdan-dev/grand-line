use crate::prelude::*;
use syn::{
    Meta, Result, Token,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
};

#[derive(Debug, Clone)]
pub struct AttrParse {
    pub args: Vec<(String, String)>,
    pub model: String,
}

#[allow(dead_code)]
impl AttrParse {
    pub fn into_with_validate<T>(self, debug: &str, attr: &str) -> T
    where
        T: From<Attr> + AttrValidate,
    {
        Attr::new(debug, attr, self.args, &self.model).into_with_validate()
    }
}

impl Parse for AttrParse {
    fn parse(s: ParseStream) -> Result<Self> {
        let mut args = Vec::new();
        let mut first = true;
        let mut model = str!();
        for m in Punctuated::<Meta, Token![,]>::parse_terminated(s)? {
            match m {
                Meta::Path(m) => {
                    let k = str!(m.get_ident().unwrap());
                    let v = str!();
                    args.push((k.clone(), v));
                    // no value here, could be model
                    if first {
                        model = k;
                    }
                }
                Meta::NameValue(m) => {
                    let k = str!(m.path.get_ident().unwrap());
                    let v = str!(m.value.to_token_stream());
                    args.push((k, v));
                }
                _ => {}
            }
            first = false;
        }
        Ok(Self { args, model })
    }
}
