use crate::prelude::*;

/// Helper trait to abstract extra methods into sea_orm column.
pub trait ColumnX<E>
where
    E: EntityX,
    Self: ColumnTrait,
{
    fn to_loader_key(&self, look_ahead: &Vec<LookaheadX<E>>, include_deleted: bool) -> String {
        let include_deleted = if include_deleted {
            "include_deleted-"
        } else {
            ""
        };
        E::_model_name().to_owned()
            + "."
            + self.as_str()
            + "-"
            + include_deleted
            + &look_ahead
                .iter()
                .map(|l| l.c.to_owned())
                .collect::<Vec<_>>()
                .join(",")
    }
}
