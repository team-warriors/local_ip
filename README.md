# local_ip

✨ Simple library was made with rust, to get your local ip.

## Getting Started.

+ install

```
[dependencies]
get_local_ip = "0.1.1"
```

+ How to use?
```rust
extern crate get_local_ip;

use get_local_ip::get;

fn main() {
  // Example to use
  println!("{:?}", get::to_string().unwrap());
  // output
  // "{\"local\":\"127.0.0.1\",\"network\":\"192.168.43.102\"}"
}
```
