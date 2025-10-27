use crate::prelude::*;
use syn::Field;

pub struct GqlAttr {
    pub struk: Vec<Ts2>,
    pub resolver: Vec<Ts2>,
    pub into: Vec<Ts2>,
    pub cols: Vec<Ts2>,
    pub select: Vec<Ts2>,
    pub get_col: Vec<Ts2>,
}

pub fn gql_attr(gql_fields: &Vec<(Field, Vec<Attr>)>) -> GqlAttr {
    let (mut struk, mut resolver, mut into, mut cols, mut select, mut get_col) =
        (vec![], vec![], vec![], vec![], vec![], vec![]);

    for (f, _) in gql_fields {
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
            get_col.push(quote! {
                Column::#col => self.#name.clone(),
            });
        }
    }

    GqlAttr {
        struk,
        resolver,
        into,
        cols,
        select,
        get_col,
    }
}

pub struct GqlAttrVirtuals {
    pub select: Vec<Ts2>,
}

pub fn gql_attr_virtuals(virtual_resolvers: &Vec<Box<dyn VirtualResolverFn>>) -> GqlAttrVirtuals {
    let mut select = vec![];
    for v in virtual_resolvers {
        let gql_name = v.gql_name();
        for name_str in v.sql_dep() {
            select.push(quote! {
                m.entry(#gql_name).or_insert_with(HashSet::new).insert(#name_str);
            });
        }
    }
    GqlAttrVirtuals { select }
}

pub struct GqlAttrExprs {
    pub struk: Vec<Ts2>,
    pub resolver: Vec<Ts2>,
    pub select: Vec<Ts2>,
    pub exprs: Vec<Ts2>,
}

pub fn gql_exprs_ts2(exprs: &Vec<Vec<Attr>>) -> GqlAttrExprs {
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

    GqlAttrExprs {
        struk,
        resolver,
        select,
        exprs: gql_exprs,
    }
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
