use standard_error::StandardResult as Result;
use std::old_io::process::Command;

use super::CONFIG_PATH;

pub fn get(key: &str) -> Result<String> {
  let mut process = try!(Command::new("git")
    .arg("config")
    .arg("-f")
    .arg(&Path::new(CONFIG_PATH))
    .arg(key)
    .spawn());

  let result = try!(process.wait());

  if result.success() {
    let output = try!(process.stdout.as_mut().unwrap().read_to_end());
    let string = try!(String::from_utf8(output));
    Ok(string.trim().to_string())
  } else {
    fail!(format!("config not found! {}", key));
  }
}

pub fn set(key: &str, value: &str) -> Result<()> {
  let mut process = try!(Command::new("git")
    .arg("config")
    .arg("-f")
    .arg(&Path::new(CONFIG_PATH))
    .arg(key)
    .arg(value)
    .spawn());

  let result = try!(process.wait());

  if result.success() {
    Ok(())
  } else {
    fail!(format!("cannot set config! {} to {}", key, value));
  }
}

pub fn list(keyexp: &str) -> Result<Vec<String>> {
  let mut process = try!(Command::new("git")
    .arg("config")
    .arg("-f")
    .arg(&Path::new(CONFIG_PATH))
    .arg("--get-regexp")
    .arg(keyexp)
    .spawn());

  let result = try!(process.wait());

  if !result.success() {
    fail!("unable to list config");
  }

  let output = try!(process.stdout.as_mut().unwrap().read_to_end());
  let string = String::from_utf8(output).unwrap();

  Ok(string.trim().split('\n').map(ToString::to_string).collect())
}
