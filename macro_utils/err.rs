/// Internal macro utils to quickly construct a client error.
#[macro_export]
macro_rules! _err_client {
    ($($v:tt)*) => {
        GrandLineError::Client(ErrClient::$($v)*)
    };
}

/// Internal macro utils to quickly construct a client error result.
#[macro_export]
macro_rules! err_client {
    ($($v:tt)*) => {
        Err(_err_client!($($v)*))
    };
}

/// Internal macro utils to quickly construct a server error.
#[macro_export]
macro_rules! _err_server {
    ($($v:tt)*) => {
        GrandLineError::Server(ErrServer::$($v)*)
    };
}

/// Internal macro utils to quickly construct a server error result.
#[macro_export]
macro_rules! err_server {
    ($($v:tt)*) => {
        Err(_err_server!($($v)*))
    };
}
