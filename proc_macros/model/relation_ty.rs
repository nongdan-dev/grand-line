use crate::prelude::*;
use strum_macros::{Display, EnumString};

#[derive(Debug, Clone, Eq, PartialEq, Display, EnumString, PartialEqString)]
pub enum RelationTy {
    #[strum(serialize = "belongs_to")]
    BelongsTo,
    #[strum(serialize = "has_one")]
    HasOne,
    #[strum(serialize = "has_many")]
    HasMany,
    #[strum(serialize = "many_to_many")]
    ManyToMany,
}

impl RelationTy {
    pub fn all() -> Vec<Self> {
        vec![
            RelationTy::BelongsTo,
            RelationTy::HasOne,
            RelationTy::HasMany,
            RelationTy::ManyToMany,
        ]
    }
}
