use super::prelude::*;

/// Extension to insert GrandLineContext on each request, then cleanup at the end of each request.
/// The extension also handle error automatically to only expose client errors to the client.
pub struct GrandLineExtension;

impl ExtensionFactory for GrandLineExtension {
    fn create(&self) -> Arc<dyn Extension> {
        Arc::new(GrandLineExtensionImpl)
    }
}

struct GrandLineExtensionImpl;

#[async_trait]
impl Extension for GrandLineExtensionImpl {
    /// Insert GrandLineContext on each request.
    async fn prepare_request(
        &self,
        ctx: &ExtensionContext<'_>,
        request: Request,
        next: NextPrepareRequest<'_>,
    ) -> ServerResult<Request> {
        let db = ctx
            .data::<Arc<DatabaseConnection>>()
            .map_err(|e| MyErr::CtxDb404 { inner: e.message })?;
        let gl = GrandLineContext::new(db.clone());
        next.run(ctx, request.data(Arc::new(gl))).await
    }

    /// Cleanup GrandLineContext at the end of each request.
    async fn execute(
        &self,
        ctx: &ExtensionContext<'_>,
        operation_name: Option<&str>,
        next: NextExecute<'_>,
    ) -> Response {
        let mut r = next.run(ctx, operation_name).await;
        match ctx._grand_line_context() {
            Ok(gl) => {
                if let Err(e) = gl.cleanup(!r.errors.is_empty()).await {
                    r.errors.push(e.into());
                }
            }
            Err(e) => {
                r.errors.push(e.into());
            }
        };
        for e in &mut r.errors {
            if e.source.is_none() {
                continue;
            }
            let gl = e
                .source
                .as_deref()
                .and_then(|e| e.downcast_ref::<GrandLineErr>());
            if let Some(GrandLineErr(gl)) = gl
                && gl.client()
            {
                e.extensions = Some(gl.extensions());
            } else {
                eprintln!("{}", e.message);
                e.message = MyErr::InternalServer.to_string();
                e.source = None;
                e.extensions = Some(MyErr::InternalServer.extensions())
            }
        }
        r
    }
}
