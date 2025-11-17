# GrandLine

Rust macro framework to build graphql resolvers using `sea-orm` and `graphql-async` with powerful nested filter and relationship.

<p align="center">
  <img src="https://github.com/nongdan-dev/grand-line/blob/master/.md/banner.jpg?raw=true" alt="Grand Line One Piece"/>
</p>

### Examples

```rs
use grand_line::prelude::*;
use serde_json::to_string as json;

// create a sea orm model and graphql object
// id, created_at, updated_at, deleted_at... are inserted automatically
#[model]
pub struct Todo {
    pub content: String,
    pub done: bool,
}

// search Todo with filter, sort, pagination from client
// variables are generated automatically
#[search(Todo)]
fn resolver() {
    let f = json(&filter)?;
    let o = json(&order_by)?;
    let p = json(&page)?;
    println!("todoSearch filter={f} order_by={o} page={p}");
    (None, None)
}

// we can also have a custom name
// with extra filter, or default sort in the resolver as well
// the extra will be combined as and condition with the value from client
#[search(Todo)]
fn todo_search_2024() {
    let extra_filter = filter!(Todo {
        content_starts_with: "2024",
    });
    let default_order_by = order_by!(Todo [DoneAsc, ContentAsc]);
    (Some(extra_filter), Some(default_order_by))
}

// checkout the examples and documentation below for more
```

<p align="center">
  <img src="https://github.com/nongdan-dev/grand-line/blob/master/.md/altair.jpg?raw=true" alt="Altair screenshot"/>
</p>

- [Simple Todo example](https://github.com/nongdan-dev/grand-line/blob/master/examples/simple_todo/src/main.rs)
- [All examples](https://github.com/nongdan-dev/grand-line/blob/master/examples)
- [Tests](https://github.com/nongdan-dev/grand-line/blob/master/tests)

### Documentation

TODO:
