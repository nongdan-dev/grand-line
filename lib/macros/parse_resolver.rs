#[macro_export]
macro_rules! parse_resolver {
    (q, $item:ident) => {
        parse_resolver!(q, $item, "")
    };
    (m, $item:ident) => {
        parse_resolver!(m, $item, "")
    };
    (q, $item:ident, $name_default:expr) => {
        parse_resolver!(ty_query, $item, $name_default)
    };
    (m, $item:ident, $name_default:expr) => {
        parse_resolver!(ty_mutation, $item, $name_default)
    };
    ($ty:ident, $item:ident, $name_default:expr) => {{
        use crate::prelude::*;
        use heck::ToSnakeCase;
        use proc_macro2::TokenStream as TokenStream2;
        use quote::{ToTokens, format_ident, quote};
        use syn::{ItemFn, ReturnType, parse_macro_input};

        let ifn = parse_macro_input!($item as ItemFn);
        let name_default = format!("{}", $name_default);

        let mut name_graphql = ifn.sig.ident.to_string();
        if name_graphql == "resolver" {
            if name_default == "" {
                panic!("resolver name must be different than the reserved keyword `resolver`");
            }
            name_graphql = name_default;
        }
        let name = format_ident!("{}", name_graphql.to_snake_case()).to_token_stream();

        let inputs = ifn.sig.inputs.to_token_stream();
        let output: TokenStream2 = if let ReturnType::Type(_, ref ty) = ifn.sig.output {
            ty.to_token_stream()
        } else {
            "()".parse().unwrap()
        };

        let body = ifn.block.stmts;
        let body = quote! { #(#body)* };

        let ty = $ty(&name).to_token_stream();

        GenResolver {
            ty,
            name,
            name_graphql,
            inputs,
            output,
            body,
            ..Default::default()
        }
    }};
}
