use grand_line::prelude::*;

#[model]
pub struct Task {
    pub title: String,
    pub assignee_id: String,
    pub org_id: String,
}

#[query(authz(realm = "org"))]
fn tasks(order_by: Option<Vec<TaskOrderBy>>) -> Vec<TaskGql> {
    let filter: TaskFilter = ctx.authz_row().await?.unwrap_or_default();
    let q = order_by
        .unwrap_or_default()
        .into_iter()
        .fold(filter.into_select(), |q, o| o.chain_select(q));
    q.gql_select(ctx)?.all(tx).await?
}
