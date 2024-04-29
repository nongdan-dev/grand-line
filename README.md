# GrandLine

Rust macro framework to build graphql handlers using `sea-orm` and `graphql-async` with powerful nested filter and relationship.

<p align="center">
  <img src="https://github.com/nongdan-dev/grand-line/blob/master/doc/banner.jpg?raw=true" alt="GrandLine banner"/>
</p>

### Examples

```rs
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
```

- [Simple Todo example](examples/simple-todo/src/main.rs)
- [All examples](examples)

### Development

```sh
cargo install cargo-update dprint
```
