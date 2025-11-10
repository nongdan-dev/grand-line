use super::prelude::*;

#[derive(Default, MergedObject)]
pub struct AuthenticateMergedQuery(LoginSessionCurrentQuery, LoginSessionSearchQuery);

#[derive(Default, MergedObject)]
pub struct AuthenticateMergedMutation(
    RegisterMutation,
    RegisterResolveMutation,
    LoginMutation,
    ForgotMutation,
    ForgotResolveMutation,
    AuthOtpResolveMutation,
);
