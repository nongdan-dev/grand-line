use crate::prelude::*;

// Build "postDetail.comments" style path from the current resolver position,
// skipping list indices so "postDetail.0.comments" becomes "postDetail.comments".
pub fn authz_field_path(ctx: &Context<'_>) -> String {
    let Some(node) = ctx.path_node.as_ref() else {
        return ctx.field().name().to_owned();
    };
    node.to_string_vec()
        .into_iter()
        .filter(|s| s.parse::<usize>().is_err())
        .collect::<Vec<_>>()
        .join(".")
}

async fn authz_row_compute<F>(ctx: &Context<'_>, field_path: &str) -> Res<Option<F>>
where
    F: Serialize + DeserializeOwned,
{
    let r = ctx.authz_role().await?;
    let Some(script) = r.row_policy.get(field_path) else {
        return Ok(None);
    };
    if script.is_null() {
        return Ok(None);
    }
    let Some(script) = script.as_str() else {
        return Err(MyErr::RowScript404.into());
    };
    let h = &ctx.authz_config().handlers;
    let Some(json) = h.execute_script(ctx, script).await? else {
        return Ok(None);
    };
    let f = F::from_json(json)?;
    Ok(Some(f))
}

// Per-request cache for authz_row results, keyed by (filter TypeId, field path).
// Avoids calling the handler repeatedly for the same field in the same request
// (e.g. N parents each resolving the same has_one relation with row auth).
type AuthzRowCacheMap = Mutex<HashMap<(TypeId, String), ArcAny>>;

async fn authz_row_cache_or_init(ctx: &Context<'_>) -> Res<Arc<AuthzRowCacheMap>> {
    ctx.cache(async || Ok(Mutex::new(HashMap::<(TypeId, String), ArcAny>::new())))
        .await
}

/// Retrieve the row-level filter defined in the current operation's policy.
///
/// Must be called after `authz_ensure_in_macro` has run. Returns None when
/// the matched policy has no row script (all rows accessible).
/// Results are cached per (filter type, field path) for the lifetime of the request.
#[async_trait]
pub trait AuthzRowContext {
    async fn authz_row<F>(&self) -> Res<Option<F>>
    where
        F: Serialize + DeserializeOwned + Clone + Send + Sync + 'static;

    // Like authz_row but returns None when there is no authz context (MissingMacro).
    // Used in relation resolvers where authz was checked by the parent root resolver.
    async fn authz_row_graceful<F>(&self) -> Res<Option<F>>
    where
        F: Serialize + DeserializeOwned + Clone + Send + Sync + 'static;
}

#[async_trait]
impl AuthzRowContext for Context<'_> {
    async fn authz_row<F>(&self) -> Res<Option<F>>
    where
        F: Serialize + DeserializeOwned + Clone + Send + Sync + 'static,
    {
        let field_path = authz_field_path(self);
        let cache_k = (TypeId::of::<F>(), field_path.clone());

        let cache = authz_row_cache_or_init(self).await?;
        {
            let guard = cache.lock().await;
            if let Some(cached) = guard.get(&cache_k) {
                let v = Arc::clone(cached)
                    .downcast::<Option<F>>()
                    .map_err(|_| MyErr::RowCacheDowncast)?;
                return Ok((*v).clone());
            }
        }

        let result = authz_row_compute::<F>(self, &field_path).await?;
        cache.lock().await.insert(cache_k, Arc::new(result.clone()) as ArcAny);
        Ok(result)
    }

    async fn authz_row_graceful<F>(&self) -> Res<Option<F>>
    where
        F: Serialize + DeserializeOwned + Clone + Send + Sync + 'static,
    {
        match self.authz_row::<F>().await {
            Err(e) if e.0.code() == MyErr::MissingMacro.code() => Ok(None),
            f => f,
        }
    }
}
