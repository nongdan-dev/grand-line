use crate::prelude::*;

pub trait AttrDebug {
    fn attr_debug(&self) -> String;
    fn span(&self) -> Span {
        Span::call_site()
    }
    fn syn_err(&self, err: &str) -> SynErr {
        let msg = [self.attr_debug(), err.to_owned()]
            .into_iter()
            .filter(|v| !v.is_empty())
            .collect::<Vec<_>>()
            .join(" ");
        SynErr::new(self.span(), msg)
    }
}
