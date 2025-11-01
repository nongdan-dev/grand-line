use crate::prelude::*;
use syn::{Attribute, Field, FieldsNamed, punctuated::Punctuated, token::Comma};

pub struct ModelDeriveAttr {
    pub defaults: Vec<Attr>,
    pub virtuals: Vec<Vec<Attr>>,
    pub exprs: Vec<Vec<Attr>>,
    pub gql_fields: Vec<(Field, Vec<Attr>)>,
    pub sql_fields: FieldsNamed,
}

/// Parse macro attributes, extract and validate fields.
pub fn model_derive_attr(model: &str, fields: &Punctuated<Field, Comma>) -> ModelDeriveAttr {
    let (mut defaults, mut virtuals, mut exprs, mut gql, mut sql) =
        (vec![], vec![], vec![], vec![], vec![]);

    for f in fields {
        let attrs = Attr::from_field(model, f, &|a| ATTR_RAW.contains(a));
        attr_validate(&attrs);
        // default
        if let Some(def) = attrs.iter().find(|a| a.attr == AttrTy::Default).cloned() {
            defaults.push(def);
        }
        // virtuals
        if let Some(v) = attr_is_virtual(&attrs) {
            if v == VirtualTy::SqlExpr {
                exprs.push(attrs);
            } else {
                virtuals.push(attrs);
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

    ModelDeriveAttr {
        defaults,
        virtuals,
        exprs,
        gql_fields: gql,
        sql_fields: FieldsNamed {
            brace_token: Default::default(),
            named: Punctuated::from_iter(sql),
        },
    }
}

/// Validate or panic.
fn attr_validate(attrs: &[Attr]) {
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
            "{}.{} should have only one between: {}",
            attrs[0].field_model(),
            attrs[0].field_name(),
            matches.iter().map(|v| s!(v)).collect::<Vec<_>>().join(", "),
        );
        pan!(err);
    }
}

/// All virtual attributes.
/// If any of these attributes matched, we should extract the whole field out.
fn attr_is_virtual(attrs: &[Attr]) -> Option<VirtualTy> {
    let map = VirtualTy::all()
        .iter()
        .map(|v| (s!(v), v.clone()))
        .collect::<HashMap<_, _>>();
    attrs
        .iter()
        .filter_map(|a| map.get(&a.attr))
        .next()
        .cloned()
}

/// Filter to only keep related attrs for the sql model.
/// If any of these attributes matched, we should removed them out of the field.
fn attr_sql(attrs: &[Attr]) -> Vec<Attribute> {
    let mut tobe_removed = AttrTy::all().iter().map(|a| s!(a)).collect::<HashSet<_>>();
    tobe_removed.insert(s!(AttrTy::Graphql));
    attrs
        .iter()
        .filter(|a| !tobe_removed.contains(&a.attr))
        .map(|a| a.field_attr())
        .collect()
}

/// Check if we should not include this field in the gql model.
fn attr_is_gql_skip(attrs: &[Attr]) -> bool {
    attrs.iter().any(|a| a.is("graphql") && a.has("skip"))
}

/// Filter to only keep related attrs for the gql model.
fn attr_gql(attrs: &[Attr]) -> Vec<Attribute> {
    let ne = [
        // TODO: currently remove all attrs for the gql model
        "",
    ]
    .into_iter()
    .map(|a| s!(a))
    .collect::<HashSet<_>>();
    attrs
        .iter()
        .filter(|a| !ne.contains(&a.attr))
        .map(|a| a.field_attr())
        .collect()
}
