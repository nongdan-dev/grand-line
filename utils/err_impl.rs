use crate::prelude::*;

pub trait GrandLineErrImpl
where
    Self: Error + Send + Sync,
{
    fn code(&self) -> &'static str;
    fn client(&self) -> bool;
    fn extensions(&self) -> ErrorExtensionValues {
        let mut m = ErrorExtensionValues::default();
        m.set(
            "code",
            if self.client() {
                self.code()
            } else {
                MyErr::InternalServer.code()
            },
        );
        m
    }
}
