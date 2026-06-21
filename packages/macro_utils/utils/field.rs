use crate::prelude::*;

pub trait Ts2ToField {
    fn field_or_err(self) -> SynRes<Field>;
}

impl Ts2ToField for Ts2 {
    fn field_or_err(self) -> SynRes<Field> {
        Parser::parse2(Field::parse_named, self)
    }
}
