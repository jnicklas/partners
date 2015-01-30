#![feature(io)]
#![feature(path)]
#![feature(collections)]
#![feature(core)]
extern crate docopt;
#[macro_use]
extern crate standard_error;

use docopt::Docopt;
use standard_error::StandardResult as Result;

mod config;

static CONFIG_PATH: &'static str = "./partners.cfg";

// Write the Docopt usage string.
static USAGE: &'static str = "
Usage: partners info
       partners list
       partners add --name=<name> --nick=<nick> --email=<email>
       partners add

Options:
    -a, --archive  Copy everything.
";

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

fn parse_author_line(line: &str) -> Result<Author> {
  let mut parts = line.splitn(1, ' ');
  let nick = parts.next().unwrap().split('.').nth(1).unwrap().to_string();
  let name = parts.next().unwrap().to_string();
  let email = match config::get(&format!("author.{}.email", nick)[]) {
    Ok(email) => email,
    Err(_) => format!("{}@{}", nick, try!(config::get("config.domain")))
  };

  Ok(Author { nick: nick, name: name, email: email })
}

fn get_authors() -> Result<Vec<Author>> {
  let lines = try!(config::list("author.\\w+.name"));
  lines.iter().map(|line| parse_author_line(&line[])).collect()
}

fn write_author(author: &Author) -> Result<()> {
  try!(config::set(&*format!("author.{}.name", author.nick), &*author.name));
  try!(config::set(&*format!("author.{}.email", author.nick), &*author.email));
  Ok(())
}

fn main() {
  let args = Docopt::new(USAGE).and_then(|d| d.parse()).unwrap_or_else(|e| e.exit());

  if args.get_bool("list") {
    let authors = get_authors().unwrap();
    print_author_list(&authors[]);
  } else if args.get_bool("add") {
    let author = Author {
      nick: args.get_str("--nick").to_string(),
      name: args.get_str("--name").to_string(),
      email: args.get_str("--email").to_string(),
    };
    write_author(&author).unwrap();
  }
}
