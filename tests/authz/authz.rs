pub use grand_line::prelude::*;

#[query(authz)]
fn t1() -> String {
    "".to_owned()
}

#[query(authz(org, user))]
fn t2() -> String {
    "".to_owned()
}

#[tokio::test]
async fn t() -> Res<()> {
    Ok(())
}
