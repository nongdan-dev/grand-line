use crate::prelude::*;

#[derive(FromQueryResult)]
pub struct LoginSessionMinimal {
    pub id: String,
    pub secret: String,
    pub user_id: String,
}
impl LoginSessionMinimal {
    pub fn select(q: Select<LoginSession>) -> Selector<SelectModel<Self>> {
        q.select_only()
            .column(LoginSessionColumn::Id)
            .column(LoginSessionColumn::Secret)
            .column(LoginSessionColumn::UserId)
            .into_model::<LoginSessionMinimal>()
    }
}
