#[path = "../test_utils/mod.rs"]
mod test_utils;
use test_utils::prelude::*;

#[tokio::test]
#[cfg_attr(feature = "serial", serial)]
async fn default() -> Result<(), Box<dyn Error>> {
    mod test {
        use super::*;

        #[model]
        pub struct User {
            first_name: String,
            middle_name: String,
            last_name: String,
            #[resolver(sql_dep=first_name+middle_name+last_name)]
            full_name: String,
        }

        async fn resolve_full_name(
            u: &UserGql,
            _: &Context<'_>,
        ) -> Result<String, Box<dyn Error + Send + Sync>> {
            let full_name = vec![
                u.try_first_name()?,
                u.try_middle_name()?,
                u.try_last_name()?,
            ]
            .join(" ");
            Ok(full_name)
        }

        #[detail(User)]
        fn resolver() {}
    }
    use test::*;

    let db = db_1(User).await?;
    let d = am_create!(User {
        first_name: "Olivia",
        middle_name: "Anna",
        last_name: "Dunham",
    })
    .insert(&db)
    .await?;

    let q = r#"
    query test($id: ID!) {
        userDetail(id: $id) {
            fullName
        }
    }
    "#;
    let v = value!({
        "id": d.id,
    });
    let expected = value!({
        "userDetail": {
            "fullName": "Olivia Anna Dunham",
        },
    });

    let s = schema_q::<UserDetailQuery>(&db);
    exec_assert(s, q, v, expected).await?;
    Ok(())
}
