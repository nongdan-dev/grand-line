use crate::prelude::*;
use std::marker::PhantomData;
use syn::{
    Meta, Result, Token,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
};

pub struct AttrParse {
    pub args: Vec<(String, String)>,
    pub model: String,
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

pub struct AttrParseX<T>
where
    T: From<Attr>,
{
    pub inner: AttrParse,
    _attr: PhantomData<T>,
}

impl<T> AttrParseX<T>
where
    T: From<Attr>,
{
    pub fn to_map(self, debug: &str, attr: &str) -> AttrX<T> {
        AttrX::new(debug, attr, self.inner.args, &self.inner.model)
    }
    pub fn attr(self, debug: &str, attr: &str) -> T {
        self.to_map(debug, attr).attr()
    }
}

impl<T> Parse for AttrParseX<T>
where
    T: From<Attr>,
{
    fn parse(s: ParseStream) -> Result<Self> {
        let r = Self {
            inner: AttrParse::parse(s)?,
            _attr: PhantomData,
        };
        Ok(r)
    }
}
