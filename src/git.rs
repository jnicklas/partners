use standard_error::StandardResult as Result;
use std::old_io::process::Command;

pub enum Config {
  File(&'static str),
  Global,
  None,
}

impl Config {
  fn command(&self) -> Command {
    let mut command = Command::new("git");
    let mut command = command.arg("config");
    match *self {
      Config::File(path) => command.arg("-f").arg(path).clone(),
      Config::Global => command.arg("--global").clone(),
      Config::None => command.clone()
    }
  }

  pub fn get(&self, key: &str) -> Result<String> {
    let mut process = try!(self.command().arg(key).spawn());

    let result = try!(process.wait());

    if result.success() {
      let output = try!(process.stdout.as_mut().unwrap().read_to_end());
      let string = try!(String::from_utf8(output));
      Ok(string.trim().to_string())
    } else {
      fail!(try!(String::from_utf8(try!(process.stderr.as_mut().unwrap().read_to_end()))));
    }
  }

  pub fn set(&self, key: &str, value: &str) -> Result<()> {
    let mut process = try!(self.command().arg(key).arg(value).spawn());

    let result = try!(process.wait());

    if result.success() {
      Ok(())
    } else {
      fail!(try!(String::from_utf8(try!(process.stderr.as_mut().unwrap().read_to_end()))));
    }
  }

  pub fn list(&self, keyexp: &str) -> Result<Vec<String>> {
    let mut process = try!(self.command().arg("--get-regexp").arg(keyexp).spawn());

    let result = try!(process.wait());

    if !result.success() {
      fail!(try!(String::from_utf8(try!(process.stderr.as_mut().unwrap().read_to_end()))));
    }

    let output = try!(process.stdout.as_mut().unwrap().read_to_end());
    let string = String::from_utf8(output).unwrap();

    Ok(string.trim().split('\n').map(ToString::to_string).collect())
  }
}
