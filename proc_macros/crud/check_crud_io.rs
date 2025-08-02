use crate::prelude::*;

pub fn check_crud_io(a: &CrudAttr, r: &ResolverTyItem) {
    let ra = &a.resolver_attr;
    if !a.resolver_inputs && str!(r.inputs) != "" {
        panic_with_location!(
            "{} inputs must be empty unless resolver_inputs=true, found `{}`",
            r.gql_name,
            r.inputs,
        );
    }
    if !a.resolver_output {
        if str!(r.output) != "()" {
            panic_with_location!(
                "{} output must be empty unless resolver_output=true, found `{}`",
                r.gql_name,
                r.output,
            );
        }
        if ra.no_tx || ra.no_ctx {
            panic_with_location!("{} output requires tx, ctx", r.gql_name);
        }
    }
    if a.resolver_inputs && a.resolver_output {
        panic_with_location!(
            "{} should use #[query] or #[mutation] instead since both resolver_inputs=true and resolver_output=true",
            r.gql_name,
        );
    }
    if !ra.no_tx {
        if ra.no_ctx {
            panic_with_location!("{} tx requires ctx", r.gql_name);
        }
    }
}
