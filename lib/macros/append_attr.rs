#[macro_export]
macro_rules! append_attr {
    ($pre:tt, $suf:tt) => {{
        let mut pre = format!("{}", $pre).trim().to_string();
        let mut suf = format!("{}", $suf).trim().to_string();
        if pre == "" && suf.contains("=") {
            pre = "_,".to_string();
        }
        if pre != "" && !pre.ends_with(",") {
            pre = format!("{},", pre);
        }
        if suf == "_" {
            suf = "".to_string();
        }
        if suf.starts_with("_,") {
            suf = suf.replacen("_,", "", 1);
        }
        format!("{}{}", pre, suf).parse().unwrap()
    }};
}
