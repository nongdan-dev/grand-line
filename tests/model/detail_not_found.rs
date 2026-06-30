pub use grand_line::prelude::*;

// detail resolver returns null when the record does not exist.
#[tokio::test]
async fn t() -> Res<()> {
    mod test {
        use super::*;

        #[model]
        pub struct User {
            pub name: String,
        }

        #[detail(User)]
        fn resolver() {
        }
    }
    use test::*;

    let tmp = tmp_db!(User);
    let s = schema_q::<UserDetailQuery>(&tmp.db).finish();

    let q = "
    query test($id: ID!) {
        userDetail(id: $id) {
            name
        }
    }
    ";
    let v = value!({
        "id": "nonexistent-id",
    });
    let expected = value!({
        "userDetail": null,
    });
    exec_assert(&s, q, Some(v), &expected).await;

    tmp.drop().await
}
