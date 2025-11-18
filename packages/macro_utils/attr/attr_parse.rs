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
    pub fn from_meta_list_token_stream(ts: Ts2) -> Self {
        let metas = if ts.to_string().trim().is_empty() {
            vec![]
        } else {
            let x = &ts;
            Punctuated::<Meta, Token![,]>::parse_terminated
                .parse2(ts.clone())
                .unwrap_or_else(|e| {
                    panic!("failed to parse meta list from token stream `{x}`: {e}")
                })
                .into_iter()
                .collect()
        };
        AttrParse::from_meta_list(metas)
    }
    pub fn from_meta_list(metas: Vec<Meta>) -> Self {
        let mut args = Vec::new();
        let mut first = true;
        let mut first_path = None;
        for m in metas {
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
        Self { args, first_path }
    }
}

impl Parse for AttrParse {
    fn parse(s: ParseStream) -> Result<Self> {
        let metas = Punctuated::<Meta, Token![,]>::parse_terminated(s)?
            .into_iter()
            .collect();
        let a = Self::from_meta_list(metas);
        Ok(a)
    }
}
