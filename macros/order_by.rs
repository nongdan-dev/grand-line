#[macro_export]
macro_rules! order_by {
    ($model:ident[$($order_by:tt),*]) => {
        grand_line::macros::paste! {
            vec![$([<$model OrderBy>]::$order_by),*]
        }
    };
}

#[macro_export]
macro_rules! order_by_some {
    ($model:ident[$($order_by:tt),*]) => {
        Some(order_by!($model[$($order_by),*]))
    };
}
