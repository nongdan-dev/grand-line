use crate::prelude::*;
use syn::{Attribute, Field, FieldsNamed, punctuated::Punctuated, token::Comma};

/// Parse macro attributes, extract and validate fields.
pub fn attr_parse(
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
        attr_validate(&attrs);
        // default
        if let Some(def) = attrs.iter().find(|a| a.attr == AttrTy::Default).cloned() {
            defs.push(def);
        }
        // virtuals
        if let Some(v) = attr_is_virtual(&attrs) {
            if v == VirtualTy::SqlExpr {
                exprs.push(attrs);
            } else {
                virs.push(attrs);
            }
            continue;
        }
        // sql
        let mut extracted = f.clone();
        extracted.attrs = attr_sql(&attrs);
        sql.push(extracted);
        // gql
        if !attr_is_gql_skip(&attrs) {
            let mut extracted = f.clone();
            extracted.attrs = attr_gql(&attrs);
            gql.push((extracted, attrs));
        }
    }

    let fields = FieldsNamed {
        brace_token: Default::default(),
        named: Punctuated::from_iter(sql),
    };
    (defs, virs, exprs, gql, fields)
}

/// Validate or panic.
fn attr_validate(attrs: &Vec<Attr>) {
    // ensure it should not have more than one of our attributes
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
fn attr_is_virtual(attrs: &Vec<Attr>) -> Option<VirtualTy> {
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

/// Filter to only keep related attrs for the sql model.
/// If any of these attributes matched, we should removed them out of the field.
fn attr_sql(attrs: &Vec<Attr>) -> Vec<Attribute> {
    let mut tobe_removed = AttrTy::all().iter().map(|a| s!(a)).collect::<HashSet<_>>();
    tobe_removed.insert(s!(AttrTy::Graphql));
    attrs
        .iter()
        .filter(|a| !tobe_removed.contains(&a.attr))
        .map(|a| a.field_attr())
        .collect()
}

/// Check if we should not include this field in the gql model.
fn attr_is_gql_skip(attrs: &Vec<Attr>) -> bool {
    attrs.iter().any(|a| a.is("graphql") && a.has("skip"))
}

/// Filter to only keep related attrs for the gql model.
fn attr_gql(attrs: &Vec<Attr>) -> Vec<Attribute> {
    let ne = vec![
        // TODO: currently remove all attrs for the gql model
        "",
    ]
    .iter()
    .map(|a| s!(a))
    .collect::<HashSet<_>>();
    attrs
        .iter()
        .filter(|a| !ne.contains(&a.attr))
        .map(|a| a.field_attr())
        .collect()
}
