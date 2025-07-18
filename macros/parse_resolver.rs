#[macro_export]
macro_rules! parse_resolver {
    ($ty:ident, $item:ident) => {
        parse_resolver!($ty, $item, "")
    };
    ($ty:ident, $item:ident, $name_default:expr) => {{
        use crate::prelude::*;
        use heck::ToSnakeCase;
        use proc_macro2::TokenStream as TokenStream2;
        use quote::{ToTokens, quote};
        use syn::{ItemFn, ReturnType, parse_macro_input};

        let ifn = parse_macro_input!($item as ItemFn);
        let name_default = $name_default.to_string();

        let mut gql_name = ifn.sig.ident.to_string();
        if gql_name == "resolver" {
            if name_default == "" {
                panic!("resolver name must be different than the reserved keyword `resolver`");
            }
            gql_name = name_default;
        }
        let name = gql_name.to_snake_case().parse::<TokenStream2>().unwrap();

        let inputs = ifn.sig.inputs.to_token_stream();
        let output = if let ReturnType::Type(_, ref ty) = ifn.sig.output {
            ty.to_token_stream()
        } else {
            "()".parse::<TokenStream2>().unwrap()
        };

        let body = ifn.block.stmts;
        let body = quote! { #(#body)* };

        let ty = $ty(&name).to_token_stream();

        GenResolver {
            ty,
            name,
            gql_name,
            inputs,
            output,
            body,
            ..Default::default()
        }
    }};
}
