use grand_line::*;

// create the graphql input object for pagination
pagination!();

// create a sea orm model and graphql object
// id, created_at, updated_at will be inserted automatically
#[model]
pub struct Todo {
    pub name: String,
    pub description: String,
}

// create a graphql query search based on Todo model with filter, sort, pagination
#[search(Todo)]
fn handler() {
    next(None, None).await
}
// we can also have a custom name, add extra filter, or default sort in the handler
#[search(Todo, name = "search2024Todo")]
fn handler() {
    let f = filter!(Todo {
        name_starts_with: "2024",
    });
    let o = order_by!(Todo[NameAsc]);
    next(f, o).await
}

use async_graphql::{EmptyMutation, EmptySubscription, MergedObject, Schema};
use sea_orm::prelude::*;

#[derive(Default, MergedObject)]
struct Query(SearchTodoQuery, Search2024TodoQuery);
type MySchema = Schema<Query, EmptyMutation, EmptySubscription>;

#[tokio::main]
async fn main() {
    let schema = Schema::build(Query::default(), EmptyMutation, EmptySubscription)
        .data(init_db().await.unwrap())
        .finish();
    internal::quick_serve_axum!(schema);
}

async fn init_db() -> Result<DatabaseConnection, DbErr> {
    let db = Database::connect("sqlite::memory:").await?;
    db.execute_unprepared(
        "CREATE TABLE todo (
            id TEXT PRIMARY KEY NOT NULL,
            name TEXT NOT NULL,
            description TEXT NOT NULL,
            created_at TEXT NOT NULL DEFAULT (DATETIME('NOW')),
            updated_at TEXT NOT NULL DEFAULT (DATETIME('NOW'))
        );",
    )
    .await?;
    TodoEntity::insert_many(vec![
        active_model!(Todo {
            name: "2023 good bye",
            description: "a todo in 2023",
        }),
        active_model!(Todo {
            name: "2023 great",
            description: "another todo in 2023",
        }),
        active_model!(Todo {
            name: "2024 hello",
            description: "a todo in 2024",
        }),
        active_model!(Todo {
            name: "2024 awesome",
            description: "another todo in 2024",
        }),
    ])
    .exec(&db)
    .await?;
    Ok(db)
}
