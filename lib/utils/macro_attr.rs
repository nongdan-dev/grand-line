#[derive(Default, Clone)]
pub struct MacroAttr {
    pub no_created_at: bool,
    pub no_updated_at: bool,
    pub no_deleted_at: bool,
    pub no_by_id: bool,
    /// model name in `#[crud]`
    pub model: String,
    /// to not use builtin generated inputs in `#[crud]`
    ///     use the inputs from the resolver instead
    pub resolver_inputs: bool,
    /// to not use builtin generated output in `#[crud]`
    ///     use the inputs from the resolver instead
    pub resolver_output: bool,
    /// to not generate db transaction `tx` in the resolver
    pub no_tx: bool,
    /// to not generate `count` in the search resolver
    pub no_count: bool,
}
