use crate::prelude::*;

/// Abstract extra Select methods implementation.
pub trait SelectXImpl<T, M, A, F, O, G>
where
    T: EntityX<M, A, F, O, G>,
    M: FromQueryResult + Send + Sync,
    A: ActiveModelTrait<Entity = T>,
    F: Filter<T>,
    O: OrderBy<T>,
    G: FromQueryResult + Send + Sync,
    Self: QueryFilter + QuerySelect,
{
    /// Filter with condition deleted_at is not null, if there is deleted_at.
    fn exclude_deleted(self) -> Self {
        match T::config_col_deleted_at() {
            Some(c) => self.filter(Condition::all().add(c.is_null())),
            None => self,
        }
    }

    /// Select only id for the graphql delete response.
    fn gql_select_id(self) -> Res<Selector<SelectModel<G>>>;
}

/// Automatically implement for Select<T>.
impl<T, M, A, F, O, G> SelectXImpl<T, M, A, F, O, G> for Select<T>
where
    T: EntityX<M, A, F, O, G>,
    M: FromQueryResult + Send + Sync,
    A: ActiveModelTrait<Entity = T>,
    F: Filter<T>,
    O: OrderBy<T>,
    G: FromQueryResult + Send + Sync,
{
    fn gql_select_id(self) -> Res<Selector<SelectModel<G>>> {
        T::config_col_id().map(|c| self.select_only().column(c).into_model::<G>())
    }
}
