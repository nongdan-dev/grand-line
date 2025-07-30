#![allow(dead_code)]

use super::prelude::*;

pub fn schema_q<Q>(db: &DatabaseConnection) -> Schema<Q, EmptyMutation, EmptySubscription>
where
    Q: ObjectType + Default + 'static,
{
    let sb = Schema::build(Q::default(), EmptyMutation, EmptySubscription);
    finish(sb, db)
}
pub fn schema_m<M>(db: &DatabaseConnection) -> Schema<EmptyQuery, M, EmptySubscription>
where
    M: ObjectType + Default + 'static,
{
    let sb = Schema::build(EmptyQuery::default(), M::default(), EmptySubscription);
    finish(sb, db)
}

pub fn schema_qm<Q, M>(db: &DatabaseConnection) -> Schema<Q, M, EmptySubscription>
where
    Q: ObjectType + Default + 'static,
    M: ObjectType + Default + 'static,
{
    let sb = Schema::build(Q::default(), M::default(), EmptySubscription);
    finish(sb, db)
}

fn finish<Q, M, S>(sb: SchemaBuilder<Q, M, S>, db: &DatabaseConnection) -> Schema<Q, M, S>
where
    Q: ObjectType + 'static,
    M: ObjectType + 'static,
    S: SubscriptionType + 'static,
{
    sb.extension(GrandLineExtension)
        .data(Arc::new(db.clone()))
        .finish()
}

#[derive(Default, SimpleObject)]
pub struct EmptyQuery {
    pub a: bool,
}
