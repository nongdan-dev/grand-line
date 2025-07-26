#[macro_export]
macro_rules! err_client {
    ($($v:tt)*) => {
        Err(Into::<GrandLineError>::into(ErrClient::$($v)*))
    };
}

#[macro_export]
macro_rules! err_server {
    ($($v:tt)*) => {
        Err(Into::<GrandLineError>::into(ErrServer::$($v)*))
    };
}
