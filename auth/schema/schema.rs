use crate::prelude::*;

#[derive(Default, MergedObject)]
pub struct AuthenticateMergedQuery(
    LoginSessionCurrentQuery,
    LoginSessionSearchQuery,
    LoginSessionCountQuery,
);

#[derive(Default, MergedObject)]
pub struct AuthenticateMergedMutation(
    RegisterMutation,
    RegisterResolveMutation,
    LoginMutation,
    ForgotMutation,
    ForgotResolveMutation,
    AuthOtpResolveMutation,
    LogoutMutation,
    LoginSessionDeleteMutation,
    LoginSessionDeleteAllMutation,
);
