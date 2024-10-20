use crate::prelude::*;
use syn::{Fields, ItemStruct};

pub fn insert_builtin(a: &MacroAttr, mut struk: ItemStruct) -> ItemStruct {
    if a.no_builtin {
        return struk;
    }

    let id = gen_field! {
        #[sea_orm(primary_key, column_type="Text", auto_increment=false)]
        pub id: String
    };
    let created_at = gen_field! {
        pub created_at: DateTimeUtc
    };
    let updated_at = gen_field! {
        pub updated_at: Option<DateTimeUtc>
    };

    let fields = parse_unwrap_ref!(struk.fields => Fields::Named);
    fields.named.insert(0, id);
    fields.named.push(created_at);
    fields.named.push(updated_at);

    struk
}
