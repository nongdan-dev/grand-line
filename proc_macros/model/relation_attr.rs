use crate::prelude::*;
use field_names::FieldNames;

#[derive(Debug, Clone, Default, FieldNames)]
pub struct RelationAttr {
    pub key: String,
    pub through: String,
    pub other_key: String,
}
impl From<Attr> for RelationAttr {
    fn from(a: Attr) -> Self {
        let f = Self::FIELDS;
        Self {
            key: a.str(f[0]),
            through: a.str(f[0]),
            other_key: a.str(f[0]),
        }
    }
}
impl AttrValidate for RelationAttr {
    fn attr_fields(a: &Attr) -> Vec<String> {
        let f = Self::FIELDS;
        let f = if a.attr == RelationTy::ManyToMany {
            f.to_vec()
        } else {
            vec![f[0]]
        };
        f.iter().map(|f| str!(f)).collect()
    }
}
