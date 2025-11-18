use crate::prelude::*;
use std::any::type_name;
use std::str::FromStr;

#[derive(Clone)]
pub struct Attr {
    /// In proc macro, this is empty.
    /// In field, this will be Model.field.
    debug: String,
    /// In proc macro, this is the macro name.
    /// In field, this will be one of AttrTy.
    pub attr: String,
    /// Raw args parsed as strings
    args: HashMap<String, (String, AttrParseTy)>,
    /// Only in proc macro like crud(Model, ...).
    /// The first path will be the model name.
    first_path: Option<String>,
    /// Only in field.
    field: Option<(String, Attribute, Field)>,
    /// Only in attr such as #[default(..)], #[sql_expr(..)], etc..
    raw: Option<String>,
}

impl Attr {
    fn init(debug: &str, attr: &str, args: Vec<(String, (String, AttrParseTy))>) -> Self {
        let mut a = Self {
            debug: debug.to_owned(),
            attr: attr.to_owned(),
            args: HashMap::new(),
            first_path: None,
            field: None,
            raw: None,
        };
        for (k, v) in args {
            if a.args.contains_key(&k) {
                a.panic_by_key(&k, "appears more than once");
            }
            a.args.insert(k, v);
        }
        a
    }

    pub fn from_proc_macro(macro_name: &str, a: AttrParse) -> Self {
        let mut r = Self::init("", macro_name, a.args);
        r.first_path = a.first_path;
        r
    }
    pub fn from_ts2(debug: &str, attr: &str, ts: Ts2) -> Self {
        let a = AttrParse::from_meta_list_token_stream(ts);
        let mut r = Self::init(debug, attr, a.args);
        r.first_path = a.first_path;
        r
    }
    pub fn from_ts2_into<V>(debug: &str, attr: &str, ts: Ts2) -> V
    where
        V: From<Self> + AttrValidate,
    {
        Self::from_ts2(debug, attr, ts).into_with_validate()
    }

    pub fn from_field(model: &str, f: &Field, raw: &dyn Fn(&str) -> bool) -> Vec<Self> {
        f.attrs
            .iter()
            .map(|a| Self::from_field_attr(model, f, a, raw))
            .collect::<Vec<_>>()
    }
    fn from_field_attr(model: &str, f: &Field, a: &Attribute, raw: &dyn Fn(&str) -> bool) -> Self {
        let attr = a.path().to_token_stream().to_string();
        let field = f.ident.to_token_stream();
        let debug = format!("{model}.{field}");
        let field = Some((model.to_owned(), a.clone(), f.clone()));
        let mut r = if raw(&attr) {
            let mut r = Self::init(&debug, &attr, vec![]);
            r.raw = Some(match &a.meta {
                Meta::List(l) => l.tokens.to_string(),
                _ => panic!("raw attr should be meta list #[{attr}(some_value)]"),
            });
            r
        } else {
            match &a.meta {
                // #[attr(nested)]
                Meta::List(l) => Self::from_ts2(&debug, &attr, l.tokens.clone()),
                // Meta::Path(_) => #[attr] without any nested meta, args should be empty
                // Meta::NameValue(_) => #[attr = some_value] we are not using, args should be empty
                // there are case such as #[doc = "some_value"] then we should not panic
                _ => Self::init(&debug, &attr, vec![]),
            }
        };
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
                if v != v.to_pascal_case() {
                    let err = format!("model `{v}` is not pascal case");
                    self.panic(&err);
                }
                v
            }
            None => {
                let attr = &self.attr;
                let err = format!("missing model #[{attr}(Model, ...)]");
                self.panic(&err);
            }
        }
    }

    pub fn bool(&self, k: &str) -> Option<bool> {
        match self.args.get(k) {
            Some((_, AttrParseTy::Path)) => Some(true),
            Some((v, AttrParseTy::NameValue)) => match v == "false" {
                true => Some(false),
                false => self.panic_invalid_bool(k),
            },
            Some(_) => self.panic_invalid_bool(k),
            None => None,
        }
    }
    pub fn bool_or_panic(&self, k: &str) -> bool {
        match self.bool(k) {
            Some(v) => v,
            None => self.panic_required(k),
        }
    }
    pub fn bool_should_omit(&self, k: &str) -> bool {
        match self.bool(k) {
            Some(v) => {
                if !v {
                    self.panic_by_key(k, "should omit");
                }
                true
            }
            None => false,
        }
    }

    pub fn str(&self, k: &str) -> Option<String> {
        match self.args.get(k) {
            Some((v, AttrParseTy::NameValue)) => match parse2::<LitStr>(v.ts2_or_panic()) {
                Ok(v) => Some(v.value()),
                Err(_) => self.panic_invalid_string(k),
            },
            Some(_) => self.panic_invalid_string(k),
            None => None,
        }
    }
    pub fn str_or_panic(&self, k: &str) -> String {
        match self.str(k) {
            Some(v) => v,
            None => self.panic_required(k),
        }
    }

    pub fn nested(&self, k: &str) -> Option<String> {
        match self.args.get(k) {
            Some((v, AttrParseTy::List)) => Some(v.to_owned()),
            Some(_) => self.panic_invalid_nested(k),
            None => None,
        }
    }
    pub fn nested_or_panic(&self, k: &str) -> String {
        match self.nested(k) {
            Some(v) => v,
            None => self.panic_required(k),
        }
    }
    pub fn nested_into<V>(&self, k: &str) -> Option<V>
    where
        V: From<Self> + AttrValidate,
    {
        self.nested(k)
            .map(|v| Self::from_ts2_into(&self.attr_debug(), k, v.ts2_or_panic()))
    }

    pub fn nested_with_path(&self, k: &str) -> Option<(bool, String)> {
        match self.args.get(k) {
            Some((v, AttrParseTy::Path)) => Some((true, v.to_owned())),
            _ => self.nested(k).map(|v| (false, v)),
        }
    }
    pub fn nested_with_path_or_panic(&self, k: &str) -> (bool, String) {
        match self.nested_with_path(k) {
            Some(v) => v,
            None => self.panic_required(k),
        }
    }
    pub fn nested_with_path_into<V>(&self, k: &str) -> Option<(bool, V)>
    where
        V: From<Self> + AttrValidate,
    {
        self.nested_with_path(k).map(|(path, v)| {
            (
                path,
                if path {
                    Self::init(&self.attr_debug(), k, vec![]).into_with_validate()
                } else {
                    Self::from_ts2_into(&self.attr_debug(), k, v.ts2_or_panic())
                },
            )
        })
    }

    pub fn parse<V>(&self, k: &str) -> Option<V>
    where
        V: FromStr,
    {
        match self.args.get(k) {
            Some((v, AttrParseTy::NameValue)) => match v.parse::<V>() {
                Ok(v) => Some(v),
                Err(_) => {
                    let t = type_name::<V>();
                    let err = format!("cannot parse `{v}` as {t}");
                    self.panic_by_key(k, &err);
                }
            },
            Some(_) => {
                let err = format!("should be `{k} = some_value`");
                self.panic_by_key(k, &err);
            }
            None => None,
        }
    }
    pub fn parse_or_panic<V>(&self, k: &str) -> V
    where
        V: FromStr,
    {
        match self.parse(k) {
            Some(v) => v,
            None => self.panic_required(k),
        }
    }

    fn field(&self) -> (String, Attribute, Field) {
        match self.field.clone() {
            Some(v) => v,
            None => self.panic("field: None"),
        }
    }
    pub fn field_model(&self) -> String {
        self.field().0
    }
    pub fn field_attr(&self) -> Attribute {
        self.field().1
    }
    pub fn field_name(&self) -> String {
        self.field().2.ident.to_token_stream().to_string()
    }
    pub fn field_ty(&self) -> String {
        self.field().2.ty.to_token_stream().to_string()
    }

    pub fn raw(&self) -> String {
        match self.raw.clone() {
            Some(v) => v,
            None => self.panic("raw: None"),
        }
    }

    pub fn into_with_validate<V>(self) -> V
    where
        V: From<Self> + AttrValidate,
    {
        let attrs = V::attr_fields(&self);
        let map = attrs.iter().collect::<HashSet<_>>();
        for (k, _) in self.args.clone() {
            if !map.contains(&k) {
                self.panic_invalid(&k, &attrs);
            }
        }
        self.into()
    }

    pub fn panic_required(&self, k: &str) -> ! {
        let err = "is required";
        self.panic_by_key(k, err);
    }
    pub fn panic_invalid(&self, k: &str, valid: &[String]) -> ! {
        let valid = valid.join(", ");
        let err = format!("is not valid here, should be one of: {valid}");
        self.panic_by_key(k, &err);
    }
    pub fn panic_invalid_bool(&self, k: &str) -> ! {
        let err = format!("should be `{k}` for true, or `{k} = false` for false");
        self.panic_by_key(k, &err);
    }
    pub fn panic_invalid_string(&self, k: &str) -> ! {
        let err = format!(r#"should be `{k} = "some_value"` for string"#);
        self.panic_by_key(k, &err);
    }
    pub fn panic_invalid_nested(&self, k: &str) -> ! {
        let err = format!("should be `{k}(some_value)` for nested");
        self.panic_by_key(k, &err);
    }
    pub fn panic_by_key(&self, k: &str, err: &str) -> ! {
        let err = format!("key `{k}` {err}");
        self.panic(&err);
    }
}

impl AttrDebug for Attr {
    fn attr_debug(&self) -> String {
        let Attr { attr, debug, .. } = &self;
        if debug.is_empty() {
            format!("macro `{attr}`:")
        } else {
            format!("{debug} attr `{attr}`:")
        }
    }
}

pub trait AttrValidate {
    fn attr_fields(a: &Attr) -> Vec<String>;
}
