use crate::prelude::*;

#[async_trait]
pub trait AuthContext<'a>
where
    Self: AuthConfigContext<'a> + AuthHttpContext<'a> + AuthCacheContext<'a> + AuthEnsureContext<'a>,
{
    async fn auth(&self) -> Res<String> {
        let user_id = self
            .auth_unchecked()
            .await?
            .as_ref()
            .0
            .as_ref()
            .map(|ls| ls.user_id.clone())
            .ok_or(MyErr::Unauthenticated)?;
        Ok(user_id)
    }
}

#[async_trait]
impl<'a> AuthContext<'a> for Context<'a> {
}
