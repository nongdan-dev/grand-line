use crate::prelude::*;

pub fn check_err<E>(r: &Response, err: E)
where
    E: GrandLineErrImpl,
{
    assert!(r.errors.len() == 1, "response should have an error");

    let e = &r.errors[0];
    let expected_message = err.to_string();
    pretty_eq!(e.message, expected_message, "error message should match");

    if let Some(extensions) = e.extensions.as_ref() {
        let expected_code = err.code();
        if let Some(Value::String(code)) = extensions.get("code") {
            pretty_eq!(code, expected_code, "error extensions code should match");
        } else {
            assert!(false, "error extensions code should be some string");
        }
    } else {
        assert!(false, "error extensions should be some");
    }
}
