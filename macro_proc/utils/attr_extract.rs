use crate::prelude::*;
use syn::{Attribute, Field, FieldsNamed, punctuated::Punctuated, token::Comma};

/// Validate it should have only one of our attributes.
fn validate(attrs: &Vec<Attr>) {
    let map = AttrTy::all()
        .iter()
        .map(|v| (s!(v), v.clone()))
        .collect::<HashMap<_, _>>();
    let mut matches = vec![];
    for a in attrs {
        if let Some(v) = map.get(&a.attr) {
            matches.push(v.clone());
        }
    }
    if matches.len() > 1 {
        let err = f!(
            "`{}.{}` should have only one between: {}",
            attrs[0].field_model(),
            attrs[0].field_name(),
            matches.iter().map(|v| s!(v)).collect::<Vec<_>>().join(", "),
        );
        pan!(err);
    }
}

/// All virtual attributes.
/// If any of these attributes matched, we should extract the whole field out.
fn is_virtual(attrs: &Vec<Attr>) -> Option<VirtualTy> {
    let map = VirtualTy::all()
        .iter()
        .map(|v| (s!(v), v.clone()))
        .collect::<HashMap<_, _>>();
    attrs
        .iter()
        .filter_map(|a| map.get(&a.attr))
        .nth(0)
        .cloned()
}

/// All attribute to be extracted from sql.
/// If any of these attributes matched, we should removed them out of the field.
fn extract_sql(attrs: &Vec<Attr>) -> Vec<Attribute> {
    let ne = AttrTy::all().iter().map(|a| s!(a)).collect::<HashSet<_>>();
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
        .map(|a| s!(a))
        .collect::<HashSet<_>>();
    attrs
        .iter()
        .filter(|a| !ne.contains(&a.attr))
        .map(|a| a.field_attr())
        .collect()
}

pub fn attr_extract(
    model: &str,
    fields: &Punctuated<Field, Comma>,
) -> (
    Vec<Attr>,
    Vec<Vec<Attr>>,
    Vec<Vec<Attr>>,
    Vec<(Field, Vec<Attr>)>,
    FieldsNamed,
) {
    let (mut defs, mut virs, mut exprs, mut gql, mut sql) =
        (vec![], vec![], vec![], vec![], vec![]);

    for f in fields {
        let attrs = Attr::from_field(model, &f, &|a| ATTR_RAW.contains(a));
        validate(&attrs);
        // default
        if let Some(def) = attrs.iter().find(|a| a.attr == AttrTy::Default).cloned() {
            defs.push(def);
        }
        // virtuals
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
    (defs, virs, exprs, gql, fields)
}
