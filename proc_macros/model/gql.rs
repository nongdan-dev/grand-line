use crate::prelude::*;
use syn::Field;

pub fn gql_fields(
    gfields: &Vec<(Field, Vec<Attr>)>,
) -> (
    Vec<TokenStream2>,
    Vec<TokenStream2>,
    Vec<TokenStream2>,
    Vec<TokenStream2>,
) {
    let (mut struk, mut resolver, mut try_unwrap, mut into) = (vec![], vec![], vec![], vec![]);

    for (f, _) in gfields {
        let name = f.ident.to_token_stream();
        let ty = f.ty.to_token_stream();
        let (opt, _) = unwrap_option_str(&ty);
        push_struk_resolver(&name, &ty, &mut struk, &mut resolver);

        if !opt {
            let try_name = ts2!("try_", name);
            let msg_str = strf!(
                "{} should already be selected from database using graphql look ahead",
                name
            );
            try_unwrap.push(quote! {
                pub fn #try_name(&self) -> Result<#ty, Box<dyn Error + Send + Sync>> {
                    self
                    .#name
                    .clone()
                    .ok_or(#msg_str.into())
                }
            });
        }

        into.push(if opt {
            quote! {
                #name: v.#name,
            }
        } else {
            quote! {
                #name: Some(v.#name),
            }
        });
    }

    (struk, resolver, try_unwrap, into)
}

pub fn gql_virtuals(virs: &Vec<Box<dyn VirtualGen>>) -> Vec<TokenStream2> {
    let mut select = vec![];
    for v in virs {
        for sql in v.sql_dep() {
            let gql = v.gql_name();
            let col = pascal!(sql);
            select.push(quote! {
                m.entry(#gql).or_insert_with(Vec::new).push(Column::#col);
            });
        }
    }
    select
}

pub fn gql_exprs(
    exprs: &Vec<Vec<Attr>>,
) -> (Vec<TokenStream2>, Vec<TokenStream2>, Vec<TokenStream2>) {
    let (mut struk, mut resolver, mut select_as) = (vec![], vec![], vec![]);

    for e in exprs {
        let a = e
            .iter()
            .find(|a| a.attr == VirtualTy::SqlExpr)
            .unwrap_or_else(|| bug!("failed to find VirtualTy::SqlExpr to build select as"));
        let name_str = a.field_name();
        let name = ts2!(name_str);
        let ty = ts2!(a.field_ty());
        push_struk_resolver(&name, &ty, &mut struk, &mut resolver);

        let gql_name = camel_str!(name);
        let sql_expr = ts2!(a.raw());
        select_as.push(quote!(m.insert(#gql_name, (#name_str, #sql_expr));));
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
