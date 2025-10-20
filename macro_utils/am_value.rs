/// To quickly get the active value from active model.
/// This is useful to get default data on field such as id before insert/update.
#[macro_export]
macro_rules! am_value {
    ($am:ident.$k:ident) => {
        $am.$k
            .try_as_ref()
            .ok_or_else(|| GrandLineInternalErr::DbAmField404 {
                model: am._model_name(),
                field: stringify!($k),
            })
    };
}
