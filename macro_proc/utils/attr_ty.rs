use crate::prelude::*;
use strum_macros::{AsRefStr, Display};

#[derive(Clone, Eq, PartialEq, AsRefStr, Display, PartialEqString)]
#[strum(serialize_all = "snake_case")]
pub enum MacroTy {
    Model,
    Search,
    Count,
    Detail,
    Create,
    Update,
    Delete,
}

pub static ATTR_RAW: LazyLock<HashSet<String>> = LazyLock::new(|| {
    let mut set = HashSet::new();
    set.insert(s!(AttrTy::Default));
    set.insert(s!(VirtualTy::SqlExpr));
    set
});

#[derive(Clone, Eq, PartialEq, AsRefStr, Display, PartialEqString)]
#[strum(serialize_all = "snake_case")]
pub enum AttrTy {
    Graphql,
    Default,
    #[strum(serialize = "{0}")]
    Virtual(VirtualTy),
}
impl AttrTy {
    pub fn all() -> Vec<AttrTy> {
        let mut all = VirtualTy::all()
            .iter()
            .map(|r| AttrTy::Virtual(r.clone()))
            .collect::<Vec<_>>();
        all.push(AttrTy::Default);
        all
    }
}

#[derive(Clone, Eq, PartialEq, AsRefStr, Display, PartialEqString)]
#[strum(serialize_all = "snake_case")]
pub enum VirtualTy {
    #[strum(serialize = "{0}")]
    Relation(RelationTy),
    SqlExpr,
    Resolver,
}
impl VirtualTy {
    pub fn all() -> Vec<VirtualTy> {
        let mut all = RelationTy::all()
            .iter()
            .map(|r| VirtualTy::Relation(r.clone()))
            .collect::<Vec<_>>();
        all.push(VirtualTy::SqlExpr);
        all.push(VirtualTy::Resolver);
        all
    }
}

#[derive(Clone, Eq, PartialEq, AsRefStr, Display, PartialEqString)]
#[strum(serialize_all = "snake_case")]
pub enum RelationTy {
    BelongsTo,
    HasOne,
    HasMany,
    ManyToMany,
}
impl RelationTy {
    pub fn all() -> Vec<RelationTy> {
        vec![
            RelationTy::BelongsTo,
            RelationTy::HasOne,
            RelationTy::HasMany,
            RelationTy::ManyToMany,
        ]
    }
}
