use crate::prelude::*;

#[async_trait]
pub trait AuthEnsureContext {
    async fn auth_ensure_in_macro(&self, check: AuthDirectiveCheck) -> Res<()>;
    async fn auth_ensure_authenticated(&self) -> Res<()>;
    async fn auth_ensure_not_authenticated(&self) -> Res<()>;
}

#[async_trait]
impl AuthEnsureContext for Context<'_> {
    async fn auth_ensure_in_macro(&self, check: AuthDirectiveCheck) -> Res<()> {
        match check {
            AuthDirectiveCheck::Authenticated => self.auth_ensure_authenticated().await?,
            AuthDirectiveCheck::Unauthenticated => self.auth_ensure_not_authenticated().await?,
        }
        Ok(())
    }

    async fn auth_ensure_authenticated(&self) -> Res<()> {
        if self.auth_with_cache().await?.as_ref().is_none() {
            Err(MyErr::Unauthenticated)?;
        }
        Ok(())
    }

    async fn auth_ensure_not_authenticated(&self) -> Res<()> {
        if self.auth_with_cache().await?.as_ref().is_some() {
            Err(MyErr::AlreadyAuthenticated)?;
        }
        Ok(())
    }
}
