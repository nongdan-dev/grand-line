use crate::prelude::*;

#[field_names]
pub struct ResolverAttr {
    pub call: String,
    pub sql_dep: Vec<String>,
    #[field_names(skip)]
    pub ra: ResolverTyAttr,
    #[field_names(skip)]
    pub inner: Attr,
}
impl TryFrom<Attr> for ResolverAttr {
    type Error = SynErr;
    fn try_from(a: Attr) -> SynRes<Self> {
        let call = if let Some(v) = a.str(Self::FIELD_CALL)? {
            v
        } else {
            let field = a.field_name()?;
            format!("resolve_{field}")
        };
        let sql_dep = a
            .str(Self::FIELD_SQL_DEP)?
            .unwrap_or_default()
            .split(',')
            .map(|s| s.trim().to_owned())
            .collect();
        let ra = a.clone().try_into()?;
        Ok(Self {
            call,
            sql_dep,
            ra,
            inner: a,
        })
    }
}
impl AttrValidate for ResolverAttr {
    fn attr_fields(a: &Attr) -> Vec<String> {
        Self::FIELDS
            .iter()
            .copied()
            .map(|f| f.to_owned())
            .chain(ResolverTyAttr::attr_fields(a))
            .collect()
    }
}
