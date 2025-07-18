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
            // TODO: check condition if it is not in the correct macro
            // for example no_count is only available in the search query
            parse_attr_path!(m, found, a.no_created_at, LitBool);
            parse_attr_path!(m, found, a.no_updated_at, LitBool);
            parse_attr_path!(m, found, a.no_deleted_at, LitBool);
            parse_attr_path!(m, found, a.no_by_id, LitBool);
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
    ($m:ident, $found:ident, $a:ident.$path:ident, $lit:ident) => {
        if $m.path.is_ident(stringify!($path)) {
            fn err_invalid() {
                let path = stringify!($path);
                let lit = stringify!($lit)
                    .to_owned()
                    .replacen("Lit", "", 1)
                    .to_lowercase();
                panic!("{} attribute must be a {} literal", path, lit);
            }
            let v = $m.value();
            if !v.is_ok() {
                err_invalid();
            }
            let v = v.unwrap().parse::<$lit>();
            if !v.is_ok() {
                err_invalid();
            }
            $a.$path = v.unwrap().value();
            $found = true;
        }
    };
}
