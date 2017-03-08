use std::process::{Command};
use std::path::PathBuf;
use PartnersError;

#[derive(Debug)]
pub enum Config {
    File(PathBuf),
    Global,
    None,
}

fn read_result(command: &mut Command) -> Result<String, PartnersError> {
    let result = try!(command.output());

    if result.status.success() {
        let string = try!(String::from_utf8(result.stdout));
        Ok(string.trim().to_string())
    } else {
        let string = try!(String::from_utf8(result.stderr));
        Err(PartnersError::GitError(string))
    }
}

impl Config {
    fn command(&self) -> Command {
        let mut command = Command::new("git");
        match *self {
            Config::File(ref path) => command.arg("config").arg("-f").arg(path),
            Config::Global => command.arg("config").arg("--global"),
            Config::None => command.arg("config")
        };
        command
    }

    pub fn get(&self, key: &str) -> Result<String, PartnersError> {
        let mut command = self.command();

        read_result(command.arg(key))
    }

    pub fn set(&self, key: &str, value: &str) -> Result<(), PartnersError> {
        let mut command = self.command();

        try!(read_result(command.arg(key).arg(value)));

        Ok(())
    }

    pub fn list(&self, keyexp: &str) -> Result<Vec<String>, PartnersError> {
        let mut command = self.command();

        let string = try!(read_result(command.arg("--get-regexp").arg(keyexp)));

        Ok(string.split('\n').map(ToString::to_string).collect())
    }
}
