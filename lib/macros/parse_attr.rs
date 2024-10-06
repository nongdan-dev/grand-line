#[macro_export]
macro_rules! parse_attr {
    ($attr:ident) => {{
        use crate::prelude::*;
        use quote::ToTokens;
        use syn::{meta::parser, parse_macro_input, LitStr, LitBool};

        let mut a: MacroAttr = Default::default();
        let mut first = false;

        let attr_parser = parser(|m| {
            let mut found = false;
            if !first {
                a.model = m.path.to_token_stream().to_string();
                found = true;
                first = true;
            }
            parse_attr_path!(m, found, a.meta, LitStr);
            parse_attr_path!(m, found, a.no_builtin, LitBool);
            parse_attr_path!(m, found, a.no_macro, LitBool);
            parse_attr_path!(m, found, a.resolver_inputs, LitBool);
            parse_attr_path!(m, found, a.resolver_output, LitBool);
            parse_attr_path!(m, found, a.no_tx, LitBool);
            parse_attr_path!(m, found, a.no_count, LitBool);
            if !found {
                panic!("unknown attribute {}", m.path.to_token_stream().to_string());
            }
            Ok(())
        });
        parse_macro_input!($attr with attr_parser);

        a
    }};
}

#[macro_export]
macro_rules! parse_attr_path {
    ($m:ident, $found:ident, $a:ident.$path:ident, $lit:ident $(, $wrap:ident)?) => {
        if $m.path.is_ident(stringify!($path)) {
            fn err() {
                let path = stringify!($path);
                let lit = stringify!($lit).to_owned().replacen("Lit", "", 1).to_lowercase();
                panic!("{} attribute must be a {} literal", path, lit);
            }
            let v = $m.value();
            if !v.is_ok() {
                err();
            }
            let v = v.unwrap().parse::<$lit>();
            if !v.is_ok() {
                err();
            }
            $a.$path = $($wrap)?(v.unwrap().value());
            $found = true;
        }
    };
    ($m:ident, $found:ident, $a:ident.$path:ident, $wrap:ident($lit:ident)) => {
        parse_attr_path!($m, $found, $a.$path, $lit, $wrap)
    };
}

#[macro_export]
macro_rules! parse_attr_check {
    ($a:ident.$path:ident, $arr:expr) => {
        let v = $a.clone().$path;
        parse_attr_check!($path, v, $arr);
    };
    ($wrap:ident($a:ident.$path:ident), $arr:expr) => {
        if let $wrap(v) = $a.clone().$path {
            parse_attr_check!($path, v, $arr);
        }
    };
    ($path:ident, $v:ident, $arr:expr) => {
        let arr = $arr;
        if !arr.contains(&&$v[..]) {
            panic!(
                "{} attribute must be one of {}",
                stringify!($path),
                arr.join(", ")
            );
        }
    };
}
