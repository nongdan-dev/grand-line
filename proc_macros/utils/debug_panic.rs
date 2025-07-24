pub trait DebugPanic {
    fn debug(&self) -> String;
    fn panic(&self, err: &str) -> ! {
        let debug = self.debug();
        panic!("{} {}", debug, err);
    }
}
