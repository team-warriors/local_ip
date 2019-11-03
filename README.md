# local_ip

✨ Simple library was made with rust, to get your local ip.

## Getting Started.

+ install

```
[dependencies]
get_local_ip = "0.1.3"
```

+ How to use?
```rust
extern crate get_local_ip;

fn main() {
  println!("{:?}", get_local_ip::network::ip().unwrap()); // "{\"local\":\"127.0.0.1\",\"network\":\"192.168.43.102\"}"
}
```
