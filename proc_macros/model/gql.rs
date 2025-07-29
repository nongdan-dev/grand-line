use crate::prelude::*;
use syn::Field;

pub fn gql_fields(
    fields: &Vec<Field>,
    virtuals: &Vec<Box<dyn GenVirtual>>,
) -> (
    Vec<TokenStream2>,
    Vec<TokenStream2>,
    Vec<TokenStream2>,
    Vec<TokenStream2>,
) {
    let (mut struk, mut resolver, mut into, mut select) = (vec![], vec![], vec![], vec![]);
    for f in fields {
        let name = f.ident.to_token_stream();
        let gql_name = camel_str!(name);
        let ty = f.ty.to_token_stream();
        let (opt, _) = unwrap_option(&ty);
        push_struk_resolver(&name, &ty, &mut struk, &mut resolver);

        into.push(if opt {
            quote! {
                #name: v.#name,
            }
        } else {
            quote! {
                #name: Some(v.#name),
            }
        });

        let sql_dep = virtuals
            .iter()
            .map(|v| v.sql_dep())
            .enumerate()
            .filter(|(_, v)| **v == str!(name))
            .map(|(i, _)| i)
            .collect::<Vec<_>>();
        let mut gql = virtuals
            .iter()
            .map(|v| v.gql_name())
            .enumerate()
            .filter(|(i, _)| sql_dep.contains(i))
            .map(|(_, v)| camel_str!(v))
            .collect::<Vec<_>>();
        gql.push(gql_name);
        let gql = gql
            .iter()
            .collect::<HashSet<_>>()
            .into_iter()
            .collect::<Vec<_>>();
        let column = pascal!(name.to_token_stream());
        select.push(quote! {
            #(m.insert(#gql, Column::#column);)*
        });
    }

    (struk, resolver, into, select)
}

pub fn gql_exprs(
    exprs: &Vec<Vec<Attr>>,
) -> (Vec<TokenStream2>, Vec<TokenStream2>, Vec<TokenStream2>) {
    let (mut struk, mut resolver, mut select_as) = (vec![], vec![], vec![]);

    for e in exprs {
        let a = e
            .iter()
            .find(|a| a.attr == VirtualTy::SqlExpr)
            .expect("select_as attr == VirtualTy::SqlExpr");
        let (name, ty) = (ts2!(a.field_name()), ts2!(a.field_ty()));
        push_struk_resolver(&name, &ty, &mut struk, &mut resolver);
        let sql_expr = a.sql_expr.as_ref().expect("select_as sql_expr unwrap");
        select_as.push(ts2!(sql_expr));
    }

    (struk, resolver, select_as)
}

fn push_struk_resolver(
    name: &TokenStream2,
    ty: &TokenStream2,
    struk: &mut Vec<TokenStream2>,
    resolver: &mut Vec<TokenStream2>,
) {
    let gql_name = camel_str!(name);
    let (opt, uw) = unwrap_option(ty);

    struk.push(quote! {
        pub #name: Option<#uw>,
    });

    let res = if opt {
        quote!(v)
    } else {
        quote!(v.unwrap_or_default())
    };
    resolver.push(quote! {
        // TODO: copy #[graphql...] and comments from the original field
        #[graphql(name=#gql_name)]
        pub async fn #name(&self) -> #ty {
            let v = self.#name.clone();
            #res
        }
    });
}
