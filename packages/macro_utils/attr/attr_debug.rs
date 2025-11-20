pub trait AttrDebug {
    fn attr_debug(&self) -> String;
    fn panic(&self, err: &str) -> ! {
        let err = [self.attr_debug(), err.to_owned()]
            .into_iter()
            .filter(|v| !v.is_empty())
            .collect::<Vec<_>>()
            .join(" ");
        panic!("{err}");
    }
}
