extern crate get_local_ip;

fn main() {
  println!("{:?}", get_local_ip::network::ip().unwrap());
}
