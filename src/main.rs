extern crate get_local_ip;

use get_local_ip::get;

fn main() {
  // Example to use
  println!("{:?}", get::to_string().unwrap());
}
