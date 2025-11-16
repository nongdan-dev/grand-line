pub use grand_line::prelude::*;

#[tokio::test]
async fn t() -> Res<()> {
    mod test {
        use super::*;

        #[model]
        pub struct User {
            #[has_one]
            pub person: Person,
        }
        #[model]
        pub struct Person {
            pub gender: String,
            pub user_id: String,
        }

        #[detail(User)]
        fn resolver() {}
    }
    use test::*;

    let tmp = tmp_db!(User, Person);
    let s = schema_q::<UserDetailQuery>(&tmp.db).finish();

    let u = db_create!(&tmp.db, User);
    let _ = db_create!(
        &tmp.db,
        Person {
            gender: "Unknown",
            user_id: u.id.clone(),
        },
    );

    let q = r#"
    query test($id: ID!) {
        userDetail(id: $id) {
            person {
                gender
            }
        }
    }
    "#;
    let v = value!({
        "id": u.id,
    });
    let expected = value!({
        "userDetail": {
            "person": {
                "gender": "Unknown",
            },
        },
    });

    exec_assert(&s, q, Some(&v), &expected).await;
    tmp.drop().await
}
