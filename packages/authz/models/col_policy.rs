use crate::prelude::*;

/// ColPolicy is a map from operation name (or "*" wildcard) to its per-operation policy.
/// Checked once at the operation root resolver before any DB work.
pub type ColPolicy = HashMap<String, ColPolicyOperation>;

#[gql_input]
pub struct ColPolicyOperation {
    /// Controls which GraphQL arguments callers may pass.
    pub inputs: ColPolicyField,
    /// Controls which GraphQL fields callers may select in the response.
    pub output: ColPolicyField,
}

/// A node in the allow-tree.  Both inputs and output share this shape.
///
/// Wildcard semantics (stored as special keys in `children`):
///   "*"  -- allow all direct scalar children without explicit entries.
///   "**" -- allow all descendants at any depth (short-circuits the whole subtree).
#[gql_input]
pub struct ColPolicyField {
    pub allow: bool,
    pub children: Option<ColPolicyFields>,
}
pub type ColPolicyFields = HashMap<String, ColPolicyField>;

impl ColPolicyField {
    /// True when children["*"].allow -- direct scalars are allowed without explicit entries.
    pub(crate) fn wildcard(&self) -> bool {
        self.children
            .as_ref()
            .map(|m| m.get("*"))
            .unwrap_or(None)
            .is_some_and(|p| p.allow)
    }

    /// True when children["**"].allow -- entire subtree is allowed, skip further checks.
    pub(crate) fn wildcard_nested(&self) -> bool {
        self.children
            .as_ref()
            .map(|m| m.get("**"))
            .unwrap_or(None)
            .is_some_and(|p| p.allow)
    }
}

/// Entry point: verify every argument in the current GraphQL field against `inputs`.
pub fn col_policy_check_inputs(ctx: &Context<'_>, inputs: &ColPolicyField) -> bool {
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

/// Check a single key-value argument pair against `parent`.
/// Dispatches on value shape:
///   List   -- each element is checked against the child policy (check_inputs_value).
///   Object -- each field of the object recurses back into check_inputs_tree.
///   scalar -- the child key must exist and be allowed, or parent wildcard(*) applies.
fn check_inputs_tree(parent: &ColPolicyField, child_k: &str, child_v: &GraphQLValue) -> bool {
    if parent.wildcard_nested() {
        return true;
    }
    let child = parent.children.as_ref().and_then(|m| m.get(child_k));

    match child_v {
        GraphQLValue::List(list) => {
            let Some(child) = child else {
                return false;
            };
            list.iter().all(|v| check_inputs_value(child, v))
        }
        GraphQLValue::Object(object) => {
            let Some(child) = child else {
                return false;
            };
            object.iter().all(|(k, v)| check_inputs_tree(child, k.as_str(), v))
        }
        _ => parent.wildcard() || child.is_some_and(|p| p.allow),
    }
}

/// Check a value that is already known to belong to an allowed child.
/// Handles the case where a list element is itself an object or another list.
fn check_inputs_value(child: &ColPolicyField, child_v: &GraphQLValue) -> bool {
    if child.wildcard_nested() {
        return true;
    }
    match child_v {
        GraphQLValue::List(list) => list.iter().all(|v| check_inputs_value(child, v)),
        GraphQLValue::Object(object) => object.iter().all(|(k, v)| {
            let grand = child.children.as_ref().and_then(|m| m.get(k.as_str()));
            let Some(grand) = grand else {
                return false;
            };
            check_inputs_value(grand, v)
        }),
        _ => child.allow,
    }
}

/// Entry point: verify every selected field in the GraphQL response against `output`.
pub fn col_policy_check_output(ctx: &Context<'_>, output: &ColPolicyField) -> bool {
    check_output(ctx.field(), output)
}

/// Recursively walk the selection set.
/// Nested selections (objects/relations) must have an explicit child entry.
/// Leaf fields (scalars) may be covered by the parent wildcard(*).
fn check_output(field: SelectionField<'_>, parent: &ColPolicyField) -> bool {
    if parent.wildcard_nested() {
        return true;
    }
    for sub in field.selection_set() {
        let child_k = sub.name();
        let child = parent.children.as_ref().and_then(|m| m.get(child_k));
        if sub.selection_set().next().is_some() {
            // Nested object/relation: must have an explicit entry and pass recursively.
            let Some(child) = child else {
                return false;
            };
            if !check_output(sub, child) {
                return false;
            }
        } else {
            // Leaf scalar: allowed if explicitly listed, covered by wildcard, or a built-in.
            let allow = parent.wildcard() || ALLOW_BUILT_IN.contains(child_k) || child.is_some_and(|p| p.allow);
            if !allow {
                return false;
            }
        }
    }
    // Empty selection set means a scalar resolved by a parent recursive call -- allow.
    true
}

static ALLOW_BUILT_IN: LazyLock<HashSet<&'static str>> = LazyLock::new(|| {
    let mut m = HashSet::new();
    m.insert("__typename");
    m
});
