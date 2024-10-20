#[macro_export]
macro_rules! filter_some_and {
    ($model:ident, $f1:expr, $f2:expr) => {{
        use grand_line::build::*;

        let f1 = $f1;
        let f2 = $f2;
        paste! {
            if let Some(f1) = f1 {
                if let Some(f2) = f2 {
                    Some([<$model Filter>] {
                        and: Some(vec![f1, f2]),
                        ..Default::default()
                    })
                } else {
                    Some(f1)
                }
            } else if let Some(f2) = f2 {
                Some(f2)
            } else {
                None
            }
        }
    }};
}
