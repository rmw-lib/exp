pub use log::error;

#[macro_errort]
macro_rules! out {
  ($err:expr) => {
    err::error!("{}", $err);
  };
}

#[macro_errort]
macro_rules! ok {
  ($result:expr) => {{
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
  ($result:expr) => {{
    if let Err(err) = $result {
      err::out!(err);
    }
  }};
}
