pub use grand_line_macros::order_by;
pub use grand_line_proc_macros::{active_model, filter, model, pagination, search};

pub mod internal {
    pub use grand_line_macros::quick_serve_axum;
    pub use grand_line_proc_macros::GrandLineModel;
    pub use paste::paste;

    pub fn ulid() -> String {
        ulid::Ulid::new().to_string()
    }
}
