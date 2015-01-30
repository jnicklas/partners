#![feature(io)]
#![feature(collections)]
#![feature(core)]
extern crate docopt;
#[macro_use]
extern crate standard_error;

use docopt::Docopt;
use standard_error::StandardResult as Result;
use std::rc::Rc;
use std::borrow::Cow;

mod git;

static CFG: git::Config = git::Config::File("./partners.cfg");

// Write the Docopt usage string.
static USAGE: &'static str = "
Usage: partners info
       partners list
       partners add --nick=<nick> --name=<name> [--email=<email>]
       partners add
       partners current
       partners set <nick>

Options:
    -a, --archive  Copy everything.
";

#[derive(Debug)]
struct Config {
  domain: String,
  prefix: String,
  separator: String,
}

#[derive(Debug)]
struct Author {
  config: Rc<Config>,
  nick: String,
  name: String,
  email: Option<String>,
}

impl Author {
  fn get_email(&self) -> std::string::CowString {
    match self.email {
      Some(ref email) => Cow::Borrowed(email.as_slice()),
      None => Cow::Owned(format!("{}@{}", self.nick, self.config.domain)),
    }
  }
}

fn print_author_list(list: &[Author]) {
  for item in list.iter() {
    println!("{}:", item.nick);
    println!("  Name:  {}", item.name);
    println!("  Email: {}", item.get_email());
  }
}

fn parse_author_line(config: &Rc<Config>, line: &str) -> Result<Author> {
  let mut parts = line.splitn(1, ' ');
  let nick = parts.next().unwrap().split('.').nth(1).unwrap().to_string();
  let name = parts.next().unwrap().to_string();
  let email = git::get(&CFG, &format!("author.{}.email", nick)[]).ok();

  Ok(Author { config: config.clone(), nick: nick, name: name, email: email })
}

fn get_authors(config: &Rc<Config>) -> Result<Vec<Author>> {
  let lines = try!(git::list(&CFG, "author.\\w+.name"));
  lines.iter().map(|line| parse_author_line(config, &line[])).collect()
}

fn write_author(author: &Author) -> Result<()> {
  try!(git::set(&CFG, &*format!("author.{}.name", author.nick), &*author.name));
  if let Some(ref email) = author.email {
    try!(git::set(&CFG, &*format!("author.{}.email", author.nick), &**email));
  }
  Ok(())
}

fn print_current() {
  println!("Name:  {}", git::get(&git::Config::None, "user.name").unwrap());
  println!("Email: {}", git::get(&git::Config::None, "user.email").unwrap());
}

fn main() {
  let args = Docopt::new(USAGE).and_then(|d| d.parse()).unwrap_or_else(|e| e.exit());

  let config = Rc::new(Config {
    domain: git::get(&CFG, "config.domain").unwrap_or_else(|_| "example.com".to_string()),
    prefix: git::get(&CFG, "config.prefix").unwrap_or_else(|_| "dev".to_string()),
    separator: git::get(&CFG, "config.separator").unwrap_or_else(|_| "+".to_string()),
  });

  if args.get_bool("list") {
    let authors = get_authors(&config).unwrap();
    print_author_list(&authors[]);
  } else if args.get_bool("add") {
    let email = args.get_str("--email");
    let nick = args.get_str("--nick");
    let name = args.get_str("--name");

    let email = if email.is_empty() { None } else { Some(email.to_string()) };

    let author = Author { config: config.clone(), nick: nick.to_string(), name: name.to_string(), email: email };
    write_author(&author).unwrap();
  } else if args.get_bool("current") {
    print_current();
  } else if args.get_bool("set") {
    let nick = args.get_str("<nick>");

    match get_authors(&config).unwrap().iter().find(|a| a.nick == nick) {
      Some(author) => {
        git::set(&git::Config::Global, "partners.current", &*author.nick).unwrap();
        git::set(&git::Config::Global, "user.name", &*author.name).unwrap();
        git::set(&git::Config::Global, "user.email", &*author.get_email()).unwrap();
        print_current();
      }
      None => {
        println!("no such author");
      }
    }
  }
}
