use crate::prelude::*;
use syn::{
    Meta, Result,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    token::Comma,
};

/// Only in proc macro. For example with `#[proc_macro(k, k1=v1, k2=v2)]`
/// it will only pass the nested `k, k1=v1, k2=v2` part to this impl.
#[derive(Debug, Clone)]
pub struct AttrParse {
    pub args: Vec<(String, (String, AttrTy))>,
    /// Only in proc macro #crud[Model, ...].
    /// The first path will be the model name.
    pub first_path: Option<String>,
}

impl AttrParse {
    pub fn into_inner<T>(self, name: &str) -> T
    where
        T: From<Attr> + AttrValidate,
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
                    k = str!(m.get_ident().unwrap());
                    v = str!();
                    ty = AttrTy::Path;
                }
                Meta::NameValue(m) => {
                    k = str!(m.path.get_ident().unwrap());
                    v = str!(m.value.to_token_stream());
                    ty = AttrTy::NameValue;
                }
                Meta::List(m) => {
                    k = str!(m.path.get_ident().unwrap());
                    v = str!(m.tokens);
                    ty = AttrTy::List;
                }
            }
            if first && ty == AttrTy::Path {
                first_path = Some(k.clone());
            }
            args.push((k, (v, ty)));
            first = false;
        }
        Ok(Self { args, first_path })
    }
}
