use crate::prelude::*;

pub trait Ts2ToFieldOrPanic {
    fn field_or_panic(self) -> Field;
}

impl Ts2ToFieldOrPanic for Ts2 {
    fn field_or_panic(self) -> Field {
        Parser::parse2(Field::parse_named, self).unwrap_or_else(|e| {
            panic!("token stream to field error: {e}");
        })
    }
}
