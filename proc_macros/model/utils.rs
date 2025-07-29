use crate::prelude::*;
use syn::{Attribute, Field, FieldsNamed, punctuated::Punctuated};

/// All virtual attributes, defined in derive GrandLineModel.
/// If any of these attributes matched, we should extract the whole field out.
pub fn is_virtual(attrs: &Vec<Attr>) -> bool {
    let eq = VirtualTy::all();
    let eq = eq.iter().collect::<HashSet<_>>();
    let attrs = attrs
        .iter()
        .filter(|a| eq.contains(&a.attr))
        .collect::<Vec<_>>();
    if attrs.len() > 1 {
        let multiple = attrs
            .iter()
            .map(|a| a.attr.clone())
            .collect::<Vec<_>>()
            .join(", ");
        panic!(
            "{}.{} should have only one between: {}",
            attrs[0].field_model(),
            attrs[0].field_name(),
            multiple
        );
    }
    attrs.len() == 1
}

/// All attribute to be extracted from sql, defined in derive GrandLineModel.
/// If any of these attributes matched, we should removed them out of the field.
pub fn extract_sql(attrs: &Vec<Attr>) -> Vec<Attribute> {
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
pub fn is_gql_skip(attrs: &Vec<Attr>) -> bool {
    attrs.iter().any(|a| a.is("graphql") && a.has("skip"))
}

/// All attribute to be extracted from gql.
/// Keep sea_orm from_expr to support custom sql expression.
pub fn extract_gql(attrs: &Vec<Attr>) -> Vec<Attribute> {
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
        if is_virtual(&attrs) {
            if attrs.iter().any(|a| a.attr == VirtualTy::SqlExpr) {
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
