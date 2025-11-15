use crate::prelude::*;

#[derive(Default, MergedObject)]
pub struct AuthMergedQuery(
    LoginSessionCurrentQuery,
    LoginSessionSearchQuery,
    LoginSessionCountQuery,
);

#[derive(Default, MergedObject)]
pub struct AuthMergedMutation(
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
