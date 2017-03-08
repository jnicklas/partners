#[macro_use] extern crate clap;
#[macro_use] extern crate derive_error;

use std::rc::Rc;
use std::process;
use std::io;
use std::io::Write;
use std::path::PathBuf;

#[macro_use]
mod git;
mod author;
mod pair;
mod config;
mod concat;

use clap::{App, ArgMatches};
use author::Author;
use pair::Pair;
use config::Config;
use std::borrow::Cow;

trait AuthorInformation {
    fn get_nick(&self) -> Cow<str>;
    fn get_name(&self) -> Cow<str>;
    fn get_email(&self) -> Cow<str>;
}

fn parse_author_line(config: Rc<Config>, line: &str) -> Result<Author, PartnersError> {
    let mut parts = line.splitn(2, ' ');
    let nick = parts.next().expect("does not contain nick").split('.').nth(1).unwrap().to_string();
    let name = parts.next().expect("does not contain name").to_string();
    let email = config.git.get(&format!("author.{}.email", nick)).ok();

    Ok(Author { config: config, nick: nick, name: name, email: email })
}

fn get_authors(config: Rc<Config>) -> Result<Vec<Author>, PartnersError> {
    let lines = try!(config.git.list("author.\\w+.name"));
    lines.iter().map(|line| parse_author_line(config.clone(), line)).collect()
}

fn write_author(author: &Author) -> Result<(), PartnersError> {
    try!(author.config.git.set(&format!("author.{}.name", author.nick), &author.name));
    if let Some(ref email) = author.email {
        try!(author.config.git.set(&format!("author.{}.email", author.nick), email));
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

fn set_current<T>(current: T) -> Result<(), PartnersError> where T: AuthorInformation {
    try!(git::Config::Global.set("partners.current", &current.get_nick()));
    try!(git::Config::Global.set("user.name", &current.get_name()));
    try!(git::Config::Global.set("user.email", &current.get_email()));
    Ok(())
}

#[derive(Debug, Error)] pub enum PartnersError {
    RandomError,
    NoSuchCommand,
    HomeDirectoryNotFound,
    NoAuthorSpecified,
    AuthorNotFound,
    #[error(msg_embedded, non_std, no_from)]
    GitError(String),
    IoError(io::Error),
    UTF8Error(::std::string::FromUtf8Error)
}

pub type Result<T, E=PartnersError> = ::std::result::Result<T, E>;

fn get_config_path() -> Result<PathBuf> {
    Ok(std::env::home_dir().ok_or(PartnersError::HomeDirectoryNotFound)?.join(".partners.cfg"))
}

fn get_config() -> Result<Config> {
    Ok(Config::from_git(git::Config::File(get_config_path()?.clone())))
}


fn list() -> Result<()> {
    let config = Rc::new(get_config()?);
    let authors = get_authors(config).unwrap();

    for item in authors.iter() {
        println!("{}:", item.nick);
        println!("  Name:  {}", item.name);
        println!("  Email: {}", item.get_email());
    }

    Ok(())
}

fn add(matches: &ArgMatches) -> Result<()> {
    let nick = matches.value_of("nick").unwrap();
    let name = matches.value_of("name").unwrap();
    let email = matches.value_of("email").unwrap();

    let config = Rc::new(get_config()?);

    let email = if email.is_empty() { None } else { Some(email.to_string()) };

    let author = Author { config: config.clone(), nick: nick.to_string(), name: name.to_string(), email: email };
    write_author(&author).unwrap();
    Ok(())
}

fn current() -> Result<()> {
    print_current();
    Ok(())
}

fn set(matches: &ArgMatches) -> Result<()> {
    let config = Rc::new(get_config()?);
    let nicks: Vec<&str> = matches.values_of("nicks").unwrap().collect();

    let authors = get_authors(config.clone()).unwrap();

    match filter_authors(&authors, &nicks) {
        Ok(filtered_authors) => {
            match filtered_authors.len() {
                0 => Err(PartnersError::NoAuthorSpecified)?,
                1 => {
                    set_current(filtered_authors[0]).unwrap();
                }
                _ => {
                    let pair = Pair { config: config.clone(), authors: &filtered_authors };
                    set_current(&pair).unwrap();
                }
            }
            print_current();
            Ok(())
        },
        Err(_nick) => {
            Err(PartnersError::AuthorNotFound)
        }
    }
}


fn run() -> Result<()> {
    let yaml = load_yaml!("cli.yml");
    let app = App::from_yaml(yaml).version(crate_version!()).author(crate_authors!());
    let matches = app.get_matches();

    match matches.subcommand() {
        ("list", Some(_sub_matches)) => list(),
        ("current", Some(_sub_matches)) => current(),
        ("set", Some(sub_matches)) => set(sub_matches),
        ("add", Some(sub_matches)) => add(sub_matches),
        _ => {
            println!("{}", matches.usage());
            Ok(())
        }
    }
}

fn main() {
    match run() {
        Ok(_) => {},
        Err(e) => {
            writeln!(&mut io::stderr(), "ERROR: {}", e).unwrap();
            process::exit(1);
        }
    }
}
