#![allow(dead_code)]

use crate::prelude::*;
use std::any::type_name;
use std::str::FromStr;
use syn::{
    Attribute, Field, parenthesized,
    token::{Eq, Paren},
};

#[derive(Clone)]
pub struct Attr {
    /// In proc macro this is empty.
    /// In field, this will be `Model.field`.
    debug: String,
    /// In proc macro this is the macro name.
    /// In field, this will be one of attribute from AttrTy.
    pub attr: String,
    /// Raw args parsed as strings
    args: HashMap<String, (String, AttrParseTy)>,
    /// Only in proc macro like #crud[Model, ...].
    /// The first path will be the model name.
    first_path: Option<String>,
    /// Only in field.
    field: Option<(String, Attribute, Field)>,
    /// Only in attribute #[sql_expr(...)].
    raw: Option<String>,
}

impl Attr {
    fn init(debug: &str, attr: &str, args: Vec<(String, (String, AttrParseTy))>) -> Self {
        let mut a = Self {
            debug: s!(debug),
            attr: s!(attr),
            args: HashMap::new(),
            first_path: None,
            field: None,
            raw: None,
        };
        for (k, v) in args {
            if a.args.contains_key(&k) {
                let err = a.errk(&k, "appears more than once");
                pan!(err);
            }
            a.args.insert(k, v);
        }
        a
    }

    pub fn from_proc_macro(name: &str, a: AttrParse) -> Self {
        let mut r = Self::init("", name, a.args);
        r.first_path = a.first_path;
        r
    }

    pub fn from_field(model: &str, f: &Field, raw: &dyn Fn(&str) -> bool) -> Vec<Self> {
        f.attrs
            .iter()
            .map(|a| Self::from_field_attr(model, f, a, raw))
            .collect::<Vec<_>>()
    }
    fn from_field_attr(model: &str, f: &Field, a: &Attribute, raw: &dyn Fn(&str) -> bool) -> Self {
        let attr = s!(a.path().to_token_stream());
        let debug = f!("`{}.{}`", model, f.ident.to_token_stream());
        let field = Some((s!(model), a.clone(), f.clone()));
        if raw(&attr) {
            let err = f!("should match syntax #[{}(some_thing)]", attr);
            let raw = a
                .meta
                .to_token_stream()
                .to_string()
                .trim()
                .strip_prefix(&attr)
                .unwrap_or_else(|| pan!(err))
                .trim()
                .strip_prefix("(")
                .unwrap_or_else(|| pan!(err))
                .strip_suffix(")")
                .unwrap_or_else(|| pan!(err))
                .trim()
                .to_string();
            return Self {
                debug,
                attr,
                args: HashMap::new(),
                first_path: None,
                field,
                raw: Some(raw),
            };
        }
        let mut args = Vec::<(String, (String, AttrParseTy))>::new();
        let mut first = true;
        let mut first_path = None;
        let _ = a.parse_nested_meta(|m| {
            let k = s!(m.path.get_ident().to_token_stream());
            let (v, ty);
            if m.input.peek(Eq) {
                v = s!(m.value()?);
                ty = AttrParseTy::NameValue;
            } else if m.input.peek(Paren) {
                let nested;
                parenthesized!(nested in m.input);
                v = s!(nested.parse::<Ts2>()?);
                ty = AttrParseTy::List;
            } else {
                v = s!();
                ty = AttrParseTy::Path;
            }
            if first && ty == AttrParseTy::Path {
                first_path = Some(k.clone());
            }
            args.push((k, (v, ty)));
            first = false;
            Ok(())
        });
        let mut r = Self::init(&debug, &attr, args);
        r.first_path = first_path;
        r.field = field;
        r
    }

    pub fn is(&self, attr: &str) -> bool {
        self.attr == attr
    }
    pub fn has(&self, k: &str) -> bool {
        self.args.contains_key(k)
    }

    pub fn model_from_first_path(&self) -> String {
        match self.first_path.clone() {
            Some(v) => {
                if v != pascal_str!(v) {
                    let err = f!("model `{}` is not pascal case", v);
                    let err = self.err(&err);
                    pan!(err);
                }
                v
            }
            None => {
                let err = f!("missing model `#[{}(Model, ...)]`", self.attr);
                let err = self.err(&err);
                pan!(err);
            }
        }
    }

    pub fn bool(&self, k: &str) -> Option<bool> {
        match self.args.get(k) {
            Some((_, AttrParseTy::Path)) => Some(true),
            Some((v, AttrParseTy::NameValue)) => match v == "0" {
                true => Some(false),
                false => {
                    let err = self.err_bool(k);
                    pan!(err);
                }
            },
            Some(_) => {
                let err = self.err_bool(k);
                pan!(err);
            }
            None => None,
        }
    }
    pub fn bool_must(&self, k: &str) -> bool {
        match self.bool(k) {
            Some(v) => v,
            None => {
                let err = self.err_404(k);
                pan!(err);
            }
        }
    }

    pub fn str(&self, k: &str) -> Option<String> {
        match self.args.get(k) {
            Some((v, AttrParseTy::NameValue)) => {
                match !(v.starts_with("\"") || v.starts_with("r#")) {
                    true => Some(s!(v)),
                    false => {
                        let err = self.err_str(k);
                        pan!(err);
                    }
                }
            }
            Some(_) => {
                let err = self.err_str(k);
                pan!(err);
            }
            None => None,
        }
    }
    pub fn str_must(&self, k: &str) -> String {
        match self.str(k) {
            Some(v) => v,
            None => {
                let err = self.err_404(k);
                pan!(err);
            }
        }
    }

    pub fn parse<T>(&self, k: &str) -> Option<T>
    where
        T: FromStr,
    {
        match self.args.get(k) {
            Some((v, AttrParseTy::NameValue)) => match v.parse::<T>() {
                Ok(v) => Some(v),
                Err(_) => {
                    let err = f!("failed to parse `{}` as {}", v, type_name::<T>());
                    let err = self.errk(k, err);
                    pan!(err);
                }
            },
            Some(_) => {
                let err = self.err_str(k);
                pan!(err);
            }
            None => None,
        }
    }
    pub fn parse_must<T>(&self, k: &str) -> T
    where
        T: FromStr,
    {
        match self.parse(k) {
            Some(v) => v,
            None => {
                let err = self.err_404(k);
                pan!(err);
            }
        }
    }

    fn field(&self) -> (String, Attribute, Field) {
        match self.field.to_owned() {
            Some(v) => v,
            None => {
                let err = self.err("field: None");
                bug!(err);
            }
        }
    }
    pub fn field_model(&self) -> String {
        self.field().0
    }
    pub fn field_attr(&self) -> Attribute {
        self.field().1
    }
    pub fn field_name(&self) -> String {
        s!(self.field().2.ident.to_token_stream())
    }
    pub fn field_ty(&self) -> String {
        s!(self.field().2.ty.to_token_stream())
    }

    pub fn raw(&self) -> String {
        match self.raw.to_owned() {
            Some(v) => v,
            None => {
                let err = self.err("raw: None");
                bug!(err);
            }
        }
    }

    pub fn into_with_validate<T>(self) -> T
    where
        T: From<Self> + AttrValidate,
    {
        let map = T::attr_fields(&self).into_iter().collect::<HashSet<_>>();
        for (k, _) in self.args.clone() {
            if !map.contains(&k) {
                let err = self.err_incorrect(&k);
                pan!(err);
            }
        }
        self.into()
    }

    pub fn err_incorrect(&self, k: &str) -> String {
        let err = "should not be here";
        self.errk(k, err)
    }
    pub fn err_404(&self, k: &str) -> String {
        let err = "not found";
        self.errk(k, err)
    }
    pub fn err_bool(&self, k: &str) -> String {
        let err = f!("use `{}` for true, or `{} = 0` for false", k, k);
        self.errk(k, err)
    }
    pub fn err_str(&self, k: &str) -> String {
        let err = f!("use `{} = some_value` without quotes", k);
        self.errk(k, err)
    }
    pub fn errk(&self, k: &str, err: impl Display) -> String {
        let err = f!("key `{}` {}", k, err);
        self.err(&err)
    }
}

impl AttrDebug for Attr {
    fn attr_debug(&self) -> String {
        if self.debug == "" {
            f!("macro `{}`:", self.attr)
        } else {
            f!("{} attr `{}`:", self.debug, self.attr)
        }
    }
}

pub trait AttrValidate {
    fn attr_fields(a: &Attr) -> Vec<String>;
}
