use crate::prelude::*;

#[gql_input]
pub struct OperationPolicy {
    pub inputs: OperationFieldPolicy,
    pub output: OperationFieldPolicy,
}

#[gql_input]
pub struct OperationFieldPolicy {
    pub allow: bool,
    pub children: HashMap<String, OperationFieldPolicy>,
}

impl OperationFieldPolicy {
    pub fn wildcard(&self) -> bool {
        self.children.get("*").map(|p| p.allow).unwrap_or_default()
    }
    pub fn wildcard_nested(&self) -> bool {
        self.children.get("**").map(|p| p.allow).unwrap_or_default()
    }
}

pub(crate) fn policy_check_inputs(ctx: &Context<'_>, inputs: &OperationFieldPolicy) -> bool {
    let Ok(pairs) = ctx.field().arguments() else {
        return false;
    };

    for (k, v) in pairs {
        if !check_inputs_tree(inputs, k.as_str(), &v) {
            return false;
        }
    }

    true
}

fn check_inputs_tree(parent: &OperationFieldPolicy, child_k: &str, child_v: &Value) -> bool {
    if parent.wildcard_nested() {
        return true;
    }
    let child = parent.children.get(child_k);

    match child_v {
        Value::List(list) => {
            let Some(child) = child else {
                return false;
            };
            for child_v in list {
                if !check_inputs_value(child, child_v) {
                    return false;
                }
            }
        }
        Value::Object(object) => {
            let Some(child) = child else {
                return false;
            };
            for (grand_k, grand_v) in object.iter() {
                if !check_inputs_tree(child, grand_k.as_str(), grand_v) {
                    return false;
                }
            }
        }
        _ => {
            let allow = parent.wildcard() || child.map(|p| p.allow).unwrap_or_default();
            if !allow {
                return false;
            }
        }
    }

    true
}

fn check_inputs_value(child: &OperationFieldPolicy, child_v: &Value) -> bool {
    if child.wildcard_nested() {
        return true;
    }

    match child_v {
        Value::List(list) => {
            for child_v in list {
                if !check_inputs_value(child, child_v) {
                    return false;
                }
            }
        }
        Value::Object(object) => {
            for (grand_k, grand_v) in object.iter() {
                let Some(grand) = child.children.get(grand_k.as_str()) else {
                    return false;
                };
                if !check_inputs_value(grand, grand_v) {
                    return false;
                }
            }
        }
        _ => {
            if !child.allow {
                return false;
            }
        }
    }

    true
}

pub(crate) fn policy_check_output(ctx: &Context<'_>, policy: &OperationFieldPolicy) -> bool {
    let field = ctx.field();
    check_output(field, policy)
}

fn check_output(field: SelectionField<'_>, policy: &OperationFieldPolicy) -> bool {
    if policy.wildcard_nested() {
        return true;
    }

    for sub in field.selection_set() {
        let name = sub.name();
        let child = policy.children.get(name);

        let has_children = sub.selection_set().next().is_some();
        if has_children {
            match child {
                Some(child) => {
                    let allow = check_output(sub, child);
                    if !allow {
                        return false;
                    }
                }
                None => {
                    return false;
                }
            }
        } else {
            let allow = policy.wildcard()
                || ALLOW_BUILT_IN.contains(name)
                || child.map(|p| p.allow).unwrap_or_default();
            if !allow {
                return false;
            }
        }
    }

    // if the selection set is empty which mean primitive type such as String Int
    // it should be allowed here since we already checked in the previous rescursive
    true
}

static ALLOW_BUILT_IN: LazyLock<HashSet<&'static str>> = LazyLock::new(|| {
    let mut m = HashSet::new();
    m.insert("__typename");
    m
});
