# rmw_err

utils for log err

`cargo add  rmw_err --rename err` then

```rust
use std::fs::File;

fn main() {
  if let Ok(_) = err::ok!(File::open("/not_exist")) {
    err::log!(Err::<(), _>("test"));
  }
}
```
