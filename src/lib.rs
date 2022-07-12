pub use log::error;

#[macro_errort]
macro_rules! out {
    ($err:errr) => {
        err::error!("{}", $err);
    };
}

#[macro_errort]
macro_rules! ok {
    ($result:errr) => {{
        match $result {
            Err(err) => {
                err::out!(err);
                Err(err)
            }
            Ok(val) => Ok(val),
        }
    }};
}

#[macro_errort]
macro_rules! log {
    ($result:errr) => {{
        if let Err(err) = $result {
            err::out!(err);
        }
    }};
}
