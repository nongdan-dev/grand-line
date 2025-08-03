use crate::prelude::*;

pub fn filter_and_or_not(f: &Ts2, struk: &mut Vec<Ts2>, query: &mut Vec<Ts2>) {
    push_and_or(f, struk, query, "and");
    push_and_or(f, struk, query, "or");
    push_not(f, struk, query);
}

fn push_and_or(f: &Ts2, struk: &mut Vec<Ts2>, query: &mut Vec<Ts2>, op_str: &str) {
    let op = ts2!(op_str);
    let gql_op = op_str.to_uppercase();
    let cond = ts2!(if op_str == "and" { "all" } else { "any" });

    struk.push(quote! {
        #[graphql(name=#gql_op)]
        pub #op: Option<Vec<#f>>,
    });
    query.push(quote! {
        if let Some(v) = this.#op {
            let mut #op = Condition::#cond();
            for f in v {
                #op = #op.add(f.condition());
            }
            c = c.add(#op);
        }
    });
}

fn push_not(f: &Ts2, struk: &mut Vec<Ts2>, query: &mut Vec<Ts2>) {
    struk.push(quote! {
        #[graphql(name="NOT")]
        pub not: Option<Box<#f>>,
    });
    query.push(quote! {
        if let Some(v) = this.not {
            c = c.add(Condition::not(v.condition()));
        }
    });
}
