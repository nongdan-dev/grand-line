use crate::prelude::*;
use async_graphql::Context;

/// Abstract extra Select async methods implementation.
#[async_trait]
pub trait SelectXAsyncImpl<T, M, A, F, O, G>
where
    T: EntityX<M, A, F, O, G> + 'static,
    M: FromQueryResult + Send + Sync + 'static,
    A: ActiveModelTrait<Entity = T> + 'static,
    F: Filter<T> + 'static,
    O: OrderBy<T> + 'static,
    G: FromQueryResult + Send + Sync + 'static,
    Self: QueryFilter + QuerySelect + 'static,
{
    /// Select only columns from requested fields in the graphql context.
    async fn gql_select(self, ctx: &Context<'_>) -> Res<Selector<SelectModel<G>>>;
}

/// Automatically implement for Select<T>.
#[async_trait]
impl<T, M, A, F, O, G> SelectXAsyncImpl<T, M, A, F, O, G> for Select<T>
where
    T: EntityX<M, A, F, O, G> + 'static,
    M: FromQueryResult + Send + Sync + 'static,
    A: ActiveModelTrait<Entity = T> + 'static,
    F: Filter<T> + 'static,
    O: OrderBy<T> + 'static,
    G: FromQueryResult + Send + Sync + 'static,
{
    async fn gql_select(self, ctx: &Context<'_>) -> Res<Selector<SelectModel<G>>> {
        let mut q = self.select_only();
        for c in T::gql_look_ahead(ctx).await? {
            q = q.select_column(c);
        }
        let r = q.into_model::<G>();
        Ok(r)
    }
}
