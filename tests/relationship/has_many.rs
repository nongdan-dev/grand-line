pub use grand_line::prelude::*;

#[tokio::test]
async fn resolver_default_name() -> Res<()> {
    mod test {
        use super::*;

        #[model]
        pub struct User {
            #[has_many(resolver)]
            pub aliases: Alias,
        }
        #[model]
        pub struct Alias {
            pub name: String,
            pub user_id: String,
        }

        #[many_resolver(Alias, parent = "User")]
        fn resolve_aliases() {
            let f = filter!(Alias {
                name: "Liv"
            });
            (Some(f), None)
        }

        #[detail(User)]
        fn resolver() {
        }
    }
    use test::*;

    let tmp = tmp_db!(User, Alias);
    let s = schema_q::<UserDetailQuery>(&tmp.db).finish();

    let u = am_create!(User).exec_without_ctx(&tmp.db).await?;
    am_create!(Alias {
        name: "Liv",
        user_id: u.id.clone(),
    })
    .exec_without_ctx(&tmp.db)
    .await?;
    am_create!(Alias {
        name: "Fauxlivia",
        user_id: u.id.clone(),
    })
    .exec_without_ctx(&tmp.db)
    .await?;

    let q = "
    query test($id: ID!) {
        userDetail(id: $id) {
            aliases {
                name
            }
        }
    }
    ";
    let v = value!({
        "id": u.id,
    });
    let expected = value!({
        "userDetail": {
            "aliases": [{
                "name": "Liv",
            }],
        },
    });

    exec_assert(&s, q, Some(v), &expected).await;
    tmp.drop().await
}

#[tokio::test]
async fn resolver_custom_fn() -> Res<()> {
    mod test {
        use super::*;

        #[model]
        pub struct User {
            #[has_many(resolver = "custom_aliases")]
            pub aliases: Alias,
        }
        #[model]
        pub struct Alias {
            pub name: String,
            pub user_id: String,
        }

        #[many_resolver(Alias, parent = "User")]
        fn custom_aliases() {
            let o = order_by!(Alias[NameDesc]);
            (None, Some(o))
        }

        #[detail(User)]
        fn resolver() {
        }
    }
    use test::*;

    let tmp = tmp_db!(User, Alias);
    let s = schema_q::<UserDetailQuery>(&tmp.db).finish();

    let u = am_create!(User).exec_without_ctx(&tmp.db).await?;
    am_create!(Alias {
        name: "Astrid",
        user_id: u.id.clone(),
    })
    .exec_without_ctx(&tmp.db)
    .await?;
    am_create!(Alias {
        name: "Walter",
        user_id: u.id.clone(),
    })
    .exec_without_ctx(&tmp.db)
    .await?;

    let q = "
    query test($id: ID!) {
        userDetail(id: $id) {
            aliases {
                name
            }
        }
    }
    ";
    let v = value!({
        "id": u.id,
    });
    let expected = value!({
        "userDetail": {
            "aliases": [{
                "name": "Walter",
            }, {
                "name": "Astrid",
            }],
        },
    });

    exec_assert(&s, q, Some(v), &expected).await;
    tmp.drop().await
}

#[tokio::test]
async fn has_many_returns_children() -> Res<()> {
    mod test {
        use super::*;

        #[model]
        pub struct User {
            #[has_many]
            pub aliases: Alias,
        }
        #[model]
        pub struct Alias {
            pub name: String,
            pub user_id: String,
        }

        #[detail(User)]
        fn resolver() {
        }
    }
    use test::*;

    let tmp = tmp_db!(User, Alias);
    let s = schema_q::<UserDetailQuery>(&tmp.db).finish();

    let u = am_create!(User).exec_without_ctx(&tmp.db).await?;
    am_create!(Alias {
        name: "Liv",
        user_id: u.id.clone(),
    })
    .exec_without_ctx(&tmp.db)
    .await?;

    let q = "
    query test($id: ID!) {
        userDetail(id: $id) {
            aliases {
                name
            }
        }
    }
    ";
    let v = value!({
        "id": u.id,
    });
    let expected = value!({
        "userDetail": {
            "aliases": [{
                "name": "Liv",
            }],
        },
    });

    exec_assert(&s, q, Some(v), &expected).await;
    tmp.drop().await
}
