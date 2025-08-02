use crate::prelude::*;
use syn::{Attribute, Field, FieldsNamed, punctuated::Punctuated};

/// All virtual attributes, defined in derive GrandLineModel.
/// If any of these attributes matched, we should extract the whole field out.
fn is_virtual(attrs: &Vec<Attr>) -> Option<VirtualTy> {
    let map = VirtualTy::all()
        .iter()
        .map(|v| (str!(v), v.clone()))
        .collect::<HashMap<_, _>>();
    let mut matches = vec![];
    for a in attrs {
        if let Some(v) = map.get(&a.attr) {
            matches.push(v.clone());
        }
    }
    if matches.len() > 1 {
        panic_with_location!(
            "{}.{} should have only one between: {}",
            attrs[0].field_model(),
            attrs[0].field_name(),
            matches
                .iter()
                .map(|v| str!(v))
                .collect::<Vec<_>>()
                .join(", "),
        );
    }
    matches.get(0).cloned()
}

/// All attribute to be extracted from sql, defined in derive GrandLineModel.
/// If any of these attributes matched, we should removed them out of the field.
fn extract_sql(attrs: &Vec<Attr>) -> Vec<Attribute> {
    let ne = vec!["graphql"]
        .iter()
        .map(|a| str!(a))
        .collect::<HashSet<_>>();
    attrs
        .iter()
        .filter(|a| !ne.contains(&a.attr))
        .map(|a| a.field_attr())
        .collect()
}

/// Check if we should not include this field in the gql.
fn is_gql_skip(attrs: &Vec<Attr>) -> bool {
    attrs.iter().any(|a| a.is("graphql") && a.has("skip"))
}

/// All attribute to be extracted from gql.
/// Keep sea_orm from_expr to support custom sql expression.
fn extract_gql(attrs: &Vec<Attr>) -> Vec<Attribute> {
    let ne = vec!["sea_orm"]
        .iter()
        .map(|a| str!(a))
        .collect::<HashSet<_>>();
    attrs
        .iter()
        .filter(|a| !ne.contains(&a.attr))
        .map(|a| a.field_attr())
        .collect()
}

pub fn extract_and_validate_fields(
    model: &str,
    fields: &Vec<Field>,
) -> (
    Vec<Vec<Attr>>,
    Vec<Vec<Attr>>,
    Vec<(Field, Vec<Attr>)>,
    FieldsNamed,
) {
    let (mut virs, mut exprs, mut gql, mut sql) = (vec![], vec![], vec![], vec![]);

    for f in fields {
        // virtuals
        let attrs = Attr::from_field(model, &f);
        if let Some(v) = is_virtual(&attrs) {
            if v == VirtualTy::SqlExpr {
                exprs.push(attrs);
            } else {
                virs.push(attrs);
            }
            continue;
        }
        // sql
        let mut extracted = f.clone();
        extracted.attrs = extract_sql(&attrs);
        sql.push(extracted);
        // gql
        if !is_gql_skip(&attrs) {
            let mut extracted = f.clone();
            extracted.attrs = extract_gql(&attrs);
            gql.push((extracted, attrs));
        }
    }

    let fields = FieldsNamed {
        brace_token: Default::default(),
        named: Punctuated::from_iter(sql),
    };
    (virs, exprs, gql, fields)
}
