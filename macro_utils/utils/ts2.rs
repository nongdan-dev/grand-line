use crate::prelude::*;

pub trait ToTs2 {
    fn ts2(&self) -> Ts2;
}

impl ToTs2 for String {
    fn ts2(&self) -> Ts2 {
        self.to_string().parse::<Ts2>().unwrap_or_else(|e| {
            pan!("ts2 parse error: {e}");
        })
    }
}

impl ToTs2 for str {
    fn ts2(&self) -> Ts2 {
        self.to_owned().ts2()
    }
}
