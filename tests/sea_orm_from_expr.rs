#[cfg(test)]
mod tests {
    use async_graphql::{EmptyMutation, EmptySubscription, Schema};
    use grand_line::*;
    use sea_orm::prelude::*;
    use sea_orm::*;

    #[model]
    pub struct User {
        v: u32,
        #[sql_expr(Expr::col(Column::V).add(1000))]
        vv: u32,
    }
    #[detail(User)]
    fn resolver() {}

    #[tokio::test]
    async fn sea_orm_from_expr() -> Result<(), Box<dyn Error>> {
        let db = Database::connect("sqlite::memory:").await?;

        let gql = Schema::build(UserDetailQuery::default(), EmptyMutation, EmptySubscription)
            // TODO: add feature flag tracing
            .extension(GrandLineExtension)
            .data(Arc::new(db.clone()))
            .finish();

        let backend = db.get_database_backend();
        let schema = sea_orm::Schema::new(backend);
        let stmt = schema.create_table_from_entity(User);
        db.execute(backend.build(&stmt)).await?;

        // let x = User::find().select_only().select_column(UserColumn::Vv);

        let u1 = active_create!(User { v: 1 }).insert(&db).await?;
        let request =
            async_graphql::Request::new("query test($id: ID!) { userDetail(id: $id) { vv } }")
                .variables(async_graphql::Variables::from_value(
                    async_graphql::value!({ "id": u1.id }),
                ));

        let response = gql.execute(request).await;

        assert_eq!(response.errors, vec![]);
        assert_eq!(
            response.data,
            async_graphql::value!({ "userDetail": {"vv": 1001} })
        );

        Ok(())
    }
}
