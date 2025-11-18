use crate::prelude::*;

#[derive(Clone, Eq, PartialEq)]
pub enum AttrParseTy {
    Path,
    NameValue,
    List,
}

/// Only in proc macro. For example with proc_macro(k, k1=v1, k2=v2)
/// it will only pass the nested part k, k1=v1, k2=v2 to this impl.
pub struct AttrParse {
    pub args: Vec<(String, (String, AttrParseTy))>,
    /// Only in proc macro #crud[Model, ...].
    /// The first path will be the model name.
    pub first_path: Option<String>,
}

impl AttrParse {
    pub fn into_inner<A>(self, macro_name: &str) -> A
    where
        A: From<Attr> + AttrValidate,
    {
        Attr::from_proc_macro(macro_name, self).into_with_validate()
    }
}

impl Parse for AttrParse {
    fn parse(s: ParseStream) -> Result<Self> {
        let mut args = Vec::new();
        let mut first = true;
        let mut first_path = None;
        for m in Punctuated::<Meta, Token![,]>::parse_terminated(s)? {
            let (k, v, ty);
            match m {
                Meta::Path(m) => {
                    k = m.get_ident().to_token_stream().to_string();
                    v = "".to_owned();
                    ty = AttrParseTy::Path;
                }
                Meta::NameValue(m) => {
                    k = m.path.get_ident().to_token_stream().to_string();
                    v = m.value.to_token_stream().to_string();
                    ty = AttrParseTy::NameValue;
                }
                Meta::List(m) => {
                    k = m.path.get_ident().to_token_stream().to_string();
                    v = m.tokens.to_string();
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
