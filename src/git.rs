use standard_error::StandardResult as Result;
use std::old_io::process::Command;

pub enum ConfigType {
  File(&'static str),
  Global,
  None,
}

impl ConfigType {
  fn command(&self) -> Command {
    let mut command = Command::new("git");
    let mut command = command.arg("config");
    match *self {
      ConfigType::File(path) => command.arg("-f").arg(path).clone(),
      ConfigType::Global => command.arg("--global").clone(),
      ConfigType::None => command.clone()
    }
  }
}

pub fn get(config: &ConfigType, key: &str) -> Result<String> {
  let mut process = try!(config.command().arg(key).spawn());

  let result = try!(process.wait());

  if result.success() {
    let output = try!(process.stdout.as_mut().unwrap().read_to_end());
    let string = try!(String::from_utf8(output));
    Ok(string.trim().to_string())
  } else {
    fail!(format!("config not found! {}", key));
  }
}

pub fn set(config: &ConfigType, key: &str, value: &str) -> Result<()> {
  let mut process = try!(config.command().arg(key).arg(value).spawn());

  let result = try!(process.wait());

  if result.success() {
    Ok(())
  } else {
    fail!(try!(String::from_utf8(try!(process.stderr.as_mut().unwrap().read_to_end()))));
  }
}

pub fn list(config: &ConfigType, keyexp: &str) -> Result<Vec<String>> {
  let mut process = try!(config.command().arg("--get-regexp").arg(keyexp).spawn());

  let result = try!(process.wait());

  if !result.success() {
    fail!("unable to list config");
  }

  let output = try!(process.stdout.as_mut().unwrap().read_to_end());
  let string = String::from_utf8(output).unwrap();

  Ok(string.trim().split('\n').map(ToString::to_string).collect())
}
