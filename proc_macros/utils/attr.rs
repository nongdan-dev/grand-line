use std::{any::type_name, str::FromStr};

use crate::prelude::*;
use syn::{
    Attribute, Meta, Result, Token,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
};

pub struct Attr {
    apply_on: String,
    name: String,
    args: HashMap<String, String>,
}

impl Attr {
    fn new(apply_on: &str, name: &str, args: Vec<(String, String)>) -> Self {
        let mut a = Self {
            apply_on: str!(apply_on),
            name: str!(name),
            args: HashMap::new(),
        };
        for (k, v) in args {
            if a.args.contains_key(&k) {
                a.panic(&strf!("key {} appear more than 1", k))
            }
            a.args.insert(k, v);
        }
        a
    }

    pub fn from_syn(apply_on: &str, a: &Attribute) -> Self {
        let mut args = Vec::<(String, String)>::new();
        let _ = a.parse_nested_meta(|m| {
            let k = str!(m.path.get_ident().unwrap());
            let mut v = str!();
            if m.input.peek(syn::Token![=]) {
                v = str!(m.value()?);
            }
            args.push((k, v));
            Ok(())
        });
        let name = str!(a.path().to_token_stream());
        Self::new(apply_on, &name, args)
    }
    pub fn from_proc(proc: ProcMacroAttr, apply_on: &str, name: &str) -> Self {
        Self::new(apply_on, name, proc.args)
    }

    pub fn has(&self, k: &str) -> bool {
        self.args.contains_key(k)
    }

    pub fn bool(&self, k: &str) -> Option<bool> {
        match self.args.get(k) {
            Some(v) => {
                if v == "" {
                    None
                } else if v == "0" || v == "false" {
                    Some(false)
                } else {
                    Some(true)
                }
            }
            None => None,
        }
    }
    pub fn must_bool(&self, k: &str) -> bool {
        match self.bool(k) {
            Some(v) => v,
            None => self.panic_key_not_found(k),
        }
    }

    pub fn str(&self, k: &str) -> Option<String> {
        self.args.get(k).cloned()
    }
    pub fn must_str(&self, k: &str) -> String {
        match self.str(k) {
            Some(v) => v,
            None => self.panic_key_not_found(k),
        }
    }

    pub fn must_parse<T>(&self, k: &str) -> T
    where
        T: FromStr,
    {
        match self.args.get(k) {
            Some(v) => match v.parse::<T>() {
                Ok(v) => v,
                Err(_) => self.panic_key(k, strf!("failed to parse as {}", type_name::<T>())),
            },
            None => self.panic_key_not_found(k),
        }
    }

    pub fn panic_key(&self, k: &str, e: impl Display) -> ! {
        self.panic(&strf!("key={} {}", k, e))
    }
    fn panic_key_not_found(&self, k: &str) -> ! {
        self.panic_key(k, "not found")
    }
}

impl DebugPanic for Attr {
    fn debug(&self) -> String {
        strf!("{} attr={}", self.apply_on, self.name)
    }
}

pub struct ProcMacroAttr {
    args: Vec<(String, String)>,
}

impl Parse for ProcMacroAttr {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut args = Vec::new();
        for m in Punctuated::<Meta, Token![,]>::parse_terminated(input)? {
            match m {
                Meta::Path(m) => {
                    let k = str!(m.get_ident().unwrap());
                    let v = str!();
                    args.push((k, v));
                }
                Meta::NameValue(m) => {
                    let k = str!(m.path.get_ident().unwrap());
                    let v = str!(m.value.to_token_stream());
                    args.push((k, v));
                }
                _ => {}
            }
        }
        Ok(ProcMacroAttr { args })
    }
}
