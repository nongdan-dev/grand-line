/// Macro utils to quickly construct MyErr.
#[macro_export]
macro_rules! err {
    ($($v:tt)*) => {
        Err(GrandLineErr(Arc::new(MyErr::$($v)*)))
    };
}
