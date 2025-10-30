#![allow(dead_code)]

#[path = "../test_utils/mod.rs"]
mod test_utils;
pub use test_utils::*;

#[model]
pub struct User {
    pub name: String,
    #[has_one]
    pub person: Person,
    #[has_many]
    pub aliases: Alias,
    #[many_to_many]
    pub orgs: Org,
}
#[detail(User)]
fn resolver() {}
#[search(User)]
fn resolver() {
    (None, None)
}
#[count(User)]
fn resolver() {
    None
}
#[delete(User)]
fn resolver() {}

#[model]
pub struct Person {
    pub gender: String,
    pub user_id: String,
    #[belongs_to]
    pub user: User,
}
#[detail(Person)]
fn resolver() {}

#[model]
pub struct Alias {
    pub name: String,
    pub user_id: String,
}

#[model]
pub struct Org {
    pub name: String,
}
#[model]
pub struct UserInOrg {
    pub user_id: String,
    pub org_id: String,
}

#[derive(Default, MergedObject)]
pub struct Query(
    UserDetailQuery,
    UserSearchQuery,
    UserCountQuery,
    PersonDetailQuery,
);
#[derive(Default, MergedObject)]
pub struct Mutation(UserDeleteMutation);

pub struct Prepare {
    pub tmp: TmpDb,
    pub s: Schema<Query, Mutation, EmptySubscription>,
    pub id1: String,
    pub id2: String,
    pub pid1: String,
    pub pid2: String,
}

pub async fn prepare() -> Res<Prepare> {
    let tmp = tmp_db!(User, Person, Alias, Org, UserInOrg);
    let s = schema_qm::<Query, Mutation>(&tmp.db).finish();

    let u1 = db_create!(&tmp.db, User { name: "Olivia" });
    let u2 = db_create!(&tmp.db, User { name: "Peter" });
    let _ = db_soft_delete_by_id!(&tmp.db, User, &u2.id);

    let p1 = db_create!(
        &tmp.db,
        Person {
            gender: "Female",
            user_id: u1.id.clone(),
        }
    );
    let p2 = db_create!(
        &tmp.db,
        Person {
            gender: "Male",
            user_id: u2.id.clone(),
        }
    );
    let _ = db_soft_delete_by_id!(&tmp.db, Person, &p1.id);

    let _ = db_create!(
        &tmp.db,
        Alias {
            name: "Liv",
            user_id: u1.id.clone(),
        }
    );
    let a = db_create!(
        &tmp.db,
        Alias {
            name: "Fauxlivia",
            user_id: u1.id.clone(),
        }
    );
    let _ = db_soft_delete_by_id!(&tmp.db, Alias, &a.id);

    let o1 = db_create!(&tmp.db, Org { name: "Fringe" });
    let o2 = db_create!(&tmp.db, Org { name: "FBI" });
    let _ = db_soft_delete_by_id!(&tmp.db, Org, &o2.id);

    let _ = db_create!(
        &tmp.db,
        UserInOrg {
            user_id: u1.id.clone(),
            org_id: o1.id.clone(),
        }
    );
    let _ = db_create!(
        &tmp.db,
        UserInOrg {
            user_id: u1.id.clone(),
            org_id: o2.id.clone(),
        }
    );

    Ok(Prepare {
        tmp,
        s,
        id1: u1.id,
        id2: u2.id,
        pid1: p1.id,
        pid2: p2.id,
    })
}
