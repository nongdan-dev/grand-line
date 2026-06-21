use crate::prelude::*;

pub trait StringToTs2 {
    fn ts2_or_err(&self) -> SynRes<Ts2>;
}

impl StringToTs2 for String {
    fn ts2_or_err(&self) -> SynRes<Ts2> {
        self.parse::<Ts2>()
            .map_err(|e| SynErr::new(Span::call_site(), e.to_string()))
    }
}

impl StringToTs2 for str {
    fn ts2_or_err(&self) -> SynRes<Ts2> {
        self.to_owned().ts2_or_err()
    }
}
