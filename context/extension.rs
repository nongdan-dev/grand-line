use crate::*;
use async_graphql::{
    extensions::{Extension, ExtensionContext, ExtensionFactory, NextExecute, NextPrepareRequest},
    Request, Response, ServerError, ServerResult,
};
use std::sync::Arc;

pub struct GrandLineExtension;

impl ExtensionFactory for GrandLineExtension {
    fn create(&self) -> Arc<dyn Extension> {
        Arc::new(GrandLineExtensionImpl)
    }
}

struct GrandLineExtensionImpl;

#[async_trait::async_trait]
impl Extension for GrandLineExtensionImpl {
    async fn prepare_request(
        &self,
        ctx: &ExtensionContext<'_>,
        request: Request,
        next: NextPrepareRequest<'_>,
    ) -> ServerResult<Request> {
        let gl = GrandLineContext::new(ctx);
        next.run(ctx, request.data(gl)).await
    }
    async fn execute(
        &self,
        ctx: &ExtensionContext<'_>,
        operation_name: Option<&str>,
        next: NextExecute<'_>,
    ) -> Response {
        let mut r = next.run(ctx, operation_name).await;
        let gl = GrandLineContext::from_extension(ctx);
        if let Err(e) = if r.errors.is_empty() {
            gl.commit().await
        } else {
            gl.rollback().await
        } {
            r.errors.push(ServerError::new(e.to_string(), None));
        }
        r
    }
}
