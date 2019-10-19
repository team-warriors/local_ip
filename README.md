# local_ip

Simple library create with rust to get your IP in local computer.

## Todo

- [x] export ip to string.
- [ ] export ip to JSON.

## How to use?

### Install

```
[dependencies]
get_local_ip = "0.1.0"
```

```rust
extern crate get_local_ip;

fn main() {
  get_local_ip::get::to_string(); // print "127.0.0.1", "192.168.43.102"
}
```
