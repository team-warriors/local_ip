extern crate regex;
extern crate serde_json;

use std::process::Command;
use regex::Regex;
use serde_json::{json};

pub fn to_string() -> Option<String> {
  let output = Command::new("ifconfig")
    .output()
    .expect("Failed to execute `ifconfig`");

  let stdout = String::from_utf8(output.stdout).unwrap();
  // this regex inspired by https://github.com/tcr/rust-local-ip/blob/master/src/lib.rs
  let rule_regex = r#"(?m)^.*inet (addr:)?(([0-9]*\.){3}[0-9]*).*$"#;
  let re = Regex::new(rule_regex).unwrap();

  let mut res = Vec::new();
  for caps in re.captures_iter(&stdout) {
    res.push(caps.get(2).unwrap().as_str());
  }

  let data_json = json!({
    "local": res[0],
    "network": res[1],
  });

  Some(data_json.to_string())
}
