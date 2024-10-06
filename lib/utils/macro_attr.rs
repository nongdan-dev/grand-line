#[derive(Default, Clone)]
pub struct MacroAttr {
    /// model name in #[crud]
    pub model: String,
    /// json reflection in #[create/update] to generate builtin inputs based on the struct definition
    ///     append in #[macro_model] since it contains the struct definition
    pub meta: String,
    /// to not insert builtin fields in #[model]
    ///     append in #[macro_model] since it is already done
    pub no_builtin: bool,
    /// to not generate macros in #[model]
    ///     append in #[macro_model] since it is already done
    pub no_macro: bool,
    /// to not use builtin generated inputs in #[crud]
    ///     use the inputs from the resolver instead
    pub resolver_inputs: bool,
    /// to not use builtin generated output in #[crud]
    ///     use the inputs from the resolver instead
    pub resolver_output: bool,
    /// to not generate db transaction `tx` in the resolver
    pub no_tx: bool,
    /// to not generate `count` in the search resolver
    pub no_count: bool,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct MacroMeta {
    pub fields: Vec<String>,
    pub types: Vec<String>,
}
