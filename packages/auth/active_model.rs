use crate::prelude::*;

#[async_trait]
pub trait AmExec: Sized {
    type Model: Send;
    async fn exec(self, ctx: &Context<'_>) -> Res<Self::Model>;
}

#[async_trait]
impl<E, A> AmExec for ActiveModelWrapper<AmCreate, E, A>
where
    E: EntityX<A = A>,
    A: ActiveModelX<E> + Send,
    E::M: Send,
{
    type Model = E::M;

    async fn exec(self, ctx: &Context<'_>) -> Res<Self::Model> {
        let mut am = self.into_active_model();
        let mut by_id = None;
        if !am.get_created_by_id().is_set() && E::col_created_by_id().is_some() {
            let user_id = ctx.auth().await?;
            by_id = Some(user_id.clone());
            am = am.set_created_by_id(Some(user_id));
        }
        let tx = &*ctx.tx().await?;
        let r = am.insert(tx).await?;
        if by_id.is_none() && E::has_history() {
            by_id = ctx.auth().await.ok();
        }
        E::record_history(tx, "create", &r, by_id).await?;
        Ok(r)
    }
}

#[async_trait]
impl<E, A> AmExec for ActiveModelWrapper<AmUpdate, E, A>
where
    E: EntityX<A = A>,
    A: ActiveModelX<E> + Send,
    E::M: Send,
{
    type Model = E::M;

    async fn exec(self, ctx: &Context<'_>) -> Res<Self::Model> {
        let mut am = self.into_active_model();
        let mut by_id = None;
        if !am.get_updated_by_id().is_set() && E::col_updated_by_id().is_some() {
            let user_id = ctx.auth().await?;
            by_id = Some(user_id.clone());
            am = am.set_updated_by_id(Some(user_id));
        }
        let tx = &*ctx.tx().await?;
        let r = am.update(tx).await?;
        if by_id.is_none() && E::has_history() {
            by_id = ctx.auth().await.ok();
        }
        E::record_history(tx, "update", &r, by_id).await?;
        Ok(r)
    }
}

#[async_trait]
impl<E, A> AmExec for ActiveModelWrapper<AmSoftDelete, E, A>
where
    E: EntityX<A = A>,
    A: ActiveModelX<E> + Send,
    E::M: Send,
{
    type Model = E::M;

    async fn exec(self, ctx: &Context<'_>) -> Res<Self::Model> {
        let mut am = self.into_active_model();
        let mut by_id = None;
        if !am.get_deleted_by_id().is_set() && E::col_deleted_by_id().is_some() {
            let user_id = ctx.auth().await?;
            by_id = Some(user_id.clone());
            am = am.set_deleted_by_id(Some(user_id));
        }
        let tx = &*ctx.tx().await?;
        let r = am.update(tx).await?;
        if by_id.is_none() && E::has_history() {
            by_id = ctx.auth().await.ok();
        }
        E::record_history(tx, "delete", &r, by_id).await?;
        Ok(r)
    }
}
