use crate::*;
use async_graphql::{
    Request, Response, ServerError, ServerResult,
    extensions::{Extension, ExtensionContext, ExtensionFactory, NextExecute, NextPrepareRequest},
};

/// Extension to insert GrandLineContext on each request, then cleanup at the end of each request.
pub struct GrandLineExtension;

impl ExtensionFactory for GrandLineExtension {
    fn create(&self) -> Arc<dyn Extension> {
        Arc::new(GrandLineExtensionImpl)
    }
}

struct GrandLineExtensionImpl;

#[async_trait::async_trait]
impl Extension for GrandLineExtensionImpl {
    /// Insert GrandLineContext on each request.
    async fn prepare_request(
        &self,
        ctx: &ExtensionContext<'_>,
        request: Request,
        next: NextPrepareRequest<'_>,
    ) -> ServerResult<Request> {
        let gl = GrandLineContext::new(ctx);
        next.run(ctx, request.data(gl)).await
    }

    /// Cleanup GrandLineContext at the end of each request.
    async fn execute(
        &self,
        ctx: &ExtensionContext<'_>,
        operation_name: Option<&str>,
        next: NextExecute<'_>,
    ) -> Response {
        let mut r = next.run(ctx, operation_name).await;
        let gl = GrandLineContext::from_extension(ctx);
        if let Err(e) = gl.cleanup(r.errors.is_empty()).await {
            r.errors.push(ServerError::new(e.to_string(), None));
        }
        r
    }
}
