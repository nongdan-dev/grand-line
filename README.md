# GrandLine

Rust macro framework for building GraphQL APIs on top of `sea-orm` and `async-graphql` — automatic CRUD resolvers, nested filtering, sorting, pagination, relationships, and soft-delete.

<p align="center">
  <img src="https://github.com/nongdan-dev/grand-line/blob/master/.md/banner.jpg?raw=true" alt="Grand Line One Piece"/>
</p>

- [Simple Todo example](https://github.com/nongdan-dev/grand-line/blob/master/examples/simple_todo/src/main.rs)
- [All examples](https://github.com/nongdan-dev/grand-line/blob/master/examples)
- [Tests](https://github.com/nongdan-dev/grand-line/blob/master/tests)

---

### Contents

<!-- START doctoc generated TOC please keep comment here to allow auto update -->
<!-- DON'T EDIT THIS SECTION, INSTEAD RE-RUN doctoc TO UPDATE -->
<!-- DON'T EDIT THIS SECTION, INSTEAD RE-RUN doctoc TO UPDATE -->

- [Quick start](#quick-start)
- [Model](#model)
  - [Auto-generated types](#auto-generated-types)
  - [Auto-added fields](#auto-added-fields)
  - [Field macro attributes](#field-macro-attributes)
  - [Input types](#input-types)
  - [Enums](#enums)
- [CRUD resolvers](#crud-resolvers)
  - [Naming convention](#naming-convention)
  - [`#[search]`](#search)
  - [`#[count]`](#count)
  - [`#[detail]`](#detail)
  - [`#[create]`](#create)
  - [`#[update]`](#update)
  - [`#[delete]`](#delete)
- [Custom resolvers](#custom-resolvers)
- [Resolver bodies](#resolver-bodies)
- [Context](#context)
  - [Core](#core)
  - [Auth (`grand_line_auth`)](#auth-grand_line_auth)
  - [Authz (`grand_line_authz`)](#authz-grand_line_authz)
- [Transactions](#transactions)
- [Relationships](#relationships)
  - [Soft-delete and relationships](#soft-delete-and-relationships)
- [Filtering and sorting](#filtering-and-sorting)
  - [`filter!`](#filter)
  - [`order_by!`](#order_by)
- [Active model helpers](#active-model-helpers)
  - [`am_create!` / `am_update!` / `am_soft_delete!`](#am_create--am_update--am_soft_delete)
  - [Soft-delete queries](#soft-delete-queries)
  - [Select helpers](#select-helpers)
- [Error handling](#error-handling)
- [Authentication](#authentication)
  - [Setup](#setup)
  - [Register](#register)
  - [Login](#login)
  - [Forgot password](#forgot-password)
  - [Session management](#session-management)
  - [`auth` attribute](#auth-attribute)
  - [Customizing behavior](#customizing-behavior)
- [Authorization](#authorization)
  - [Setup](#setup-1)
  - [`authz` attribute](#authz-attribute)
  - [Policy structure](#policy-structure)
- [Debug macro outputs](#debug-macro-outputs)

<!-- END doctoc generated TOC please keep comment here to allow auto update -->

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

### Model

#### Auto-generated types

`#[model]` turns a plain struct into a complete sea-orm entity with a paired GraphQL type. For `struct Todo` it will generate:

| Type              | Description                                                     |
| ----------------- | --------------------------------------------------------------- |
| `Todo`            | sea-orm `Entity`                                                |
| `TodoSql`         | sea-orm `Model`                                                 |
| `TodoColumn`      | sea-orm `Column`                                                |
| `TodoActiveModel` | sea-orm `ActiveModel`                                           |
| `TodoGql`         | async-graphql output object, will be named `Todo` in the schema |
| `TodoFilter`      | async-graphql filter input                                      |
| `TodoOrderBy`     | async-graphql order by enum                                     |

#### Auto-added fields

These fields are added to every model automatically:

| Field           | Type                    | Set on       |
| --------------- | ----------------------- | ------------ |
| `id`            | `String` (26-char ULID) | insert       |
| `created_at`    | `DateTimeUtc`           | insert       |
| `updated_at`    | `DateTimeUtc`           | every update |
| `deleted_at`    | `Option<DateTimeUtc>`   | soft-delete  |
| `created_by_id` | `Option<String>`        | manually     |
| `updated_by_id` | `Option<String>`        | manually     |
| `deleted_by_id` | `Option<String>`        | manually     |

They can be configured through model macro attributes as follows:

```rs
#[model(no_created_at)]   // no created_at / created_by_id
#[model(no_updated_at)]   // no updated_at / updated_by_id
#[model(no_deleted_at)]   // no deleted_at / deleted_by_id (also disable soft-delete on this model)
#[model(no_by_id)]        // no *_by_id
```

#### Field macro attributes

**`#[default(...)]`** — value applied at insert when the field is omitted from `am_create!`, can be any valid rust expression:

```rs
#[model]
pub struct Todo {
    pub content: String,
    #[default(false)]
    pub done: bool,
    // Alternatively, we can pass any other valid rust expression such as a function call.
    // A function call can be useful as the expression can make runtime computation.
    // We can define the function below or imported from somewhere.
    #[default(days_from_now(7))]
    pub due_at: DateTimeUtc,
}

fn days_from_now(n: i64) -> DateTimeUtc {
    Utc::now() + Duration::days(days)
}

let t = am_create!(Todo { content: "Update documentation" }).insert(tx).await?;
// t.done == false, t.due_at == now + 7 days
```

**`#[graphql(skip)]`** — hides a field from the GraphQL schema. Still stored in the database, accessible on `UserSql`, but invisible to clients:

```rs
#[model]
pub struct User {
    pub email: String,
    #[graphql(skip)]
    pub password_hashed: String,
}
```

**`#[sql_expr(...)]`** — mark this field as GraphQL-only field without actual sea-orm column. It will be resolved as a computed column from a sea-query expression, evaluated by the database at query time:

```rs
#[model]
pub struct Product {
    pub price: f64,
    pub discount_percentage: f64,
    // Not stored in DB — computed as price * (1 - discount_percentage / 100).
    // We can use Column:: here as it is in the same scope with the model definition.
    #[sql_expr(Expr::col(Column::Price).mul(
        Expr::val(1.0).sub(Expr::col(Column::DiscountPercentage).div(100.0))
    ))]
    pub discounted_price: f64,
    // Alternatively, we can pass any other valid rust expression such as a function call.
    // A function call can be useful as the expression can make runtime computation.
    // We can define the function below or imported from somewhere.
    #[sql_expr(expr_discounted_price())]
    pub discounted_price2: f64,
}

// Here we will need to use ProductColumn:: alias instead
// because the function is outside of the model definition scope.
fn expr_discounted_price() -> SimpleExpr {
    Expr::col(ProductColumn::Price).mul(
        Expr::val(1.0).sub(Expr::col(ProductColumn::DiscountPercentage).div(100.0))
    )
}

// insert price=200.0, discount_percentage=25.0 → query discounted_price returns 150.0
```

**`#[resolver(sql_dep = "col1, col2")]`** — mark this field as GraphQL-only field without actual sea-orm column. It requires a function in the same scope named `resolve_{field_name}`. `sql_dep` contains the columns that must be fetched from the DB to compute it:

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

#### Input types

**`#[gql_input]`** — defines a GraphQL input object. Use this for any mutation input, not just CRUD inputs:

```rs
#[gql_input]
pub struct TodoCreate {
    pub content: String,
    pub done: bool,
}
```

#### Enums

**`#[gql_enum]`** — shortcut to create a GraphQL-only enum, not stored in the database:

```rs
#[gql_enum]
pub enum Direction { Asc, Desc }
```

**`#[sql_enum]`** — shortcut to combine of sea-orm enum and async-graphql enum. It will be stored in the database as `VARCHAR(255)` in snake_case, and also exposed as a GraphQL enum:

```rs
#[sql_enum]
pub enum Status {
    Active,    // stored as "active"
    Inactive,  // stored as "inactive"
}

#[model]
pub struct Todo {
    // now this enum can also be used as a db model column
    pub status: Status,
}
```

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

| Function                           | GraphQL field | Input type   |
| ---------------------------------- | ------------- | ------------ |
| `#[create(Todo)] fn resolver()`    | `todoCreate`  | `TodoCreate` |
| `#[create(Todo)] fn todo_upsert()` | `todoUpsert`  | `TodoUpsert` |

It will generate an async-graphql object follows the same pattern: `todoCreate` → `TodoCreateMutation`.

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
    let extra = filter!(Todo { content_starts_with: "2024" });
    let sort  = order_by!(Todo [DoneAsc, ContentAsc]);
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

`page`:

```rs
pub struct Pagination {
    pub offset: Option<u64>,
    pub limit: Option<u64>,
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

Auto-injected locals:

| Variable          | Type                 |
| ----------------- | -------------------- |
| `filter`          | `Option<TodoFilter>` |
| `include_deleted` | `Option<bool>`       |

**Output**: `u64`

---

#### `#[detail]`

Returns a single record by ID. The body runs before the fetch — use it for logging or pre-checks. No return value needed.

```rs
#[detail(Todo)]
fn resolver() {
    println!("todoDetail id={id}");
}
```

Auto-injected locals:

| Variable          | Type           |
| ----------------- | -------------- |
| `id`              | `String`       |
| `include_deleted` | `Option<bool>` |

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

Auto-injected locals:

| Variable | Type                                 |
| -------- | ------------------------------------ |
| `data`   | PascalCase of the GraphQL field name |

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

Auto-injected locals:

| Variable | Type                                 |
| -------- | ------------------------------------ |
| `id`     | `String`                             |
| `data`   | PascalCase of the GraphQL field name |

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

Auto-injected locals:

| Variable    | Type           |
| ----------- | -------------- |
| `id`        | `String`       |
| `permanent` | `Option<bool>` |

- `permanent: false` (default) — soft-delete: sets `deleted_at`, row stays in DB
- `permanent: true` — hard-delete: row is removed from DB

**Output**: `TodoGql` with only `id` populated.

**Configuration** through macro attributes:

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

These generate `TodoCountDoneQuery` / `TodoDeleteDoneMutation` structs later use in async-graphql `MergedObject`.

---

### Resolver bodies

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

**`ctx` and `tx` are injected automatically.** Every resolver receives:

- `ctx` — a `&Context<'_>` async-graphql context with enhanced traits included through imported prelude (see [Context](#context))
- `tx` — a `&DatabaseTransaction` shared across the entire request (see [Transactions](#transactions))

Use **`resolver_inputs`** to define fully custom inputs:

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

---

### Context

`ctx` is a `&Context<'_>` injected into every resolver. Several helper traits extend it with framework-specific methods.

#### Core

```rs
ctx.tx().await?           // Arc<DatabaseTransaction> — the request transaction (also available as injected `tx`)
ctx.cache(|| async { ... }).await?  // Arc<T> — per-request cache keyed by type T; closure runs only on first call
```

#### Auth (`grand_line_auth`)

| Method                                       | Returns                            | Description                                                            |
| -------------------------------------------- | ---------------------------------- | ---------------------------------------------------------------------- |
| `ctx.auth().await?`                          | `String`                           | Current user's `id`; errors with `Unauthenticated` if no valid session |
| `ctx.auth_with_cache().await?`               | `Arc<Option<LoginSessionMinimal>>` | Current session, or `None` if unauthenticated; cached per request      |
| `ctx.auth_ensure_authenticated().await?`     | `()`                               | Errors if the request has no valid session                             |
| `ctx.auth_ensure_not_authenticated().await?` | `()`                               | Errors if the request already has a valid session                      |

The `auth_ensure_*` methods are called automatically by the `#[query(auth)]` / `#[mutation(auth(unauthenticated))]` attributes. Call them manually only when you need conditional logic.

#### Authz (`grand_line_authz`)

| Method                          | Returns           | Description                                                                                             |
| ------------------------------- | ----------------- | ------------------------------------------------------------------------------------------------------- |
| `ctx.authz().await?`            | `String`          | Verified `org_id` from `X-Org-Id` header; only valid inside org-scoped `authz(scope = "...")` resolvers |
| `ctx.authz_role().await?`       | `RoleSql`         | The matched `Role` row; valid inside any `authz(...)` resolver                                          |
| `ctx.org_unauthorized().await?` | `Arc<OrgMinimal>` | Resolves the org from `X-Org-Id` header without checking user auth; cached per request                  |

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
pub struct Org {
    pub name: String
}
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
        # has_one / belongs_to: soft-deleted record is null by default
        person { gender }
        person(includeDeleted: true) { gender }

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

### Authentication

The `grand_line_auth` package provides email + password authentication with OTP (one-time password) verification for register and forgot-password flows.

#### Setup

Register the built-in queries and mutations by merging `AuthMergedQuery` and `AuthMergedMutation` into your schema, and provide an `AuthConfig`:

```rs
use grand_line::prelude::*;

#[derive(Default, MergedObject)]
pub struct Query(AuthMergedQuery, /* your own queries */);

#[derive(Default, MergedObject)]
pub struct Mutation(AuthMergedMutation, /* your own mutations */);

let schema = Schema::build(Query::default(), Mutation::default(), EmptySubscription)
    .extension(GrandLineExtension)
    .data(Arc::new(db.clone()))
    .data(AuthConfig::default())
    .finish();
```

The following tables must be created in the database:

```rs
tmp_db!(User, AuthOtp, LoginSession)
```

**Note**: `User` only has basic fields (`email`, `password_hashed`). It is not extendable — add a second table (e.g. `UserProfile`) with a `user_id` foreign key to store extra fields.

#### Register

Registration is a two-step OTP flow:

**Step 1** — call `register`, which creates a pending OTP record and triggers `on_otp_create` (where you send the OTP code by email):

```graphql
mutation {
    register(data: { email: "user@example.com", password: "Str0ngP@ssw0rd?" }) {
        secret   # save this — needed in step 2
    }
}
```

**Step 2** — call `registerResolve` with the OTP code the user received, plus the `id` and `secret` from step 1:

```graphql
mutation {
    registerResolve(data: { id: "...", secret: "...", otp: "123456" }) {
        secret   # session token — pass as Authorization: Bearer {secret}
        inner { userId }
    }
}
```

On success: the `User` is created, a `LoginSession` is opened, and the session token is returned.

#### Login

Single-step: verify email + password and open a session:

```graphql
mutation {
    login(data: { email: "user@example.com", password: "123123" }) {
        secret   # session token
        inner { userId }
    }
}
```

The session token must be sent on subsequent requests:

```
Authorization: Bearer {secret}
```

#### Forgot password

Same two-step OTP flow as register:

**Step 1**:

```graphql
mutation {
    forgot(data: { email: "user@example.com" }) {
        secret
    }
}
```

**Step 2** — provide the OTP + new password:

```graphql
mutation {
    forgotResolve(data: { id: "...", secret: "...", otp: "123456" }, password: "NewP@ssw0rd!") {
        secret
        inner { userId }
    }
}
```

On success: the password is updated and a new `LoginSession` is opened.

#### Session management

```graphql
# current session (requires auth)
query { loginSessionCurrent { userId ip } }

# all sessions for current user (requires auth)
query { loginSessionSearch { userId ip ua } }
query { loginSessionCount }

# delete a specific session by id (requires auth)
mutation { loginSessionDelete(id: "...") { id } }

# delete all sessions for current user (requires auth)
mutation { loginSessionDeleteAll }

# delete current session (requires auth)
mutation { logout { id } }
```

#### `auth` attribute

Add to any resolver macro to enforce authentication. Use `ctx.auth().await?` (see [Context](#context)) to read the current user's ID inside the resolver:

```rs
// Requires a valid session token
#[query(auth)]
fn my_profile() -> UserGql {
    let user_id = ctx.auth().await?;
    User::find_by_id(&user_id).gql_select(ctx)?.one_or_404(tx).await?
}

// Requires the user to NOT be authenticated (for login/register endpoints)
#[mutation(auth(unauthenticated))]
fn register() -> AuthOtpWithSecret { ... }

// Works on all CRUD macros too
#[search(Todo, auth)]
fn resolver() { (None, None) }

#[create(Todo, auth)]
fn resolver() { am_create!(Todo { ... }) }
```

#### Customizing behavior

Implement `AuthHandlers` to hook into the auth lifecycle:

```rs
struct MyHandlers;

#[async_trait]
impl AuthHandlers for MyHandlers {
    // See AuthHandlers for reference
}

let config = AuthConfig {
    handlers: Arc::new(MyHandlers),
    ..Default::default()
    // See AuthConfig for reference
};
```

---

### Authorization

The `grand_line_authz` package provides role-based access control with organization scoping and fine-grained policy checks on GraphQL inputs and outputs.

#### Setup

Add the required tables and provide `AuthzConfig`:

```rs
tmp_db!(Org, Role, UserInRole)

let schema = Schema::build(Query::default(), Mutation::default(), EmptySubscription)
    .extension(GrandLineExtension)
    .data(Arc::new(db.clone()))
    .data(AuthConfig::default())   // auth is required alongside authz
    .data(AuthzConfig::default())
    .finish();
```

**Note**: `Org` only has basic fields (`name`). It is not extendable — add a second table (e.g. `OrgProfile`) with an `org_id` foreign key to store extra fields.

Roles are stored in the `Role` table. The `scope` field groups roles into named categories:

```rs
// Org-scoped role: belongs to a specific org
am_create!(Role {
    name: "Org Admin",
    scope: "admin",
    org_id: Some(org_id.clone()),
    operations: operations.to_json()?,
}).insert(tx).await?;

// System-wide role: no org
am_create!(Role {
    name: "System Admin",
    scope: "system",
    operations: operations.to_json()?,
}).insert(tx).await?;

// Assign to a user
am_create!(UserInRole {
    user_id: user_id.clone(),
    role_id: role_id.clone(),
    org_id: Some(org_id.clone()),  // must match the role's org_id
}).insert(tx).await?;
```

#### `authz` attribute

Add to any resolver macro. Two modes:

**Org-scoped** — checks that the current user has a role with the given `scope` inside the org from the `X-Org-Id` request header. Use `ctx.authz().await?` to get the verified org ID:

```rs
// Request must include: Authorization: Bearer {token}  and  X-Org-Id: {org_id}
#[query(authz(scope = "admin"))]
fn org_dashboard() -> OrgGql {
    let org_id = ctx.authz().await?;
    Org::find_by_id(&org_id).gql_select(ctx)?.one_or_404(tx).await?
}
```

**System-wide** — checks that the current user has a role with the given `scope` globally (no org required):

```rs
// Request must include: Authorization: Bearer {token}
#[query(authz(scope = "system", skip_org))]
fn system_dashboard() -> String {
    "ok".to_string()
}
```

Use `ctx.authz_role().await?` inside any `authz`-guarded resolver to get the matched `Role` record (see [Context](#context)).

Works on all resolver macros:

```rs
#[search(Todo, authz(scope = "admin"))]
fn resolver() { (None, None) }

#[create(Todo, authz(scope = "admin"))]
fn resolver() { am_create!(Todo { ... }) }
```

#### Policy structure

Each `Role` has an `operations` field — a JSON-encoded `PolicyOperations` map that controls what the role is allowed to do:

```rs
pub type PolicyOperations = HashMap<String, PolicyOperation>;

pub struct PolicyOperation {
    pub inputs: PolicyField,   // which GraphQL arguments are allowed
    pub output: PolicyField,   // which GraphQL response fields are allowed
}

pub struct PolicyField {
    pub allow: bool,
    pub children: Option<PolicyFields>,  // HashMap<String, PolicyField>
}
```

The key in `PolicyOperations` is the GraphQL operation name, or `"*"` to match all operations.

**Wildcards in `PolicyFields`**:

| Key    | Meaning                            |
| ------ | ---------------------------------- |
| `"*"`  | Allow any direct child field       |
| `"**"` | Allow any nested field recursively |

**Example — wildcard policy (allow everything)**:

```rs
let all = PolicyField { allow: true, children: Some(hashmap! {
    "**".to_owned() => PolicyField { allow: true, children: None },
}) };
let operations: PolicyOperations = hashmap! {
    "*".to_owned() => PolicyOperation { inputs: all.clone(), output: all },
};
role.operations = operations.to_json()?;
```

**Example — restricted policy (only allow specific fields)**:

```rs
let operations: PolicyOperations = hashmap! {
    "todoSearch".to_owned() => PolicyOperation {
        inputs: PolicyField { allow: true, children: Some(hashmap! {
            "filter".to_owned() => PolicyField { allow: true, children: Some(hashmap! {
                "**".to_owned() => PolicyField { allow: true, children: None },
            }) },
        }) },
        output: PolicyField { allow: true, children: Some(hashmap! {
            "id".to_owned()      => PolicyField { allow: true, children: None },
            "content".to_owned() => PolicyField { allow: true, children: None },
        }) },
    },
};
```

The policy check runs automatically before the resolver body executes. If the inputs or requested output fields are not allowed, the framework returns an `unauthorized` error.

---

### Debug macro outputs

To inspect the code generated by macros, set the environment variable `DEBUG_MACRO=1` and enable one of the following feature flags:

- `debug_macro_cli` — prints generated code to stdout during the build.
- `debug_macro_file` — writes generated code to files under `target/grand-line/` during the build. To avoid stale output, clear the folder before building.
