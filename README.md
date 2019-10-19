# local_ip

A simple get ip from your local computer. 😉

## How to use?

### Install

```
[dependencies]
get_local_ip = "0.1.0"
```

```
extern crate get_local_ip;

fn main() {
  get_local_ip::get::to_string(); // print "127.0.0.1", "192.168.43.102"
}
```
