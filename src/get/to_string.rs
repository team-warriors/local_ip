extern crate regex;

use std::process::Command;
use regex::Regex;

pub fn to_string() {
  let output = Command::new("ifconfig")
    .output()
    .expect("Failed to execute `ifconfig`");
  let stdout = String::from_utf8(output.stdout).unwrap();
  let rule_regex = r#"(?m)^.*inet (addr:)?(([0-9]*\.){3}[0-9]*).*$"#;
  let re = Regex::new(rule_regex).unwrap();

  for caps in re.captures_iter(&stdout) {
    println!("{:?}", caps.get(2).unwrap().as_str());
  }
}
