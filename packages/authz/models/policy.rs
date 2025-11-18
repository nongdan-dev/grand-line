use crate::prelude::*;

#[model]
pub struct Policy {
    pub operation: String,
    pub inputs: JsonValue,
    pub output: JsonValue,
    pub role_id: String,
}

#[gql_input]
pub struct PolicyData {
    pub allow: bool,
    pub children: HashMap<String, PolicyData>,
}

impl PolicyData {
    pub fn children(&self, k: &str) -> bool {
        ALLOW_BUILT_IN.contains(k) || self.children.get(k).map(|p| p.allow).unwrap_or_default()
    }
    pub fn wildcard(&self) -> bool {
        self.children("*")
    }
    pub fn wildcard_nested(&self) -> bool {
        self.children("**")
    }
}

static ALLOW_BUILT_IN: LazyLock<HashSet<&'static str>> = LazyLock::new(|| {
    let mut m = HashSet::new();
    m.insert("__typename");
    m
});

pub(crate) fn policy_check_inputs(ctx: &Context<'_>, root: &PolicyData) -> bool {
    let field = ctx.field();
    let Ok(pairs) = field.arguments() else {
        return false;
    };
    for (name, value) in pairs {
        if !check_inputs_tree(root, name.as_str(), &value) {
            return false;
        }
    }
    true
}

fn check_inputs_tree(node: &PolicyData, name: &str, val: &Value) -> bool {
    let child = node.children.get(name);
    match val {
        Value::List(items) => match child {
            Some(rule) => {
                let elem_rule = rule.children.get("*").unwrap_or(rule);
                items.iter().all(|v| check_inputs_value(elem_rule, v))
            }
            None => false,
        },
        Value::Object(obj) => match child {
            Some(rule) => {
                for (k, v) in obj.iter() {
                    let key = k.as_str();
                    if !check_inputs_tree(rule, key, v) {
                        return false;
                    }
                }
                true
            }
            None => false,
        },
        _ => match child {
            Some(rule) => rule.allow || !rule.children.is_empty(),
            None => false,
        },
    }
}

fn check_inputs_value(rule: &PolicyData, v: &Value) -> bool {
    match v {
        Value::List(items) => {
            let elem = rule.children.get("*").unwrap_or(rule);
            items.iter().all(|x| check_inputs_value(elem, x))
        }
        Value::Object(obj) => {
            for (k, vv) in obj.iter() {
                let key = k.as_str();
                let child = rule.children.get(key).or_else(|| rule.children.get("*"));
                match child {
                    Some(c) => {
                        if !check_inputs_value(c, vv) {
                            return false;
                        }
                    }
                    None => {
                        return false;
                    }
                }
            }
            true
        }
        _ => rule.allow,
    }
}

pub(crate) fn policy_check_output(ctx: &Context<'_>, output: &PolicyData) -> bool {
    let field = ctx.field();
    check_output(field, output)
}

fn check_output(field: SelectionField<'_>, node: &PolicyData) -> bool {
    if node.wildcard_nested() {
        return true;
    }
    for sub in field.selection_set() {
        let name = sub.name();
        let has_children = sub.selection_set().next().is_some();
        if has_children {
            let child = node.children.get(name);
            match child {
                Some(r) => {
                    let allow = check_output(sub, r);
                    if !allow {
                        return false;
                    }
                }
                None => {
                    return false;
                }
            }
        } else {
            let allow = node.wildcard() || node.children(name);
            if !allow {
                return false;
            }
        }
    }
    true
}
