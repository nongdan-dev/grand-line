#[macro_export]
macro_rules! parse_resolver {
    ($ty:ident, $item:ident) => {
        parse_resolver!($ty, $item, "")
    };
    ($ty:ident, $item:ident, $name_default:expr) => {{
        use crate::prelude::*;
        use syn::{ItemFn, ReturnType, parse_macro_input};

        let ifn = parse_macro_input!($item as ItemFn);
        let mut gql_name = str!(ifn.sig.ident);
        let name_default_str = str!($name_default);
        if gql_name == "resolver" {
            if name_default_str == "" {
                panic!("resolver name must be different than the reserved keyword `resolver`");
            }
            gql_name = name_default_str;
        }
        let name = snake!(gql_name);

        let inputs = ifn.sig.inputs.to_token_stream();
        let output = if let ReturnType::Type(_, ref ty) = ifn.sig.output {
            ty.to_token_stream()
        } else {
            ts2!("()")
        };

        let body = ifn.block.stmts;
        let body = quote!(#(#body)*);

        GenResolver {
            ty: $ty(&name),
            name,
            gql_name,
            inputs,
            output,
            body,
            ..Default::default()
        }
    }};
}
