use crate::prelude::*;

/// Abstract extra model methods implementation.
#[async_trait]
pub trait ModelXAsync<T>
where
    T: EntityX,
    Self: ModelX<T>,
{
    // Convert sql model to gql model, with checking virtual fields from context.
    async fn to_gql(self, ctx: &Context<'_>) -> Res<T::G> {
        let r = if T::gql_look_ahead(ctx)?.iter().any(|(_, _, e)| e.is_some()) {
            let _tx = ctx.tx().await?;
            let tx = _tx.as_ref();
            T::find()
                .by_id(&self._get_id())?
                .gql_select(ctx)?
                .one(tx)
                .await?
                .ok_or_else(|| GrandLineError::Client(ErrClient::Db404))?
        } else {
            self._to_gql()
        };
        Ok(r)
    }
}

#[async_trait]
impl<T, M> ModelXAsync<T> for M
where
    T: EntityX,
    M: ModelX<T>,
{
}
