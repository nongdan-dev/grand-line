pub use grand_line::prelude::*;

#[tokio::test]
async fn name_override() -> Res<()> {
    mod test {
        use super::*;

        #[model]
        pub struct User {
            #[graphql(name = "customX")]
            pub x_field: i64,
        }

        #[detail(User)]
        fn resolver() {
        }
    }
    use test::*;

    let tmp = tmp_db!(User);
    let s = schema_q::<UserDetailQuery>(&tmp.db).finish();

    let u = am_create!(User {
        x_field: 42,
    })
    .exec_without_ctx(&tmp.db)
    .await?;

    let q = "
    query test($id: ID!) {
        userDetail(id: $id) {
            customX
        }
    }
    ";
    let v = value!({ "id": u.id });
    let expected = value!({ "userDetail": { "customX": 42 } });

    exec_assert(&s, q, Some(v), &expected).await;
    tmp.drop().await
}

#[tokio::test]
async fn skip() -> Res<()> {
    mod test {
        use super::*;

        #[model]
        pub struct User {
            pub visible: i64,
            #[graphql(skip)]
            pub hidden: i64,
        }

        #[detail(User)]
        fn resolver() {
        }
    }
    use test::*;

    let tmp = tmp_db!(User);
    let s = schema_q::<UserDetailQuery>(&tmp.db).finish();
    let sdl = s.sdl();

    assert!(sdl.contains("visible"), "visible missing: {sdl}");
    assert!(!sdl.contains("hidden"), "skipped leaked: {sdl}");
    tmp.drop().await
}

#[tokio::test]
async fn doc_comment() -> Res<()> {
    mod test {
        use super::*;

        #[model]
        pub struct User {
            /// This is a description.
            pub x: i64,
        }

        #[detail(User)]
        fn resolver() {
        }
    }
    use test::*;

    let tmp = tmp_db!(User);
    let s = schema_q::<UserDetailQuery>(&tmp.db).finish();
    let sdl = s.sdl();

    assert!(sdl.contains("This is a description."), "doc missing: {sdl}");
    tmp.drop().await
}

#[tokio::test]
async fn deprecation() -> Res<()> {
    mod test {
        use super::*;

        #[model]
        pub struct User {
            #[graphql(deprecation = "use y instead")]
            pub x: i64,
            pub y: i64,
        }

        #[detail(User)]
        fn resolver() {
        }
    }
    use test::*;

    let tmp = tmp_db!(User);
    let s = schema_q::<UserDetailQuery>(&tmp.db).finish();
    let sdl = s.sdl();

    assert!(sdl.contains("@deprecated"), "deprecated missing: {sdl}");
    assert!(sdl.contains("use y instead"), "reason missing: {sdl}");
    tmp.drop().await
}

#[tokio::test]
async fn name_override_with_extra() -> Res<()> {
    mod test {
        use super::*;

        #[model]
        pub struct User {
            #[graphql(name = "renamedX", deprecation = "old field")]
            pub x_field: i64,
        }

        #[detail(User)]
        fn resolver() {
        }
    }
    use test::*;

    let tmp = tmp_db!(User);
    let s = schema_q::<UserDetailQuery>(&tmp.db).finish();

    let u = am_create!(User {
        x_field: 7,
    })
    .exec_without_ctx(&tmp.db)
    .await?;

    let q = "
    query test($id: ID!) {
        userDetail(id: $id) {
            renamedX
        }
    }
    ";
    let v = value!({ "id": u.id });
    let expected = value!({ "userDetail": { "renamedX": 7 } });

    exec_assert(&s, q, Some(v), &expected).await;

    let sdl = s.sdl();
    assert!(sdl.contains("@deprecated"), "deprecated missing: {sdl}");
    tmp.drop().await
}
