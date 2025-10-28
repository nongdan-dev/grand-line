use crate::prelude::*;
use syn::{
    Meta, Result,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    token::Comma,
};

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum AttrParseTy {
    Path,
    NameValue,
    List,
}

/// Only in proc macro. For example with proc_macro(k, k1=v1, k2=v2)
/// it will only pass the nested part k, k1=v1, k2=v2 to this impl.
#[derive(Debug, Clone)]
pub struct AttrParse {
    pub args: Vec<(String, (String, AttrParseTy))>,
    /// Only in proc macro #crud[Model, ...].
    /// The first path will be the model name.
    pub first_path: Option<String>,
}

impl AttrParse {
    pub fn into_inner<A>(self, name: &str) -> A
    where
        A: From<Attr> + AttrValidate,
    {
        Attr::from_proc_macro(name, self).into_with_validate()
    }
}

impl Parse for AttrParse {
    fn parse(s: ParseStream) -> Result<Self> {
        let mut args = Vec::new();
        let mut first = true;
        let mut first_path = None;
        for m in Punctuated::<Meta, Comma>::parse_terminated(s)? {
            let (k, v, ty);
            match m {
                Meta::Path(m) => {
                    k = s!(m.get_ident().to_token_stream());
                    v = s!();
                    ty = AttrParseTy::Path;
                }
                Meta::NameValue(m) => {
                    k = s!(m.path.get_ident().to_token_stream());
                    v = s!(m.value.to_token_stream());
                    ty = AttrParseTy::NameValue;
                }
                Meta::List(m) => {
                    k = s!(m.path.get_ident().to_token_stream());
                    v = s!(m.tokens);
                    ty = AttrParseTy::List;
                }
            }
            if first && ty == AttrParseTy::Path {
                first_path = Some(k.clone());
            }
            args.push((k, (v, ty)));
            first = false;
        }
        Ok(Self { args, first_path })
    }
}
