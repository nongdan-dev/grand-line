/// Internal macro utils to quickly construct client error result.
#[macro_export]
macro_rules! err_client {
    ($($v:tt)*) => {
        Err(Into::<GrandLineError>::into(ErrClient::$($v)*))
    };
}

/// Internal macro utils to quickly construct server error result.
#[macro_export]
macro_rules! err_server {
    ($($v:tt)*) => {
        Err(Into::<GrandLineError>::into(ErrServer::$($v)*))
    };
}
