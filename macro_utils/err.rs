/// Internal macro utils to quickly construct a client error.
#[macro_export]
macro_rules! err_client {
    ($($v:tt)*) => {
        GrandLineError::Client(GrandLineErrorClient::$($v)*)
    };
}

/// Internal macro utils to quickly construct a client error result.
#[macro_export]
macro_rules! err_client_res {
    ($($v:tt)*) => {
        Err(err_client!($($v)*))
    };
}

/// Internal macro utils to quickly construct a server error.
#[macro_export]
macro_rules! err_server {
    ($($v:tt)*) => {
        GrandLineError::Server(GrandLineErrorServer::$($v)*)
    };
}

/// Internal macro utils to quickly construct a server error result.
#[macro_export]
macro_rules! err_server_res {
    ($($v:tt)*) => {
        Err(err_server!($($v)*))
    };
}
