pub use grand_line::prelude::*;

#[model(history)]
pub struct Note {
    pub text: String,
}

#[tokio::test]
async fn record_history_on_create() -> Res<()> {
    let tmp = tmp_db!(Note, NoteHistory);

    let n = am_create!(Note { text: "hello" })
        .exec_without_ctx(&tmp.db)
        .await?;

    // exec_without_ctx skips history - count should be 0
    let count = NoteHistory::find().count(&tmp.db).await?;
    pretty_eq!(count, 0u64);

    // call record_history directly
    Note::record_history(&tmp.db, "create", &n, None).await?;

    let count = NoteHistory::find().count(&tmp.db).await?;
    pretty_eq!(count, 1u64);

    let h = NoteHistory::find().one(&tmp.db).await?.unwrap();
    pretty_eq!(h.entity_id, n.id);
    pretty_eq!(h.operation, "create");
    assert!(h.by_id.is_none());
    // data snapshot should contain the note's text
    pretty_eq!(h.data["text"], serde_json::Value::String("hello".to_owned()));

    tmp.drop().await
}

#[tokio::test]
async fn record_history_with_by_id() -> Res<()> {
    let tmp = tmp_db!(Note, NoteHistory);

    let n = am_create!(Note { text: "world" })
        .exec_without_ctx(&tmp.db)
        .await?;

    let user_id = "user_abc".to_owned();
    Note::record_history(&tmp.db, "update", &n, Some(user_id.clone())).await?;

    let h = NoteHistory::find().one(&tmp.db).await?.unwrap();
    pretty_eq!(h.operation, "update");
    pretty_eq!(h.by_id, Some(user_id));

    tmp.drop().await
}

#[tokio::test]
async fn gql_delete_soft_records_history() -> Res<()> {
    let tmp = tmp_db!(Note, NoteHistory);

    let n = am_create!(Note { text: "soft" })
        .exec_without_ctx(&tmp.db)
        .await?;

    Note::gql_delete(&tmp.db, &n.id, None, None).await?;

    // soft delete: record still in db with deleted_at set
    let still_exists = Note::find_by_id(&n.id).one(&tmp.db).await?;
    assert!(still_exists.map(|m| m.deleted_at.is_some()).unwrap_or(false));

    // history entry created
    let count = NoteHistory::find().count(&tmp.db).await?;
    pretty_eq!(count, 1u64);

    let h = NoteHistory::find().one(&tmp.db).await?.unwrap();
    pretty_eq!(h.entity_id, n.id);
    pretty_eq!(h.operation, "delete");

    tmp.drop().await
}

#[tokio::test]
async fn gql_delete_permanent_records_history() -> Res<()> {
    let tmp = tmp_db!(Note, NoteHistory);

    let n = am_create!(Note { text: "permanent" })
        .exec_without_ctx(&tmp.db)
        .await?;

    Note::gql_delete(&tmp.db, &n.id, Some(true), Some("admin".to_owned())).await?;

    // hard delete: record gone
    let gone = Note::find_by_id(&n.id).one(&tmp.db).await?;
    assert!(gone.is_none());

    // history entry still created (captured before deletion)
    let count = NoteHistory::find().count(&tmp.db).await?;
    pretty_eq!(count, 1u64);

    let h = NoteHistory::find().one(&tmp.db).await?.unwrap();
    pretty_eq!(h.entity_id, n.id);
    pretty_eq!(h.operation, "delete");
    pretty_eq!(h.by_id, Some("admin".to_owned()));
    pretty_eq!(h.data["text"], serde_json::Value::String("permanent".to_owned()));

    tmp.drop().await
}

#[tokio::test]
async fn no_history_without_flag() -> Res<()> {
    mod test {
        use super::*;

        #[model]
        pub struct Plain {
            pub x: i64,
        }
    }
    use test::*;

    // Plain entity should not have history
    assert!(!Plain::has_history());

    tmp_db!(Plain).drop().await
}
