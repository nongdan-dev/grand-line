use crate::prelude::*;

pub trait DebugPrefix {
    fn debug(&self) -> String;
    fn msg(&self, err: &str) -> String {
        [self.debug(), str!(err)]
            .iter()
            .cloned()
            .filter(|v| v != "")
            .collect::<Vec<_>>()
            .join(" ")
    }
}
