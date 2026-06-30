#![allow(ambiguous_glob_reexports, dead_code, unused_imports)]

#[path = "./setup.rs"]
mod setup;
pub use setup::*;
#[path = "./row_handlers.rs"]
mod row_handlers;
pub use row_handlers::*;

// ---------------------------------------------------------------------------
// Test models
// ---------------------------------------------------------------------------

#[model]
pub struct Post {
    pub org_id: String,
    #[has_many]
    pub comments: Comment,
    #[has_one]
    pub meta: PostMeta,
    #[many_to_many]
    pub tags: Tag,
}

#[model]
pub struct Comment {
    pub body: String,
    pub post_id: String,
    pub org_id: String,
    #[belongs_to]
    pub post: Post,
}

#[model]
pub struct PostMeta {
    pub text: String,
    pub post_id: String,
    pub org_id: String,
}

#[model]
pub struct Tag {
    pub name: String,
    pub org_id: String,
}

#[model]
pub struct PostInTag {
    pub post_id: String,
    pub tag_id: String,
}

// Root resolvers: authz checks happen here; relations inherit the cached context.
#[detail(Post, authz(realm = "org"))]
fn postDetail() {
}
#[detail(Comment, authz(realm = "org"))]
fn commentDetail() {
}

#[derive(Default, MergedObject)]
pub struct Q(PostDetailQuery, CommentDetailQuery);

// ---------------------------------------------------------------------------
// Setup
// ---------------------------------------------------------------------------

pub struct RowRelationSetup {
    pub tmp: TmpDb,
    pub schema: GraphQLSchema<Q, EmptyMutation, EmptySubscription>,
    pub org1_id: String,
    pub org2_id: String,
    // post1: org_id = org1, post2: org_id = org2
    pub post1_id: String,
    pub post2_id: String,
    // comment on post1 with org1 / org2
    pub comment_a_id: String,
    pub comment_b_id: String,
    // comment on post2 (for belongs_to no-match test)
    pub comment_on_post2_id: String,
    // meta for post1 (org1), meta for post2 (org2)
    pub meta1_id: String,
    // tags: tag1 = org1, tag2 = org2; both linked to post1
    pub tag1_id: String,
    pub tag2_id: String,
}

pub async fn row_relation_setup(row_pol: RowPolicy, cfg: AuthzConfig) -> Res<RowRelationSetup> {
    let org_impl = authz_org_impl::<Org>();
    let tmp = tmp_db!(
        User,
        LoginSession,
        Org,
        Role,
        UserInRole,
        Post,
        Comment,
        PostMeta,
        Tag,
        PostInTag
    );
    let s = schema_q::<Q>(&tmp.db).data(org_impl).data(cfg);

    let h = init_common_headers();
    let ua = Context::get_ua_raw(Context::axum_headers(&h))?;

    // Users
    let u1 = am_create!(User {
        email: "alice@example.com",
        password_hashed: rand_utils::password_hash("pw")?,
    })
    .exec_without_ctx(&tmp.db)
    .await?;

    let secret1 = rand_utils::secret();
    let ls1 = am_create!(LoginSession {
        user_id: u1.id.clone(),
        secret_hashed: rand_utils::secret_hash(&secret1),
        ip: "127.0.0.1",
        ua: ua.to_json()?,
    })
    .exec_without_ctx(&tmp.db)
    .await?;
    let token1 = rand_utils::qs_token(&ls1.id, &secret1)?;

    // Orgs
    let o1 = am_create!(Org {
        name: "Alpha"
    })
    .exec_without_ctx(&tmp.db)
    .await?;
    let o2 = am_create!(Org {
        name: "Beta"
    })
    .exec_without_ctx(&tmp.db)
    .await?;

    // Role for org1 with the given row_policy
    let r1 = am_create!(Role {
        name: "Admin",
        realm: "org",
        col_policy: col_policy_wildcard().to_json()?,
        row_policy: row_pol.to_json()?,
        org_id: Some(o1.id.clone()),
    })
    .exec_without_ctx(&tmp.db)
    .await?;
    am_create!(UserInRole {
        user_id: u1.id.clone(),
        role_id: r1.id.clone(),
        org_id: Some(o1.id.clone()),
    })
    .exec_without_ctx(&tmp.db)
    .await?;

    // Posts
    let p1 = am_create!(Post {
        org_id: o1.id.clone()
    })
    .exec_without_ctx(&tmp.db)
    .await?;
    let p2 = am_create!(Post {
        org_id: o2.id.clone()
    })
    .exec_without_ctx(&tmp.db)
    .await?;

    // Comments on post1: A (org1), B (org2)
    let ca = am_create!(Comment {
        body: "A",
        post_id: p1.id.clone(),
        org_id: o1.id.clone()
    })
    .exec_without_ctx(&tmp.db)
    .await?;
    let cb = am_create!(Comment {
        body: "B",
        post_id: p1.id.clone(),
        org_id: o2.id.clone()
    })
    .exec_without_ctx(&tmp.db)
    .await?;
    // Comment on post2 (for belongs_to no-match test)
    let cc = am_create!(Comment {
        body: "C",
        post_id: p2.id.clone(),
        org_id: o1.id.clone()
    })
    .exec_without_ctx(&tmp.db)
    .await?;

    // PostMeta: meta1 for post1 (org1), meta2 for post2 (org2)
    let m1 = am_create!(PostMeta {
        text: "M1",
        post_id: p1.id.clone(),
        org_id: o1.id.clone()
    })
    .exec_without_ctx(&tmp.db)
    .await?;
    am_create!(PostMeta {
        text: "M2",
        post_id: p2.id.clone(),
        org_id: o2.id.clone()
    })
    .exec_without_ctx(&tmp.db)
    .await?;

    // Tags: tag1 (org1), tag2 (org2); both linked to post1
    let t1 = am_create!(Tag {
        name: "T1",
        org_id: o1.id.clone()
    })
    .exec_without_ctx(&tmp.db)
    .await?;
    let t2 = am_create!(Tag {
        name: "T2",
        org_id: o2.id.clone()
    })
    .exec_without_ctx(&tmp.db)
    .await?;
    am_create!(PostInTag {
        post_id: p1.id.clone(),
        tag_id: t1.id.clone()
    })
    .exec_without_ctx(&tmp.db)
    .await?;
    am_create!(PostInTag {
        post_id: p1.id.clone(),
        tag_id: t2.id.clone()
    })
    .exec_without_ctx(&tmp.db)
    .await?;

    // Headers for user1 / org1 / role1
    let mut headers = h;
    headers.append(H_ORG_ID, h_str(&o1.id));
    headers.insert(H_AUTHORIZATION, h_bearer(&token1));
    headers.insert(H_ROLE_ID, h_str(&r1.id));

    Ok(RowRelationSetup {
        schema: s.data(headers).finish(),
        tmp,
        org1_id: o1.id,
        org2_id: o2.id,
        post1_id: p1.id,
        post2_id: p2.id,
        comment_a_id: ca.id,
        comment_b_id: cb.id,
        comment_on_post2_id: cc.id,
        meta1_id: m1.id,
        tag1_id: t1.id,
        tag2_id: t2.id,
    })
}
