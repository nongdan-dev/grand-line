use crate::prelude::*;

#[async_trait]
pub trait AuthContext {
    async fn auth(&self) -> Res<String>;
}

#[async_trait]
impl AuthContext for Context<'_> {
    async fn auth(&self) -> Res<String> {
        let user_id = self
            .auth_with_cache()
            .await?
            .as_ref()
            .as_ref()
            .map(|ls| ls.user_id.clone())
            .ok_or(MyErr::Unauthenticated)?;
        Ok(user_id)
    }
}
