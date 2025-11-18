use crate::prelude::*;

pub struct ModelFieldsAttr {
    pub defaults: Vec<Attr>,
    pub virtuals: Vec<Vec<Attr>>,
    pub exprs: Vec<Vec<Attr>>,
    pub gql_fields: Vec<(Field, Vec<Attr>)>,
    pub sql_fields: FieldsNamed,
}

/// Parse macro attributes, extract and validate fields.
pub fn model_fields_attr(model: &str, fields: &Punctuated<Field, Token![,]>) -> ModelFieldsAttr {
    let (mut defaults, mut virtuals, mut exprs, mut gql, mut sql) =
        (vec![], vec![], vec![], vec![], vec![]);

    for f in fields {
        let attrs = Attr::from_field(model, f, &|a| ATTR_RAW.contains(a));
        attr_validate(&attrs);
        // default
        if let Some(def) = attrs.iter().find(|a| a.attr == AttrTy::Default) {
            defaults.push(def.clone());
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
        let mut extracted = f.clone();
        extracted.attrs = attr_gql(&attrs);
        gql.push((extracted, attrs));
    }

    ModelFieldsAttr {
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
        .map(|t| (t.to_string(), t.clone()))
        .collect::<HashMap<_, _>>();
    let mut matches = vec![];
    for a in attrs {
        if let Some(v) = map.get(&a.attr) {
            matches.push(v.clone());
        }
    }
    if matches.len() > 1 {
        let model = attrs[0].field_model();
        let field = attrs[0].field_name();
        let matches = matches
            .iter()
            .map(|t| t.to_string())
            .collect::<Vec<_>>()
            .join(", ");
        panic!("{model}.{field} should have only one between: {matches}");
    }
}

/// All virtual attributes.
/// If any of these attributes matched, we should extract the whole field out.
fn attr_is_virtual(attrs: &[Attr]) -> Option<VirtualTy> {
    let map = VirtualTy::all()
        .iter()
        .map(|t| (t.to_string(), t.clone()))
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
    let mut tobe_removed = AttrTy::all()
        .iter()
        .map(|t| t.to_string())
        .collect::<HashSet<_>>();
    tobe_removed.insert(AttrTy::Graphql.to_string());
    attrs
        .iter()
        .filter(|a| !tobe_removed.contains(&a.attr))
        .map(|a| a.field_attr())
        .collect()
}

/// Filter to only keep related attrs for the gql model.
fn attr_gql(attrs: &[Attr]) -> Vec<Attribute> {
    let ne = hashset![
        // TODO: copy #[graphql...] and comments from the original field
        "",
    ];
    attrs
        .iter()
        .filter(|a| !ne.contains(&a.attr.as_ref()))
        .map(|a| a.field_attr())
        .collect()
}

/// Check if we should not include this field in the gql resolver, filter, order by..
/// This field can be in the gql model to support sql dep.
pub fn attr_is_gql_skip(attrs: &[Attr]) -> bool {
    attrs.iter().any(|a| a.is("graphql") && a.has("skip"))
}
