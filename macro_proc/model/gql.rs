use crate::prelude::*;
use syn::Field;

pub struct GqlAttr {
    pub struk: Vec<Ts2>,
    pub defaults: Vec<Ts2>,
    pub resolver: Vec<Ts2>,
    pub into: Vec<Ts2>,
    pub cols: Vec<Ts2>,
    pub select: Vec<Ts2>,
    pub get_col: Vec<Ts2>,
}

pub fn gql_attr(model_str: &str, gql_fields: &Vec<(Field, Vec<Attr>)>) -> GqlAttr {
    let (mut struk, mut defaults, mut resolver, mut into, mut cols, mut select, mut get_col) =
        (vec![], vec![], vec![], vec![], vec![], vec![], vec![]);

    for (f, _) in gql_fields {
        let name = f.ident.to_token_stream();
        let ty = f.ty.to_token_stream();
        let (opt, uw_str) = unwrap_option_str(&ty);
        push_struk_resolver(model_str, &name, &ty, &mut struk, &mut resolver);
        push_default(&mut defaults, &name);

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
        defaults,
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
    pub defaults: Vec<Ts2>,
    pub resolver: Vec<Ts2>,
    pub select: Vec<Ts2>,
    pub exprs: Vec<Ts2>,
}

pub fn gql_exprs_ts2(model_str: &str, exprs: &Vec<Vec<Attr>>) -> GqlAttrExprs {
    let (mut struk, mut defaults, mut resolver, mut select, mut gql_exprs) =
        (vec![], vec![], vec![], vec![], vec![]);

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
        push_struk_resolver(model_str, &name, &ty, &mut struk, &mut resolver);
        push_default(&mut defaults, &name);

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
        defaults,
    }
}

fn push_struk_resolver(
    model_str: &str,
    name: &Ts2,
    ty: &Ts2,
    struk: &mut Vec<Ts2>,
    resolver: &mut Vec<Ts2>,
) {
    let name_str = s!(name);
    let gql_name = camel_str!(name);
    let (opt, uw) = unwrap_option(ty);

    struk.push(quote! {
        pub #name: Option<#uw>,
    });

    let unwrap = if opt {
        ts2!()
    } else {
        quote! {
            .ok_or_else(|| GrandLineInternalDbErr::DbGqlField404 {
                model: #model_str,
                field: #name_str,
            })?
        }
    };

    resolver.push(quote! {
        // TODO: copy #[graphql...] and comments from the original field
        #[graphql(name=#gql_name)]
        pub async fn #name(&self) -> Res<#ty> {
            let v = self.#name.clone()#unwrap;
            Ok(v)
        }
    });
}

fn push_default(defaults: &mut Vec<Ts2>, name: &Ts2) {
    defaults.push(quote! {
        #name: None,
    });
}
