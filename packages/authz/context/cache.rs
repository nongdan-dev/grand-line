use crate::prelude::*;

#[derive(FromQueryResult)]
pub struct OrgMinimal {
    pub id: String,
}
impl OrgMinimal {
    pub fn select(q: Select<Org>) -> Selector<SelectModel<Self>> {
        q.select_only()
            .column(OrgColumn::Id)
            .into_model::<OrgMinimal>()
    }
}
