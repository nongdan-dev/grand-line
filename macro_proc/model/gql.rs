use crate::prelude::*;
use syn::Field;

pub fn gql_fields(
    gfields: &Vec<(Field, Vec<Attr>)>,
) -> (Vec<Ts2>, Vec<Ts2>, Vec<Ts2>, Vec<Ts2>, Vec<Ts2>, Vec<Ts2>) {
    let (mut struk, mut resolver, mut into, mut cols, mut select, mut get_cols) =
        (vec![], vec![], vec![], vec![], vec![], vec![]);

    for (f, _) in gfields {
        let name = f.ident.to_token_stream();
        let ty = f.ty.to_token_stream();
        let (opt, uw_str) = unwrap_option_str(&ty);
        push_struk_resolver(&name, &ty, &mut struk, &mut resolver);

        let name_str = s!(name);
        let col = pascal!(name);
        cols.push(quote! {
            m.insert(#name_str, Column::#col);
        });

        let gql_name = camel_str!(name);
        select.push(quote! {
            m.entry(#gql_name).or_insert_with(HashSet::new).insert(#name_str);
        });

        into.push(if opt {
            quote! {
                #name: self.#name,
            }
        } else {
            quote! {
                #name: Some(self.#name),
            }
        });

        if uw_str == "String" {
            get_cols.push(quote! {
                Column::#col => self.#name.clone(),
            });
        }
    }

    (struk, resolver, into, cols, select, get_cols)
}

pub fn gql_virtuals(virs: &Vec<Box<dyn VirtualResolverFn>>) -> Vec<Ts2> {
    let mut select = vec![];
    for v in virs {
        let gql_name = v.gql_name();
        for name_str in v.sql_deps() {
            select.push(quote! {
                m.entry(#gql_name).or_insert_with(HashSet::new).insert(#name_str);
            });
        }
    }
    select
}

pub fn gql_exprs(exprs: &Vec<Vec<Attr>>) -> (Vec<Ts2>, Vec<Ts2>, Vec<Ts2>, Vec<Ts2>) {
    let (mut struk, mut resolver, mut select, mut gql_exprs) = (vec![], vec![], vec![], vec![]);

    for e in exprs {
        let a = e
            .iter()
            .find(|a| a.attr == VirtualTy::SqlExpr)
            .unwrap_or_else(|| {
                let err = "failed to find VirtualTy::SqlExpr to build select as";
                bug!(err);
            });
        let name_str = a.field_name();
        let name = ts2!(name_str);
        let ty = ts2!(a.field_ty());
        push_struk_resolver(&name, &ty, &mut struk, &mut resolver);

        let gql_name = camel_str!(name);
        select.push(quote! {
            m.entry(#gql_name).or_insert_with(HashSet::new).insert(#name_str);
        });
        let sql_expr = ts2!(a.raw());
        gql_exprs.push(quote! {
            m.insert(#name_str, #sql_expr);
        });
    }

    (struk, resolver, select, gql_exprs)
}

fn push_struk_resolver(name: &Ts2, ty: &Ts2, struk: &mut Vec<Ts2>, resolver: &mut Vec<Ts2>) {
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
