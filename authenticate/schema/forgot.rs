use crate::prelude::*;

#[gql_input]
pub struct Forgot {
    pub email: String,
}

// #[create(AuthTicket, resolver_output)]
// async fn forgot() -> String {
//     // TODO: check anonymous not log in yet

//     t.id
// }

#[gql_input]
pub struct ForgotResolve {
    pub id: String,
    pub otp: String,
    pub password: String,
}

// #[create(AuthTicket, resolver_output)]
// async fn forgotResolve() -> LoginSessionGql {
//     // TODO: check anonymous not log in yet

//     s.into_gql(ctx).await?
// }
