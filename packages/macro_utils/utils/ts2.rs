use crate::prelude::*;

pub trait StringToTs2OrPanic {
    fn ts2_or_panic(&self) -> Ts2;
}

impl StringToTs2OrPanic for String {
    fn ts2_or_panic(&self) -> Ts2 {
        self.parse::<Ts2>()
            .unwrap_or_else(|e| panic!("string to ts2 error: {e}"))
    }
}

impl StringToTs2OrPanic for str {
    fn ts2_or_panic(&self) -> Ts2 {
        self.to_owned().ts2_or_panic()
    }
}
