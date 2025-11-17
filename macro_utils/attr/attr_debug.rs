pub trait AttrDebug {
    fn attr_debug(&self) -> String;
    fn panic(&self, err: &str) -> ! {
        let err = [self.attr_debug(), err.to_owned()]
            .iter()
            .filter(|v| !v.is_empty())
            .cloned()
            .collect::<Vec<_>>()
            .join(" ");
        panic!("{err}");
    }
}
