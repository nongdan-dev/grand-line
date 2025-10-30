use crate::prelude::*;

#[derive(Default, MergedObject)]
pub struct AuthenticateMergedQuery(LoginSessionCurrentQuery);

#[derive(Default, MergedObject)]
pub struct AuthenticateMergedMutation(
    RegisterMutation,
    RegisterResolveMutation,
    LoginMutation,
    ForgotMutation,
    ForgotResolveMutation,
);
