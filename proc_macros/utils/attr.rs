#![allow(dead_code)]

use crate::prelude::*;
use std::any::type_name;
use std::str::FromStr;
use syn::{
    Attribute, Field, parenthesized,
    token::{Eq, Paren},
};

static RAW_ATTR: LazyLock<HashSet<String>> = LazyLock::new(|| {
    let mut set = HashSet::new();
    set.insert(str!(VirtualTy::SqlExpr));
    set
});

#[derive(Clone)]
pub struct Attr {
    /// In proc macro this is empty.
    /// In field, this will be `Model.field`.
    pub debug: String,
    /// In proc macro this is the macro name.
    /// In field, this will be attribute from our derive macro GrandLineModel.
    pub attr: String,
    pub args: HashMap<String, (String, AttrTy)>,
    /// Only in proc macro like #crud[Model, ...].
    /// The first path will be the model name.
    first_path: Option<String>,
    /// Only in field.
    field: Option<(String, Attribute, Field)>,
    /// Only in attribute #[sql_expr(...)].
    raw: Option<String>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum AttrTy {
    Path,
    NameValue,
    List,
}

impl Attr {
    fn init(debug: &str, attr: &str, args: Vec<(String, (String, AttrTy))>) -> Self {
        let mut a = Self {
            debug: str!(debug),
            attr: str!(attr),
            args: HashMap::new(),
            first_path: None,
            field: None,
            raw: None,
        };
        for (k, v) in args {
            if a.args.contains_key(&k) {
                panic_with_location!(a.msgk(&k, "appears more than once"));
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

    pub fn from_field(model: &str, f: &Field) -> Vec<Self> {
        f.attrs
            .iter()
            .map(|a| Self::from_field_attr(model, f, a))
            .collect::<Vec<_>>()
    }
    fn from_field_attr(model: &str, f: &Field, a: &Attribute) -> Self {
        let attr = str!(a.path().to_token_stream());
        let debug = strf!("`{}.{}`", model, f.ident.to_token_stream());
        let field = Some((str!(model), a.clone(), f.clone()));
        if RAW_ATTR.contains(&attr) {
            let err = strf!("should match syntax #[{}(some_thing)]", attr);
            let raw = a
                .meta
                .to_token_stream()
                .to_string()
                .trim()
                .strip_prefix(&attr)
                .unwrap_or_else(|| panic_with_location!(err))
                .trim()
                .strip_prefix("(")
                .unwrap_or_else(|| panic_with_location!(err))
                .strip_suffix(")")
                .unwrap_or_else(|| panic_with_location!(err))
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
        let mut args = Vec::<(String, (String, AttrTy))>::new();
        let mut first = true;
        let mut first_path = None;
        let _ = a.parse_nested_meta(|m| {
            let k = str!(
                m.path
                    .get_ident()
                    .unwrap_or_else(|| bug!("failed to get ident from attr meta path"))
            );
            let (v, ty);
            if m.input.peek(Eq) {
                v = str!(m.value()?);
                ty = AttrTy::NameValue;
            } else if m.input.peek(Paren) {
                let nested;
                parenthesized!(nested in m.input);
                v = str!(nested.parse::<TokenStream2>()?);
                ty = AttrTy::List;
            } else {
                v = str!();
                ty = AttrTy::Path;
            }
            if first && ty == AttrTy::Path {
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
                    panic_with_location!(self.msg(&strf!("model `{}` is not pascal case", v)));
                }
                v
            }
            None => {
                panic_with_location!(
                    self.msg(&strf!("missing model `#[{}(Model, ...)]`", self.attr))
                );
            }
        }
    }

    pub fn bool(&self, k: &str) -> Option<bool> {
        match self.args.get(k) {
            Some((_, AttrTy::Path)) => Some(true),
            Some((v, AttrTy::NameValue)) => match v == "0" {
                true => Some(false),
                false => panic_with_location!(self.msg_bool(k)),
            },
            Some(_) => panic_with_location!(self.msg_bool(k)),
            None => None,
        }
    }
    pub fn bool_must(&self, k: &str) -> bool {
        match self.bool(k) {
            Some(v) => v,
            None => panic_with_location!(self.msg_404(k)),
        }
    }

    pub fn str(&self, k: &str) -> Option<String> {
        match self.args.get(k) {
            Some((v, AttrTy::NameValue)) => match !(v.starts_with("\"") || v.starts_with("r#")) {
                true => Some(str!(v)),
                false => panic_with_location!(self.msg_str(k)),
            },
            Some(_) => panic_with_location!(self.msg_str(k)),
            None => None,
        }
    }
    pub fn str_must(&self, k: &str) -> String {
        match self.str(k) {
            Some(v) => v,
            None => panic_with_location!(self.msg_404(k)),
        }
    }

    pub fn parse<T>(&self, k: &str) -> Option<T>
    where
        T: FromStr,
    {
        match self.args.get(k) {
            Some((v, AttrTy::NameValue)) => match v.parse::<T>() {
                Ok(v) => Some(v),
                Err(_) => {
                    panic_with_location!(
                        self.msgk(k, &strf!("failed to parse `{}` as {}", v, type_name::<T>()))
                    );
                }
            },
            Some(_) => panic_with_location!(self.msg_str(k)),
            None => None,
        }
    }
    pub fn parse_must<T>(&self, k: &str) -> T
    where
        T: FromStr,
    {
        match self.parse(k) {
            Some(v) => v,
            None => panic_with_location!(self.msg_404(k)),
        }
    }

    fn field(&self) -> (String, Attribute, Field) {
        match self.field.to_owned() {
            Some(v) => v,
            None => bug!(self.msg("field: None")),
        }
    }
    pub fn field_model(&self) -> String {
        self.field().0
    }
    pub fn field_attr(&self) -> Attribute {
        self.field().1
    }
    pub fn field_name(&self) -> String {
        str!(self.field().2.ident.to_token_stream())
    }
    pub fn field_ty(&self) -> String {
        str!(self.field().2.ty.to_token_stream())
    }

    pub fn raw(&self) -> String {
        match self.raw.to_owned() {
            Some(v) => v,
            None => bug!(self.msg("raw: None")),
        }
    }

    pub fn into_with_validate<T>(self) -> T
    where
        T: From<Self> + AttrValidate,
    {
        let map = T::attr_fields(&self).into_iter().collect::<HashSet<_>>();
        for (k, _) in self.args.clone() {
            if !map.contains(&k) {
                panic_with_location!(self.msg_incorrect(&k));
            }
        }
        self.into()
    }

    pub fn msg_incorrect(&self, k: &str) -> String {
        self.msgk(&k, "should not be here")
    }
    pub fn msg_404(&self, k: &str) -> String {
        self.msgk(k, "not found")
    }
    pub fn msg_str(&self, k: &str) -> String {
        self.msgk(k, &strf!("should be `{} = some_value`", k))
    }
    pub fn msg_bool(&self, k: &str) -> String {
        let err = strf!("should be `{}` for true, or `{} = 0` for false", k, k);
        self.msgk(k, &err)
    }
    pub fn msgk(&self, k: &str, err: &str) -> String {
        let err = strf!("key `{}` {}", k, err);
        self.msg(&err)
    }
}

impl DebugPrefix for Attr {
    fn debug(&self) -> String {
        if self.debug == "" {
            strf!("macro `{}`:", self.attr)
        } else {
            strf!("{} attr `{}`:", self.debug, self.attr)
        }
    }
}

pub trait AttrValidate {
    fn attr_fields(a: &Attr) -> Vec<String>;
}
