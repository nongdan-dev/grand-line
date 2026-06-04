use grand_line::prelude::*;

#[model(no_by_id)]
pub struct User {
    pub email: String,
    #[graphql(skip)]
    pub password_hashed: String,
    #[default("")]
    pub display_name: String,
}

impl AuthUser for User {
    fn email_col() -> UserColumn {
        UserColumn::Email
    }
    fn password_col() -> UserColumn {
        UserColumn::PasswordHashed
    }
    fn get_email(m: &UserSql) -> &str {
        &m.email
    }
    fn get_password_hashed(m: &UserSql) -> &str {
        &m.password_hashed
    }
}
