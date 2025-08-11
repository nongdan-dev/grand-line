#[path = "../test_utils/mod.rs"]
mod test_utils;
use test_utils::*;

#[tokio::test]
#[cfg_attr(feature = "serial", serial)]
async fn default() -> Result<(), Box<dyn Error + Send + Sync>> {
    mod test {
        use super::*;

        #[model]
        pub struct User {
            #[default("I love you")]
            pub a: String,
            #[default(3000)]
            pub b: i64,
            #[default(9999)]
            pub c: i64,
        }

        #[detail(User)]
        fn resolver() {}
    }
    use test::*;

    let db = db_1(User).await?;
    let s = schema_q::<UserDetailQuery>(&db);

    let u = am_create!(User { c: 9 }).insert(&db).await?;

    pretty_eq!(u.a, "I love you");
    pretty_eq!(u.b, 3000);
    pretty_eq!(u.c, 9);

    let q = r#"
    query test($id: ID!) {
        userDetail(id: $id) {
            a
            b
            c
        }
    }
    "#;
    let v = value!({
        "id": u.id,
    });
    let expected = value!({
        "userDetail": {
            "a": "I love you",
            "b": 3000,
            "c": 9,
        },
    });

    exec_assert(&s, q, Some(v), expected).await?;
    Ok(())
}
