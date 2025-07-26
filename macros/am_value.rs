#[macro_export]
macro_rules! am_value {
    ($am:ident.$k:ident) => {
        $am.$k
            .try_as_ref()
            .ok_or_else(|| ErrServer::DbAmField404(stringify!($k).to_string()))
    };
}
