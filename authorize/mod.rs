pub use crate::prelude::*;

static ALLOW_BUILT_IN: LazyLock<HashSet<&'static str>> = LazyLock::new(|| {
    let mut m = HashSet::new();
    m.insert("__typename");
    m
});

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PermNode {
    #[serde(default)]
    pub allow: bool,
    #[serde(default)]
    pub children: HashMap<String, PermNode>,
}
impl PermNode {
    pub fn children(&self, k: &str) -> bool {
        self.children.get(k).map(|p| p.allow).unwrap_or_default()
    }
    pub fn wildcard(&self) -> bool {
        self.children("*")
    }
    pub fn wildcard_nested(&self) -> bool {
        self.children("**")
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpAccess {
    pub op: String,
    #[serde(default)]
    pub args: PermNode,
    #[serde(default)]
    pub fields: PermNode,
}

fn check_args_tree(node: &PermNode, name: &str, val: &Value) -> bool {
    let child = node.children.get(name).or_else(|| node.children.get("*"));
    match val {
        Value::List(items) => match child {
            Some(rule) => {
                let elem_rule = rule.children.get("*").unwrap_or(rule);
                items.iter().all(|v| check_args_value(elem_rule, v))
            }
            None => false,
        },
        Value::Object(obj) => match child {
            Some(rule) => {
                for (k, v) in obj.iter() {
                    let key = k.as_str();
                    if !check_args_tree(rule, key, v) {
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

fn check_args_value(rule: &PermNode, v: &Value) -> bool {
    match v {
        Value::List(items) => {
            let elem = rule.children.get("*").unwrap_or(rule);
            items.iter().all(|x| check_args_value(elem, x))
        }
        Value::Object(obj) => {
            for (k, vv) in obj.iter() {
                let key = k.as_str();
                let child = rule.children.get(key).or_else(|| rule.children.get("*"));
                match child {
                    Some(c) => {
                        if !check_args_value(c, vv) {
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

pub fn check_arguments_against_perm(ctx: &Context<'_>, root: &PermNode) -> bool {
    let field = ctx.field();
    let Ok(pairs) = field.arguments() else {
        return false;
    };
    for (name, value) in pairs {
        if !check_args_tree(root, name.as_str(), &value) {
            return false;
        }
    }
    true
}

fn check_selection(field: SelectionField<'_>, rule: &PermNode) -> bool {
    for sub in field.selection_set() {
        let name = sub.name();
        let child = rule.children.get(name).or_else(|| rule.children.get("*"));
        match child {
            Some(r) => {
                let has_children = sub.selection_set().next().is_some();
                if has_children {
                    if !check_selection(sub, r) {
                        return false;
                    }
                } else {
                    if !r.allow && !r.children.contains_key("*") {
                        return false;
                    }
                }
            }
            None => {
                return false;
            }
        }
    }
    true
}

pub fn check_fields_against_perm(ctx: &Context<'_>, root: &PermNode) -> bool {
    let field = ctx.field();
    check_selection(field, root)
}
