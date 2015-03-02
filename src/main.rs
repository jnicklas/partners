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
mod pair;
mod config;
mod concat;

use author::Author;
use pair::Pair;
use config::Config;
use std::borrow::Cow;

const CONFIG_PATH: &'static str = "./partners.cfg";
const CFG: git::Config = git::Config::File(CONFIG_PATH);

// Write the Docopt usage string.
static USAGE: &'static str = "
Usage: partners current
       partners list
       partners add --nick=<nick> --name=<name> [--email=<email>]
       partners add
       partners set <nick>...
       partners (--help | --version)

Options:
    -h, --help      Show help
    --version       Show version information
";

trait AuthorInformation {
    fn get_nick(&self) -> Cow<str>;
    fn get_name(&self) -> Cow<str>;
    fn get_email(&self) -> Cow<str>;
}

fn print_author_list(list: &[Author]) {
    for item in list.iter() {
        println!("{}:", item.nick);
        println!("  Name:  {}", item.name);
        println!("  Email: {}", item.get_email());
    }
}

fn parse_author_line(config: Rc<Config>, line: &str) -> Result<Author> {
    let mut parts = line.splitn(1, ' ');
    let nick = parts.next().unwrap().split('.').nth(1).unwrap().to_string();
    let name = parts.next().unwrap().to_string();
    let email = CFG.get(&format!("author.{}.email", nick)).ok();

    Ok(Author { config: config, nick: nick, name: name, email: email })
}

fn get_authors(config: Rc<Config>) -> Result<Vec<Author>> {
    let lines = try!(CFG.list("author.\\w+.name"));
    lines.iter().map(|line| parse_author_line(config.clone(), line)).collect()
}

fn write_author(author: &Author) -> Result<()> {
    try!(CFG.set(&format!("author.{}.name", author.nick), &author.name));
    if let Some(ref email) = author.email {
        try!(CFG.set(&format!("author.{}.email", author.nick), email));
    }
    Ok(())
}

fn print_current() {
    if let Ok(nick) = git::Config::None.get("partners.current") {
        println!("Nick:  {}", nick);
    }
    if let Ok(name) = git::Config::None.get("user.name") {
        println!("Name:  {}", name);
    }
    if let Ok(email) = git::Config::None.get("user.email") {
        println!("Email: {}", email);
    }
}

fn filter_authors<'a>(authors: &'a [Author], nicks: &[&str]) -> std::result::Result<Vec<&'a Author>, String> {
    nicks.iter().map(|n| authors.iter().find(|a| &a.nick == n).ok_or_else(|| n.to_string())).collect()
}

fn set_current<T>(current: T) -> Result<()> where T: AuthorInformation {
    try!(git::Config::Global.set("partners.current", &current.get_nick()));
    try!(git::Config::Global.set("user.name", &current.get_name()));
    try!(git::Config::Global.set("user.email", &current.get_email()));
    Ok(())
}

fn main() {
    let docopt = Docopt::new(USAGE).unwrap().help(true).version(Some(env!("CARGO_PKG_VERSION").to_string()));
    let args = docopt.parse().unwrap_or_else(|e| e.exit());

    let config = Rc::new(Config {
        domain: CFG.get("config.domain").unwrap_or_else(|_| "example.com".to_string()),
        prefix: CFG.get("config.prefix").unwrap_or_else(|_| "dev".to_string()),
        separator: CFG.get("config.separator").unwrap_or_else(|_| "+".to_string()),
    });

    if args.get_bool("list") {
        let authors = get_authors(config).unwrap();
        print_author_list(&authors);
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
        let nicks = args.get_vec("<nick>");
        let authors = get_authors(config.clone()).unwrap();
        match filter_authors(&authors, &nicks) {
            Ok(filtered_authors) => {
                println!("{:?}", filtered_authors);
                match filtered_authors.len() {
                    0 => println!("no author specified"),
                    1 => set_current(filtered_authors[0]).unwrap(),
                    _ => {
                        let pair = Pair { config: config.clone(), authors: filtered_authors.as_slice() };
                        set_current(&pair).unwrap()
                    }
                }
                print_current();
            },
            Err(nick) => {
                println!("couldn't find author '{}'", nick);
            }
        }

        // match get_authors(config).unwrap().iter().find(|a| a.nick == nick) {
        //     Some(author) => {
        //     }
        //     None => {
        //         println!("no such author");
        //     }
        // }
    }
}
