use crate::prelude::*;

pub trait AttrDebug {
    fn attr_debug(&self) -> String;
    fn err(&self, err: &str) -> String {
        [self.attr_debug(), s!(err)]
            .iter()
            .cloned()
            .filter(|v| v != "")
            .collect::<Vec<_>>()
            .join(" ")
    }
}
