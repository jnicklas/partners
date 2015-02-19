#![feature(core)]
#![feature(process)]

extern crate docopt;

#[macro_use]
extern crate standard_error;

use docopt::Docopt;
use standard_error::StandardResult as Result;
use std::rc::Rc;

mod git;
mod author;
mod config;

use author::Author;
use config::Config;

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
  let email = CFG.get(&format!("author.{}.email", nick)[]).ok();

  Ok(Author { config: config.clone(), nick: nick, name: name, email: email })
}

fn get_authors(config: &Rc<Config>) -> Result<Vec<Author>> {
  let lines = try!(CFG.list("author.\\w+.name"));
  lines.iter().map(|line| parse_author_line(config, &line[])).collect()
}

fn write_author(author: &Author) -> Result<()> {
  try!(CFG.set(&*format!("author.{}.name", author.nick), &*author.name));
  if let Some(ref email) = author.email {
    try!(CFG.set(&*format!("author.{}.email", author.nick), &**email));
  }
  Ok(())
}

fn print_current() {
  println!("Nick:  {}", git::Config::None.get("partners.current").unwrap());
  println!("Name:  {}", git::Config::None.get("user.name").unwrap());
  println!("Email: {}", git::Config::None.get("user.email").unwrap());
}

fn main() {
  let args = Docopt::new(USAGE).and_then(|d| d.parse()).unwrap_or_else(|e| e.exit());

  let config = Rc::new(Config {
    domain: CFG.get("config.domain").unwrap_or_else(|_| "example.com".to_string()),
    prefix: CFG.get("config.prefix").unwrap_or_else(|_| "dev".to_string()),
    separator: CFG.get("config.separator").unwrap_or_else(|_| "+".to_string()),
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
        git::Config::Global.set("partners.current", &*author.nick).unwrap();
        git::Config::Global.set("user.name", &*author.name).unwrap();
        git::Config::Global.set("user.email", &*author.get_email()).unwrap();
        print_current();
      }
      None => {
        println!("no such author");
      }
    }
  }
}
