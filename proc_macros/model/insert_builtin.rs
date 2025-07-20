use crate::prelude::*;
use syn::{Fields, ItemStruct};

pub fn insert_builtin(a: &MacroAttr, mut struk: ItemStruct) -> ItemStruct {
    let fields = parse_unwrap_ref!(struk.fields => Fields::Named);

    fields.named.insert(
        0,
        field! {
            #[sea_orm(primary_key, column_type="String(StringLen::N(26))", auto_increment=false)]
            pub id: String
        },
    );

    if !a.no_created_at {
        fields.named.push(field! {
            pub created_at: DateTimeUtc
        });
        if !a.no_by_id {
            fields.named.push(field! {
                pub created_by_id: Option<String>
            });
        }
    }

    if !a.no_updated_at {
        fields.named.push(field! {
            pub updated_at: Option<DateTimeUtc>
        });
        if !a.no_by_id {
            fields.named.push(field! {
                pub updated_by_id: Option<String>
            });
        }
    }

    if !a.no_deleted_at {
        fields.named.push(field! {
            pub deleted_at: Option<DateTimeUtc>
        });
        if !a.no_by_id {
            fields.named.push(field! {
                pub deleted_by_id: Option<String>
            });
        }
    }

    struk
}
