use crate::prelude::*;

pub trait AttrDebug {
    fn attr_debug(&self) -> String;
    fn err(&self, err: &str) -> String {
        [self.attr_debug(), s!(err)]
            .iter()
            .filter(|v| !v.is_empty())
            .cloned()
            .collect::<Vec<_>>()
            .join(" ")
    }
}
