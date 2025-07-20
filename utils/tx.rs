use crate::GrandLineContext;
use async_graphql::{
    extensions::{Extension, ExtensionContext, ExtensionFactory, NextRequest},
    Response, ServerError,
};
use std::sync::Arc;

pub struct TxExtension;

impl ExtensionFactory for TxExtension {
    fn create(&self) -> Arc<dyn Extension> {
        Arc::new(TxExtensionImpl)
    }
}

struct TxExtensionImpl;

#[async_trait::async_trait]
impl Extension for TxExtensionImpl {
    async fn request(&self, ctx: &ExtensionContext<'_>, next: NextRequest<'_>) -> Response {
        let mut r = next.run(ctx).await;
        let gl = ctx.data_unchecked::<GrandLineContext>();

        let mut err = None;
        match gl.tx_peek().await {
            Err(e) => {
                // failed to peek
                err = Some(e);
            }
            Ok(None) => {
                // no tx in ctx arc, no need to commit/rollbback
            }
            Ok(Some(_)) => {
                if r.errors.is_empty() {
                    // executed with no error, try to commit
                    if let Err(e) = gl.commit().await {
                        err = Some(e);
                    }
                } else {
                    // executed with error, try to rollback
                    if let Err(e) = gl.rollback().await {
                        err = Some(e);
                    }
                }
            }
        }
        if let Some(e) = err {
            r.errors.push(ServerError::new(e.to_string(), None));
        }

        r
    }
}
