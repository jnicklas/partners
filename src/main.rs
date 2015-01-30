#![feature(io)]
#![feature(path)]
#![feature(collections)]
#![feature(core)]
extern crate docopt;
#[macro_use]
extern crate standard_error;

use std::old_io::process::Command;
use docopt::Docopt;
use standard_error::StandardResult as Result;

static CONFIG_PATH: &'static str = "./partners.cfg";

// Write the Docopt usage string.
static USAGE: &'static str = "
Usage: partners info
       partners list
       partners add [--interactive] [--name=<name>] [--nick=<nick>]

Options:
    -a, --archive  Copy everything.
";

fn get_config(key: &str) -> Result<String> {
  let mut process = Command::new("git")
    .arg("config")
    .arg("-f")
    .arg(&Path::new(CONFIG_PATH))
    .arg(key)
    .spawn()
    .unwrap();

  let result = try!(process.wait());

  if result.success() {
    let output = try!(process.stdout.as_mut().unwrap().read_to_end());
    let string = try!(String::from_utf8(output));
    Ok(string.trim().to_string())
  } else {
    fail!(format!("config not found! {}", key));
  }
}

#[derive(Debug)]
struct Author {
  nick: String,
  name: String,
  email: String,
}

fn print_author_list(list: &[Author]) {
  for item in list.iter() {
    println!("{}:", item.nick);
    println!("  Name:  {}", item.name);
    println!("  Email: {}", item.email);
  }
}

fn parse_author_list_line(line: &str) -> Result<Author> {
  let mut parts = line.splitn(1, ' ');
  let nick = parts.next().unwrap().split('.').nth(1).unwrap().to_string();
  let name = parts.next().unwrap().to_string();
  let email = match get_config(&format!("author.{}.email", nick)[]) {
    Ok(email) => email,
    Err(_) => format!("{}@{}", nick, try!(get_config("config.domain")))
  };

  Ok(Author { nick: nick, name: name, email: email })
}

fn parse_author_list(list: &str) -> Result<Vec<Author>> {
  list.split('\n').map(parse_author_list_line).collect()
}

fn get_authors() -> Result<Vec<Author>> {
  let mut process = try!(Command::new("git")
    .arg("config")
    .arg("-f")
    .arg(&Path::new(CONFIG_PATH))
    .arg("--get-regexp")
    .arg("author.\\w+.name")
    .spawn());

  let result = try!(process.wait());

  if !result.success() {
    fail!("unable to read author list");
  }

  let output = try!(process.stdout.as_mut().unwrap().read_to_end());
  let string = String::from_utf8(output).unwrap();

  let authors = try!(parse_author_list(string[].trim()));
  Ok(authors)
}

fn main() {
  let args = Docopt::new(USAGE).and_then(|d| d.parse()).unwrap_or_else(|e| e.exit());

  if args.get_bool("list") {
    let authors = get_authors().unwrap();
    print_author_list(&authors[]);
  } else if args.get_bool("add") {

  }
}
