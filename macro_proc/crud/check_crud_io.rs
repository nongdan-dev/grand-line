use crate::prelude::*;

pub fn check_crud_io(a: &CrudAttr, r: &ResolverTyItem) {
    let ra = &a.resolver_attr;
    if !a.resolver_inputs && s!(r.inputs) != "" {
        let err = f!(
            "{} inputs must be empty unless resolver_inputs=true, found `{}`",
            r.gql_name,
            r.inputs,
        );
        pan!(err);
    }
    if !a.resolver_output {
        if s!(r.output) != "()" {
            let err = f!(
                "{} output must be empty unless resolver_output=true, found `{}`",
                r.gql_name,
                r.output,
            );
            pan!(err);
        }
        if ra.no_tx || ra.no_ctx {
            let err = f!("{} output requires tx, ctx", r.gql_name);
            pan!(err);
        }
    }
    if a.resolver_inputs && a.resolver_output {
        let err = f!(
            "{} should use #[query] or #[mutation] instead since both resolver_inputs=true and resolver_output=true",
            r.gql_name,
        );
        pan!(err);
    }
    if !ra.no_tx {
        if ra.no_ctx {
            let err = f!("{} tx requires ctx", r.gql_name);
            pan!(err);
        }
    }
}
