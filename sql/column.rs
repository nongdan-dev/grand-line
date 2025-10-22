use crate::prelude::*;

/// Helper trait to abstract extra methods into sea_orm column.
pub trait ColumnX<E>
where
    E: EntityX,
    Self: ColumnTrait,
{
    fn to_string_with_model_name(&self) -> String {
        format!("{}.{}", E::_model_name(), self.as_str())
    }
    fn to_loader_key(&self, look_ahead: &Vec<LookaheadX<E>>) -> String {
        self.to_string_with_model_name()
            + "-"
            + &look_ahead
                .iter()
                .map(|l| l.c.to_owned())
                .collect::<Vec<_>>()
                .join(",")
    }
}
