use crate::prelude::*;

/// RowPolicy is a map from field path (e.g. "users" or "users.posts") to a script.
/// Evaluated lazily inside resolvers that call `authz_row()`, not at the operation root.
pub type RowPolicy = HashMap<String, RowPolicyField>;

#[gql_input]
pub struct RowPolicyField {
    /// A script passed to the handler implementation to produce a filter object.
    /// The result is deserialized into the resolver's concrete Filter type and applied as a
    /// WHERE condition.  Interpretation of the script is up to the handler.
    pub script: String,
}
