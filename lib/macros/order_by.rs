#[macro_export]
macro_rules! order_by {
    ($model:ident[$($order_by:tt),*]) => {{
        use grand_line::build::*;
        paste! {
            vec![$([<$model OrderBy>]::$order_by),*]
        }
    }};
}

#[macro_export]
macro_rules! order_by_some {
    ($model:ident[$($order_by:tt),*]) => {
        Some(order_by!($model[$($order_by),*]))
    };
}

#[macro_export]
macro_rules! order_by_some_or {
    ($o1:expr, $o2:expr) => {{
        let o1 = $o1;
        let o2 = $o2;
        if let Some(o1) = o1 {
            if o1.len() > 0 {
                Some(o1)
            } else {
                o2
            }
        } else {
            o2
        }
    }};
}
