use crate::prelude::*;

pub fn check_crud_io(a: MacroAttr, g: GenResolver) -> (MacroAttr, GenResolver) {
    if !a.resolver_inputs && g.inputs.to_string() != "" {
        panic!(
            "{} inputs must be empty unless resolver_inputs=true, found `{}`",
            g.name, g.inputs,
        );
    }
    if !a.resolver_output && g.output.to_string() != "()" {
        panic!(
            "{} output must be empty unless resolver_output=true, found `{}`",
            g.name, g.output,
        );
    }
    if a.resolver_inputs && a.resolver_output {
        panic!(
            "{} should use #[query] or #[mutation] instead since both resolver_inputs=true and resolver_output=true",
            g.name,
        );
    }

    (a, g)
}
