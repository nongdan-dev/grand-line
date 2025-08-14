use crate::prelude::*;

pub trait ContextX
where
    Self: GrandLineContextImpl,
{
    /// Get look ahead key with alias-aware.
    /// For example the below two queries both have the same key a.b at b if there is no alias-aware.
    /// But the selection sets are different so they need to have different keys:
    /// query q1 {
    ///   a {
    ///     b {
    ///       c
    ///     }
    ///   }
    /// }
    /// query q2 {
    ///   x: a {
    ///     b {
    ///       c
    ///       d
    ///     }
    ///   }
    /// }
    fn gql_look_ahead_key(&self) -> String;
}

impl ContextX for Context<'_> {
    fn gql_look_ahead_key(&self) -> String {
        let mut arr = vec![];
        let mut next = self.path_node;
        // TODO: alias
        while let Some(n) = next {
            if let QueryPathSegment::Name(n) = n.segment {
                arr.push(n.to_string());
            }
            next = n.parent.copied();
        }
        arr.reverse();
        arr.join(".")
    }
}
