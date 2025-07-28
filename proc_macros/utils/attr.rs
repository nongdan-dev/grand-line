use crate::prelude::*;
use std::{any::type_name, str::FromStr};
use syn::{
    Attribute, Field,
    token::{Eq, Paren},
};

#[derive(Debug, Clone)]
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
    pub first_path: String,
    /// Only in field.
    pub field: Option<(String, Field)>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum AttrTy {
    Path,
    NameValue,
    List,
}

#[allow(dead_code)]
impl Attr {
    fn new(
        debug: &str,
        attr: &str,
        args: Vec<(String, (String, AttrTy))>,
        first_path: &str,
    ) -> Self {
        let mut a = Self {
            debug: str!(debug),
            attr: str!(attr),
            args: HashMap::new(),
            first_path: str!(first_path),
            field: None,
        };
        for (k, v) in args {
            if a.args.contains_key(&k) {
                a.panic_key(&k, "appears more than once")
            }
            a.args.insert(k, v);
        }
        a
    }

    pub fn from_proc_macro(name: &str, a: AttrParse) -> Self {
        Self::new("", name, a.args, &a.first_path)
    }

    pub fn from_field(model: &str, f: &Field) -> Vec<Self> {
        f.attrs
            .iter()
            .map(|a| Self::from_field_attr(model, f, a))
            .collect::<Vec<_>>()
    }
    fn from_field_attr(model: &str, field: &Field, a: &Attribute) -> Self {
        let mut args = Vec::<(String, (String, AttrTy))>::new();
        let mut first = true;
        let mut first_path = str!();
        let _ = a.parse_nested_meta(|m| {
            let k = str!(m.path.get_ident().unwrap());
            let (v, ty);
            if m.input.peek(Eq) {
                v = str!(m.value()?);
                ty = AttrTy::NameValue;
            } else if m.input.peek(Paren) {
                v = str!("TODO:");
                ty = AttrTy::List;
            } else {
                v = str!();
                ty = AttrTy::Path;
            }
            if first && ty == AttrTy::Path {
                first_path = k.clone();
            }
            args.push((k, (v, ty)));
            first = false;
            Ok(())
        });
        let debug = strf!("`{}.{}`", model, field.ident.to_token_stream());
        let attr = str!(a.path().to_token_stream());
        let mut a = Self::new(&debug, &attr, args, &first_path);
        a.field = Some((str!(model), field.clone()));
        a
    }

    pub fn is(&self, attr: &str) -> bool {
        self.attr == attr
    }
    pub fn has(&self, k: &str) -> bool {
        self.args.contains_key(k)
    }

    pub fn model_from_first_path(&self) -> String {
        let model = self.first_path.clone();
        if model == "" {
            let err = strf!("missing model `#[{}(Model, ...)]`", self.attr);
            self.panic(&err);
        }
        if model != pascal_str!(model) {
            let err = strf!("model `{}` is not pascal case", model);
            self.panic(&err);
        }
        model
    }

    pub fn field(&self) -> (String, Field) {
        match self.field.clone() {
            Some(v) => v,
            None => self.panic_framework_bug("field none"),
        }
    }
    pub fn field_model(&self) -> String {
        self.field().0
    }
    pub fn field_name(&self) -> String {
        str!(self.field().1.ident.to_token_stream())
    }
    pub fn field_ty(&self) -> String {
        str!(self.field().1.ty.to_token_stream())
    }

    pub fn bool(&self, k: &str) -> Option<bool> {
        match self.args.get(k) {
            Some((_, AttrTy::Path)) => Some(true),
            Some((v, AttrTy::NameValue)) => match v == "0" {
                true => Some(false),
                false => self.panic_invalid_bool(k),
            },
            Some(_) => self.panic_invalid_bool(k),
            None => None,
        }
    }
    pub fn bool_must(&self, k: &str) -> bool {
        match self.bool(k) {
            Some(v) => v,
            None => self.panic_notfound(k),
        }
    }

    pub fn str(&self, k: &str) -> Option<String> {
        match self.args.get(k) {
            Some((v, AttrTy::NameValue)) => match !(v.starts_with("\"") || v.starts_with("r#")) {
                true => Some(str!(v)),
                false => self.panic_invalid(k),
            },
            Some(_) => self.panic_invalid(k),
            None => None,
        }
    }
    pub fn str_must(&self, k: &str) -> String {
        match self.str(k) {
            Some(v) => v,
            None => self.panic_notfound(k),
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
                    let err = strf!("failed to parse `{}` as {}", v, type_name::<T>());
                    self.panic_key(k, &err)
                }
            },
            Some(_) => self.panic_invalid(k),
            None => None,
        }
    }
    pub fn parse_must<T>(&self, k: &str) -> T
    where
        T: FromStr,
    {
        match self.parse(k) {
            Some(v) => v,
            None => self.panic_notfound(k),
        }
    }

    pub fn into_with_validate<T>(self) -> T
    where
        T: From<Self> + AttrValidate,
    {
        let map = T::attr_fields(&self).into_iter().collect::<HashSet<_>>();
        for (k, _) in self.args.clone() {
            if !map.contains(&k) {
                self.panic_incorrect(&k)
            }
        }
        self.into()
    }

    pub fn panic_notfound(&self, k: &str) -> ! {
        self.panic_key(k, "not found")
    }
    pub fn panic_incorrect(&self, k: &str) -> ! {
        self.panic_key(&k, "should not be here")
    }
    pub fn panic_invalid(&self, k: &str) -> ! {
        let err = strf!("should be `{} = some_value`", k);
        self.panic_key(k, &err)
    }
    pub fn panic_invalid_bool(&self, k: &str) -> ! {
        let err = strf!("should be `{}` for true, or `{} = 0` for false", k, k);
        self.panic_key(k, &err)
    }
    pub fn panic_key(&self, k: &str, err: &str) -> ! {
        let err = strf!("key `{}` {}", k, err);
        self.panic(&err)
    }

    pub fn panic_framework_bug(&self, err: &str) -> ! {
        let err = strf!("SHOULD NOT HAPPEN, FRAMEWORK BUG: {}", err);
        self.panic(&err)
    }
}

impl DebugPanic for Attr {
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
