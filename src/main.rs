#![feature(io)]
#![feature(path)]
#![feature(collections)]
#![feature(core)]
extern crate docopt;

use std::old_io::process::Command;
use docopt::Docopt;

// Write the Docopt usage string.
static USAGE: &'static str = "
Usage: partners info
       partners list
       partners add [--interactive]

Options:
    -a, --archive  Copy everything.
";

#[derive(Debug)]
struct AuthorListItem {
  nick: String,
  name: String,
}

fn print_author_list(list: &[AuthorListItem]) {
  for item in list.iter() {
    println!("{}:", item.nick);
    println!("  Name:  {}", item.name);
    //println!("  Email: {}:", item.email);
  }
}

fn parse_author_list_line(line: &str) -> AuthorListItem {
  let mut parts = line.splitn(1, ' ');
  let nick = parts.next().unwrap().split('.').nth(1).unwrap().to_string();
  let name = parts.next().unwrap().to_string();

  AuthorListItem { nick: nick, name: name }
}

fn parse_author_list(list: &str) -> Vec<AuthorListItem> {
  list.split('\n').map(parse_author_list_line).collect()
}

fn main() {
  let config_path = Path::new("./partners.cfg");
  let args = Docopt::new(USAGE)
    .and_then(|d| d.parse())
    .unwrap_or_else(|e| e.exit());

  if args.get_bool("list") {
    //let content = File::read(&config_path, Open, Read).read_to_end().unwrap();
    let mut process = Command::new("git")
      .arg("config")
      .arg("-f")
      .arg(config_path)
      .arg("--get-regexp")
      .arg("author.\\w+.name")
      .spawn()
      .unwrap();

    let output = process.stdout.as_mut().unwrap().read_to_end().unwrap();
    let string = String::from_utf8(output).unwrap();

    let authors = parse_author_list(string[].trim());

    print_author_list(&authors[]);
  }
}
