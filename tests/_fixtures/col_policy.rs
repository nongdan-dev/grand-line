use grand_line::prelude::*;

pub const fn col_policy_field(children: ColPolicyFields) -> ColPolicyField {
    ColPolicyField {
        allow: true,
        children: Some(children),
    }
}
pub const fn col_policy_field_no_children() -> ColPolicyField {
    ColPolicyField {
        allow: true,
        children: None,
    }
}

pub fn col_policy_fields(k: String, children: ColPolicyFields) -> ColPolicyFields {
    hashmap! {
        k => col_policy_field(children),
    }
}
pub fn col_policy_fields_no_children(k: String) -> ColPolicyFields {
    hashmap! {
        k => col_policy_field_no_children(),
    }
}

pub fn col_policy_fields_wildcard() -> ColPolicyFields {
    col_policy_fields_no_children("*".to_owned())
}
pub fn col_policy_fields_wildcard_nested() -> ColPolicyFields {
    col_policy_fields_no_children("**".to_owned())
}

pub const fn col_policy_operation(inputs: ColPolicyField, output: ColPolicyField) -> ColPolicyOperation {
    ColPolicyOperation {
        inputs,
        output,
    }
}
pub fn col_policy(k: String, inputs: ColPolicyField, output: ColPolicyField) -> ColPolicy {
    hashmap! {
        k => col_policy_operation(inputs, output),
    }
}

pub fn col_policy_wildcard() -> ColPolicy {
    let children = col_policy_fields_wildcard_nested();
    let field = col_policy_field(children);
    col_policy("*".to_owned(), field.clone(), field)
}

pub fn col_policy_with_children(k: &str, child_k: &str) -> ColPolicy {
    let inputs = col_policy_field(col_policy_fields_wildcard_nested());
    let output = col_policy_field(col_policy_fields_no_children(child_k.to_owned()));
    col_policy(k.to_owned(), inputs, output)
}
