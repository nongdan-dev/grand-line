# GrandLine

Rust macro framework for building GraphQL APIs on top of `sea-orm` and `async-graphql` — with automatic CRUD resolvers, nested filtering, sorting, pagination, relationships, and soft-delete.

<p align="center">
  <img src="https://github.com/nongdan-dev/grand-line/blob/master/.md/banner.jpg?raw=true" alt="Grand Line One Piece"/>
</p>

- [Simple Todo example](https://github.com/nongdan-dev/grand-line/blob/master/examples/simple_todo/src/main.rs)
- [All examples](https://github.com/nongdan-dev/grand-line/blob/master/examples)
- [Tests](https://github.com/nongdan-dev/grand-line/blob/master/tests)

---

### Quick start

```rs
use grand_line::prelude::*;

#[model]
pub struct Todo {
    pub content: String,
    pub done: bool,
}

#[search(Todo)]
fn resolver() {
    (None, None)
}

#[gql_input]
pub struct TodoCreate {
    pub content: String,
}
#[create(Todo)]
fn resolver() {
    am_create!(Todo { content: data.content })
}
```

<p align="center">
  <img src="https://github.com/nongdan-dev/grand-line/blob/master/.md/altair.jpg?raw=true" alt="Altair screenshot"/>
</p>

That produces a `todoSearch` query with filter/sort/pagination, and a `todoCreate` mutation — all type-safe, all wired to the database.

---

### Contents

- [Core concepts](#core-concepts)
- [Model](#model)
  - [Auto-generated types](#auto-generated-types)
  - [Auto-added fields](#auto-added-fields)
  - [Field attributes](#field-attributes)
  - [Model options](#model-options)
  - [Enums](#enums)
- [CRUD resolvers](#crud-resolvers)
  - [Naming convention](#naming-convention)
  - [#\[search\]](#search)
  - [#\[count\]](#count)
  - [#\[detail\]](#detail)
  - [#\[create\]](#create)
  - [#\[update\]](#update)
  - [#\[delete\]](#delete)
- [Custom resolvers](#custom-resolvers)
- [Relationships](#relationships)
- [Filtering and sorting](#filtering-and-sorting)
- [Active model helpers](#active-model-helpers)
- [Error handling](#error-handling)
- [Transactions](#transactions)

---

### Core concepts

**Resolver bodies are blocks, not functions.** Every macro body is copied into a generated `let r = { ... }` expression. `return` does not work — use `?` to exit early:

```rs
#[query]
fn my_query() -> String {
    if some_condition {
        Err(MyErr::NotFound)?;  // early exit — NOT return
    }
    "ok".to_string()
}
```

**`tx` and `ctx` are injected automatically.** Every resolver receives:

- `tx` — a `&DatabaseTransaction` shared across the entire request (see [Transactions](#transactions))
- `ctx` — a `&Context<'_>` for look-ahead field selection and dataloader access

---

### Model

#### Auto-generated types

`#[model]` turns a plain struct into a complete sea-orm entity with a paired GraphQL type. For `struct Todo`:

| Type              | Description                                                                 |
| ----------------- | --------------------------------------------------------------------------- |
| `Todo`            | sea-orm `Entity` — use for queries: `Todo::find()`, `Todo::find_by_id(...)` |
| `TodoSql`         | Raw database row (`sea_orm::Model`)                                         |
| `TodoActiveModel` | Active model for inserts/updates                                            |
| `TodoGql`         | GraphQL output object (exposed as `"Todo"` in schema)                       |
| `TodoFilter`      | GraphQL filter input (see [Filtering and sorting](#filtering-and-sorting))  |
| `TodoOrderBy`     | GraphQL order-by enum (`ContentAsc`, `DoneDesc`, ...)                       |
| `TodoColumn`      | sea-orm column enum                                                         |

#### Auto-added fields

These fields are added to every model automatically:

| Field        | Type                    | Set on       |
| ------------ | ----------------------- | ------------ |
| `id`         | `String` (26-char ULID) | insert       |
| `created_at` | `DateTimeUtc`           | insert       |
| `updated_at` | `DateTimeUtc`           | every update |
| `deleted_at` | `Option<DateTimeUtc>`   | soft-delete  |

#### Field attributes

Attributes that go on individual fields inside `#[model]`:

**`#[default(...)]`** — value applied at insert when the field is omitted from `am_create!`:

```rs
#[model]
pub struct User {
    #[default("anonymous")]
    pub name: String,
    #[default(0)]
    pub score: i64,
}

let u = am_create!(User { score: 42 }).insert(tx).await?;
// u.name == "anonymous", u.score == 42
```

**`#[graphql(skip)]`** — hides a field from the GraphQL schema. Still stored in the database, accessible on `UserSql`, but invisible to clients:

```rs
#[model]
pub struct User {
    pub first_name: String,
    #[graphql(skip)]
    pub last_name: String,
}
```

**`#[sql_expr(...)]`** — replaces the column with a custom sea-query expression computed by the database:

```rs
#[model]
pub struct User {
    pub a: i64,
    #[sql_expr(Expr::col(Column::A).add(1000))]
    pub b: i64,  // b = a + 1000, computed in SQL
}
// insert a=1 → query b returns 1001
```

**`#[resolver(sql_dep = "col1, col2")]`** — virtual field resolved in Rust. `sql_dep` lists the columns that must be fetched from the DB to compute it. Pair with a free function named `resolve_{field_name}`:

```rs
#[model]
pub struct User {
    pub first_name: String,
    #[graphql(skip)]
    pub last_name: String,
    #[resolver(sql_dep = "first_name, last_name")]
    pub full_name: String,
}

async fn resolve_full_name(u: &UserGql, _: &Context<'_>) -> Res<String> {
    let first = u.first_name.clone().ok_or(CoreDbErr::GqlResolverNone)?;
    let last  = u.last_name.clone().ok_or(CoreDbErr::GqlResolverNone)?;
    Ok(format!("{first} {last}"))
}
```

`sql_dep` can reference `#[sql_expr]` fields too:

```rs
#[model]
pub struct User {
    pub a: i64,
    #[sql_expr(Expr::col(Column::A).add(1000))]
    pub b: i64,
    #[resolver(sql_dep = "a, b")]
    pub c: i64,
}
async fn resolve_c(u: &UserGql, _: &Context<'_>) -> Res<i64> {
    Ok(u.a.ok_or(CoreDbErr::GqlResolverNone)? + u.b.ok_or(CoreDbErr::GqlResolverNone)?)
}
// a=1 → b=1001 → c=1002
```

#### Model options

Pass as comma-separated arguments to skip auto-added fields:

```rs
#[model(no_deleted_at)]   // no soft-delete support
#[model(no_created_at)]   // no created_at / created_by_id
#[model(no_updated_at)]   // no updated_at / updated_by_id
#[model(no_by_id)]        // no *_by_id audit fields
```

#### Enums

**`#[gql_enum]`** — GraphQL-only enum, not stored in the database:

```rs
#[gql_enum]
pub enum Direction { Asc, Desc }
```

Derives: `Debug`, `Clone`, `Copy`, `Eq`, `PartialEq`, `Deserialize`, `Serialize`, `Enum`.

**`#[enunn]`** — enum stored in the database as `VARCHAR(255)` in snake_case, and also exposed as a GraphQL type:

```rs
#[enunn]
pub enum Status {
    Active,    // stored as "active"
    Inactive,  // stored as "inactive"
}

#[model]
pub struct Todo {
    pub status: Status,
}
```

Adds `EnumIter` and `DeriveActiveEnum` on top of `#[gql_enum]`.

---

### CRUD resolvers

#### Naming convention

When the function is named `resolver`, the GraphQL field name defaults to `{Model}{Operation}` in camelCase (e.g. `todoSearch`, `todoCreate`). Use any other name to override:

```rs
#[search(Todo)]
fn resolver() { ... }           // → todoSearch

#[search(Todo)]
fn todo_search_2024() { ... }   // → todoSearch2024
```

The input type for `#[create]` and `#[update]` is the PascalCase of the GraphQL field name:

| Function                               | GraphQL field | Input type   |
| -------------------------------------- | ------------- | ------------ |
| `fn resolver()` + `#[create(Todo)]`    | `todoCreate`  | `TodoCreate` |
| `fn todo_upsert()` + `#[create(Todo)]` | `todoUpsert`  | `TodoUpsert` |

The generated `MergedObject` struct follows the same pattern: `todoCreate` → `TodoCreateMutation`.

---

#### `#[search]`

Returns a paginated list. The body returns `(extra_filter, default_order_by)` — both are combined with the values sent by the client.

```rs
#[search(Todo)]
fn resolver() {
    (None, None)
}

// With server-side defaults:
#[search(Todo)]
fn todo_search_2024() {
    let extra  = filter!(Todo { content_starts_with: "2024" });
    let sort   = order_by!(Todo [DoneAsc, ContentAsc]);
    (Some(extra), Some(sort))
}
```

Auto-injected locals:

| Variable          | Type                       |
| ----------------- | -------------------------- |
| `filter`          | `Option<TodoFilter>`       |
| `order_by`        | `Option<Vec<TodoOrderBy>>` |
| `page`            | `Option<Pagination>`       |
| `include_deleted` | `Option<bool>`             |

`Pagination`:

```rs
pub struct Pagination {
    pub offset: Option<u64>,  // default 0
    pub limit:  Option<u64>,  // default 10, max 100
}
```

`include_deleted`: when omitted, the framework auto-detects from the filter — if the filter already references any `deletedAt` condition, the default exclude-deleted clause is skipped. Pass `true` to explicitly include all soft-deleted rows.

**Output**: `Vec<TodoGql>`

---

#### `#[count]`

Returns the number of matching records. The body returns an optional extra filter.

```rs
#[count(Todo)]
fn resolver() {
    None
}
```

Auto-injected locals: `filter: Option<TodoFilter>`, `include_deleted: Option<bool>`

**Output**: `u64`

---

#### `#[detail]`

Returns a single record by ID. The body runs before the fetch — use it for logging or pre-checks. No return value is needed; the framework fetches and returns the record.

```rs
#[detail(Todo)]
fn resolver() {
    println!("todoDetail id={id}");
}
```

Auto-injected locals: `id: String`, `include_deleted: Option<bool>`

**Output**: `Option<TodoGql>`

---

#### `#[create]`

Creates a record. The body must evaluate to a `TodoActiveModel`.

```rs
#[gql_input]
pub struct TodoCreate {
    pub content: String,
}

#[create(Todo)]
fn resolver() {
    am_create!(Todo { content: data.content })
}
```

Auto-injected locals: `data: TodoCreate` (type name = PascalCase of GraphQL field name)

**Output**: `TodoGql`

---

#### `#[update]`

Updates a record. The body must evaluate to a `TodoActiveModel`.

```rs
#[gql_input]
pub struct TodoUpdate {
    pub content: String,
}

#[update(Todo)]
fn resolver() {
    Todo::find_by_id(&id).exists_or_404(tx).await?;
    am_update!(Todo {
        id: id.clone(),
        content: data.content,
    })
}
```

Auto-injected locals: `id: String`, `data: TodoUpdate`

Use `resolver_inputs` to define fully custom inputs:

```rs
#[update(Todo, resolver_inputs)]
fn todo_toggle_done(id: String) {
    let todo = Todo::find_by_id(&id).one_or_404(tx).await?;
    am_update!(Todo {
        id: id.clone(),
        done: !todo.done,
    })
}
```

**Output**: `TodoGql`

---

#### `#[delete]`

Deletes a record. The body runs before deletion — use it for pre-delete validation. No return value needed.

```rs
#[delete(Todo)]
fn resolver() {
    Todo::find_by_id(&id).exists_or_404(tx).await?;
}
```

Auto-injected locals: `id: String`, `permanent: Option<bool>`

- `permanent: false` (default) — soft-delete: sets `deleted_at`, row stays in DB
- `permanent: true` — hard-delete: row is removed from DB

**Output**: `TodoGql` with only `id` populated.

```rs
#[delete(Todo, no_permanent_delete)]  // remove the permanent option entirely
```

---

### Custom resolvers

Use `#[query]` and `#[mutation]` for anything not covered by the CRUD macros. `ctx` and `tx` are injected automatically.

```rs
#[query]
fn todo_count_done() -> u64 {
    let f = filter!(Todo { done: true });
    f.into_select().count(tx).await?
}

#[mutation]
fn todo_delete_done() -> Vec<TodoGql> {
    let f = filter!(Todo { done: true });
    Todo::soft_delete_many()?
        .filter(f.clone().into_condition())
        .exec(tx)
        .await?;
    f.gql_select_id().all(tx).await?
}
```

These generate `TodoCountDoneQuery` / `TodoDeleteDoneMutation` structs for use in `MergedObject`.

---

### Relationships

Declare relationships as field attributes on `#[model]`. The framework resolves them with look-ahead — only the fields the client requests are fetched.

**`#[has_one]`** — the related model holds a `{owner}_id` foreign key:

```rs
#[model]
pub struct User {
    #[has_one]
    pub person: Person,
}

#[model]
pub struct Person {
    pub gender: String,
    pub user_id: String,  // foreign key
}
```

**`#[has_many]`** — same as `#[has_one]` but returns a list:

```rs
#[model]
pub struct User {
    #[has_many]
    pub aliases: Alias,
}
```

**`#[belongs_to]`** — the current model holds the foreign key:

```rs
#[model]
pub struct Alias {
    pub name: String,
    pub user_id: String,
    #[belongs_to]
    pub user: User,
}
```

**`#[many_to_many]`** — requires a join model with both foreign keys. The join model must be named `{A}In{B}` or `{B}In{A}`:

```rs
#[model]
pub struct User {
    #[many_to_many]
    pub orgs: Org,
}
#[model]
pub struct Org { pub name: String }
#[model]
pub struct UserInOrg {
    pub user_id: String,
    pub org_id: String,
}
```

#### Soft-delete and relationships

Related records with `deleted_at` set are excluded by default. Per-field `includeDeleted` overrides this in the GraphQL query:

```graphql
query {
    userDetail(id: "...") {
        # has_one: soft-deleted person is null by default
        person { gender }
        person(includeDeleted: true) { gender }

        # belongs_to: same
        user { name }
        user(includeDeleted: true) { name }

        # has_many / many_to_many: can also use filter directly
        orgs(filter: { deletedAt_ne: null }) { name }
        orgs(
            filter: { OR: [{ deletedAt: null }, { deletedAt_ne: null }] },
            orderBy: [NameAsc],
        ) { name }
    }
}
```

---

### Filtering and sorting

#### `filter!`

Builds a model filter. String literals are auto-converted to `String`, each field is wrapped in `Some(...)`:

```rs
let f = filter!(Todo { done: true });
let f = filter!(Todo { content_starts_with: "2024", done: false });

// Combine two filters with AND
let f = TodoFilter::combine_and(f1, f2);
```

Expands to `TodoFilter { done: Some(true), ..Default::default() }`.

Filter operators generated per column (e.g. for `content: String`):

```
content          content_eq       content_ne
content_in       content_not_in
content_gt       content_gte      content_lt    content_lte
content_like     content_starts_with            content_ends_with
```

`TodoFilter` also has top-level `and`, `or`, and `not` for composing nested conditions.

#### `order_by!`

Builds a sort list:

```rs
let sort = order_by!(Todo [DoneAsc, ContentAsc]);
// → vec![TodoOrderBy::DoneAsc, TodoOrderBy::ContentAsc]
```

Every column generates `{Field}Asc` and `{Field}Desc` variants.

---

### Active model helpers

#### `am_create!` / `am_update!` / `am_soft_delete!`

Build a sea-orm `ActiveModel` and apply system-field defaults. String literals are auto-converted to `String`, each field is wrapped in `Set(...)`.

```rs
// Generates id (ULID), sets created_at and updated_at
am_create!(Todo { content: "hello", done: false })

// Sets updated_at, requires id
am_update!(Todo { id: id.clone(), content: "new content" })

// Sets deleted_at and updated_at
am_soft_delete!(Todo { id: id.clone() })
```

#### Soft-delete queries

```rs
// Soft-delete one row by id
Todo::soft_delete_by_id(&id)?.exec(tx).await?;

// Soft-delete many rows with a custom filter
Todo::soft_delete_many()?
    .filter(condition)
    .exec(tx)
    .await?;
```

#### Select helpers

```rs
// Fetch one row or return a 404 error
let todo: TodoSql = Todo::find_by_id(&id).one_or_404(tx).await?;

// Assert a row exists or return a 404 error
Todo::find_by_id(&id).exists_or_404(tx).await?;

// Select only id (used internally by delete responses)
filter.gql_select_id().all(tx).await?
```

---

### Error handling

`#[grand_line_err]` derives all required traits for a custom error enum. Variants marked `#[client]` are forwarded to the GraphQL response as-is. All others — including standard library errors — are replaced with a generic internal server error so implementation details are never leaked to clients.

```rs
#[grand_line_err]
enum MyErr {
    #[error("record not found")]
    #[client]
    NotFound,

    #[error("something went wrong internally")]
    InternalProblem,  // client only sees "internal server error"
}
```

Use `?` to raise errors from any resolver body:

```rs
#[query]
fn my_query() -> String {
    if missing {
        Err(MyErr::NotFound)?;
    }
    "ok".to_string()
}
```

Downcast from a GraphQL response error's `source` field to read the error code:

```rs
let code = error.source
    .as_deref()
    .and_then(|e| e.downcast_ref::<GrandLineErr>())
    .map(|e| e.0.code());  // e.g. "NotFound"
```

---

### Transactions

`GrandLineExtension` manages a single lazy database transaction per GraphQL request:

- **Commit** — if the request finishes with no errors.
- **Rollback** — if any resolver returns an error; all DB writes in the request are undone.

Register it when building the schema:

```rs
Schema::build(Query::default(), Mutation::default(), EmptySubscription)
    .extension(GrandLineExtension)
    .data(Arc::new(db.clone()))
    .finish()
```

Both `.extension(GrandLineExtension)` and `.data(Arc::new(db))` are required.
