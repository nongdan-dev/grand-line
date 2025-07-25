use std::{any::type_name, str::FromStr};

use crate::prelude::*;
use syn::{Attribute, Field};

#[derive(Debug, Clone)]
pub struct Attr {
    pub debug: String,
    pub attr: String,
    pub args: HashMap<String, String>,
    pub model: String,
}

#[allow(dead_code)]
impl Attr {
    pub fn new(debug: &str, attr: &str, args: Vec<(String, String)>, model: &str) -> Self {
        let mut a = Self {
            debug: str!(debug),
            attr: str!(attr),
            args: HashMap::new(),
            model: str!(model),
        };
        for (k, v) in args {
            if a.args.contains_key(&k) {
                a.panic_key(&k, "appears more than once")
            }
            a.args.insert(k, v);
        }
        a
    }

    pub fn from_syn(debug: &str, a: &Attribute) -> Self {
        let mut args = Vec::<(String, String)>::new();
        let mut first = true;
        let mut model = str!();
        let _ = a.parse_nested_meta(|m| {
            let k = str!(m.path.get_ident().unwrap());
            let mut v = str!();
            if m.input.peek(syn::Token![=]) {
                v = str!(m.value()?);
            } else if first {
                // no value here, could be model
                model = k.clone();
            }
            args.push((k, v));
            first = false;
            Ok(())
        });
        let attr = str!(a.path().to_token_stream());
        Self::new(debug, &attr, args, &model)
    }
    pub fn from_field(debug: &str, f: &Field) -> Vec<Self> {
        let debug = strf!("{}.{}", debug, f.ident.to_token_stream());
        f.attrs
            .iter()
            .map(|a| Self::from_syn(&debug, a))
            .collect::<Vec<_>>()
    }

    pub fn has(&self, k: &str) -> bool {
        self.args.contains_key(k)
    }

    pub fn model_must(&self) -> String {
        let model = self.model.clone();
        if model == "" {
            self.panic(&strf!("missing model #[{}(Model, ...)]", self.attr));
        }
        if model != pascal_str!(model) {
            self.panic(&strf!("{} not pascal case Model", model));
        }
        model
    }

    pub fn bool(&self, k: &str) -> bool {
        self.bool_opt(k).unwrap_or_default()
    }
    pub fn bool_opt(&self, k: &str) -> Option<bool> {
        match self.args.get(k) {
            Some(v) => Some(!(v == "0" || v == "false")),
            None => None,
        }
    }
    pub fn bool_must(&self, k: &str) -> bool {
        match self.bool_opt(k) {
            Some(v) => v,
            None => self.panic_key(k, "not found"),
        }
    }

    pub fn str(&self, k: &str) -> String {
        self.str_opt(k).unwrap_or_default()
    }
    pub fn str_opt(&self, k: &str) -> Option<String> {
        self.args.get(k).cloned()
    }
    pub fn str_must(&self, k: &str) -> String {
        match self.str_opt(k) {
            Some(v) => v,
            None => self.panic_key(k, "not found"),
        }
    }

    pub fn parse<T>(&self, k: &str) -> T
    where
        T: FromStr + Default,
    {
        self.parse_opt(k).unwrap_or_default()
    }
    pub fn parse_opt<T>(&self, k: &str) -> Option<T>
    where
        T: FromStr,
    {
        match self.args.get(k) {
            Some(v) => match v.parse::<T>() {
                Ok(v) => Some(v),
                Err(_) => self.panic_key(k, &strf!("failed to parse as {}", type_name::<T>())),
            },
            None => None,
        }
    }
    pub fn parse_must<T>(&self, k: &str) -> T
    where
        T: FromStr,
    {
        match self.parse_opt(k) {
            Some(v) => v,
            None => self.panic_key(k, "not found"),
        }
    }

    pub fn panic_key(&self, k: &str, e: &str) -> ! {
        self.panic(&strf!("key={} {}", k, e))
    }

    pub fn into_with_validate<T>(self) -> T
    where
        T: From<Self> + AttrValidate,
    {
        let map = T::attr_fields(&self).into_iter().collect::<HashSet<_>>();
        for (k, _) in self.args.clone() {
            if !map.contains(&k) {
                self.panic_key(&k, "not included in this attribute")
            }
        }
        self.into()
    }
}

impl DebugPanic for Attr {
    fn debug(&self) -> String {
        strf!("{} macro attr={}", self.debug, self.attr)
    }
}

pub trait AttrValidate {
    fn attr_fields(a: &Attr) -> Vec<String>;
}
