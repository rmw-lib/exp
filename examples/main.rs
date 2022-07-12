use std::fs::File;

fn main() {
  if let Ok(_) = err::ok!(File::open("/not_exist")) {
    err::log!(Err::<(), _>("test"));
  }
}
