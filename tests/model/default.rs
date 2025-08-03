#[path = "../test_utils/mod.rs"]
mod test_utils;
use test_utils::prelude::*;

#[tokio::test]
#[cfg_attr(feature = "serial", serial)]
async fn default() -> Result<(), Box<dyn Error>> {
    mod test {
        use super::*;

        #[model]
        pub struct Data {
            #[default("I love you")]
            pub a: String,
            #[default(3000)]
            pub b: i64,
            #[default(9999)]
            pub c: i64,
        }

        #[detail(Data)]
        fn resolver() {}
    }
    use test::*;

    let db = db_1(Data).await?;
    let d = am_create!(Data { c: 9 }).insert(&db).await?;

    pretty_eq!(d.a, "I love you");
    pretty_eq!(d.b, 3000);
    pretty_eq!(d.c, 9);

    let q = r#"
    query test($id: ID!) {
        dataDetail(id: $id) {
            a
            b
            c
        }
    }
    "#;
    let v = value!({
        "id": d.id,
    });
    let expected = value!({
        "dataDetail": {
            "a": "I love you",
            "b": 3000,
            "c": 9,
        },
    });

    let s = schema_q::<DataDetailQuery>(&db);
    exec_assert(s, q, v, expected).await?;
    Ok(())
}
