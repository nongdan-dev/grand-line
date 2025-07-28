use crate::prelude::*;
use syn::{Attribute, Field, FieldsNamed, punctuated::Punctuated};

/// All virtual attributes, defined in derive GrandLineModel.
/// If any of these attributes matched, we should extract the whole field out.
fn all_virtuals() -> Vec<&'static str> {
    vec!["belongs_to", "has_one", "has_many", "many_to_many"]
}
fn is_virtual(attrs: &Vec<Attr>) -> bool {
    let set = all_virtuals()
        .iter()
        .map(|a| str!(a))
        .collect::<HashSet<_>>();
    attrs.iter().any(|a| set.contains(&a.attr))
}

/// All attribute to be extracted, defined in derive GrandLineModel.
/// If any of these attributes matched, we should removed them out of the field.
fn all_extracts() -> Vec<&'static str> {
    vec!["graphql"]
}
fn is_not_extract(a: &&Attribute) -> bool {
    let set = all_extracts()
        .iter()
        .map(|a| str!(a))
        .collect::<HashSet<_>>();
    !set.contains(&str!(a.path().to_token_stream()))
}

/// Check #[graphql(skip)].
fn is_graphql_skip(attrs: &Vec<Attr>) -> bool {
    attrs.iter().any(|a| a.is("graphql") && a.has("skip"))
}

pub fn extract_and_validate_fields(
    model: &str,
    fields: &Vec<Field>,
) -> (FieldsNamed, Vec<(Field, Vec<Attr>)>, Vec<Vec<Attr>>) {
    let (mut sql, mut gql, mut virs) = (vec![], vec![], vec![]);

    for f in fields {
        // virtuals
        let attrs = Attr::from_field(model, &f);
        if is_virtual(&attrs) {
            virs.push(attrs);
            continue;
        }
        // sql
        let mut extracted = f.clone();
        extracted.attrs = extracted
            .attrs
            .iter()
            .filter(is_not_extract)
            .map(|a| a.clone())
            .collect();
        sql.push(extracted);
        // gql
        if is_graphql_skip(&attrs) {
            continue;
        }
        gql.push((f.clone(), attrs));
    }

    let fields = FieldsNamed {
        brace_token: Default::default(),
        named: Punctuated::from_iter(sql),
    };
    (fields, gql, virs)
}
