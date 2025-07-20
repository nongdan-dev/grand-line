use grand_line_proc_macros::input;

#[input]
pub struct Pagination {
    pub limit: Option<u64>,
    pub offset: Option<u64>,
}
