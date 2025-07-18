# GrandLine

Rust macro framework to build graphql resolvers using `sea-orm` and `graphql-async` with powerful nested filter and relationship.

<p align="center">
  <img src="https://github.com/nongdan-dev/grand-line/blob/master/doc/banner.jpg?raw=true" alt="GrandLine banner"/>
</p>

### Examples

```rs
use grand_line::*;
use serde_json::to_string as json;

// create a sea orm model and graphql object
// id, created_at, updated_at... will be inserted automatically
#[model]
pub struct Todo {
    pub content: String,
    pub done: bool,
}

// search Todo with filter, sort, pagination
#[search(Todo)]
fn resolver() {
    println!(
        "todoSearch filter={} order_by={} page={}",
        json(&filter)?,
        json(&order_by)?,
        json(&page)?,
    );
    (None, None)
}

// we can also have a custom name
// with extra filter, or default sort in the resolver as well
#[search(Todo)]
fn todoSearch2024() {
    let f = filter!(Todo {
        content_starts_with: "2024",
    });
    let o = order_by!(Todo [DoneAsc, ContentAsc]);
    (f, o)
}

// checkout the examples and documentation for more
```

<p align="center">
  <img src="https://github.com/nongdan-dev/grand-line/blob/master/doc/altair.jpg?raw=true" alt="Altair screenshot"/>
</p>

- [Simple Todo example](https://github.com/nongdan-dev/grand-line-examples/blob/master/simple-todo/src/main.rs)
- [All examples](https://github.com/nongdan-dev/grand-line-examples)

### Documentation

TODO:
