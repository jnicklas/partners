#[macro_use] extern crate clap;
#[macro_use] extern crate derive_error;

use std::process;
use std::io;
use std::io::Write;

#[macro_use]
mod git;
mod author;
mod concat;
mod author_selection;

use git::Config;
use clap::{App, ArgMatches};
use author::Author;
use author_selection::AuthorSelection;

pub type Result<T, E=PartnersError> = ::std::result::Result<T, E>;

#[derive(Debug, Error)] pub enum PartnersError {
    RandomError,
    NoSuchCommand,
    HomeDirectoryNotFound,
    NoAuthorSpecified,
    NoGitNick,
    NoGitName,
    NotManagedByPartners,
    #[error(msg_embedded, non_std, no_from)]
    AuthorNotFound(String),
    #[error(msg_embedded, non_std, no_from)]
    GitError(String),
    IoError(io::Error),
    UTF8Error(::std::string::FromUtf8Error)
}

fn list<'a>(partners_config: &'a Config) -> Result<()> {
    let authors = partners_config.authors()?;

    for author in authors.iter() {
        println!("{}", author);
    }

    Ok(())
}

fn add<'a>(partners_config: &'a Config, matches: &ArgMatches) -> Result<()> {
    let nick = matches.value_of("nick").unwrap();
    let name = matches.value_of("name").unwrap();
    let email = matches.value_of("email").unwrap();

    let email = if email.is_empty() { None } else { Some(email.to_string()) };

    let author = Author { nick: nick.to_string(), name: name.to_string(), email: email };
    partners_config.add_author(&author)?;

    Ok(())
}

fn current<'a>(_partners_config: &'a Config) -> Result<()> {
    println!("{}", Config::Local.current_author()?);

    Ok(())
}

fn filter_authors<'a>(authors: &'a [Author], nicks: &[&str]) -> std::result::Result<Vec<Author>, PartnersError> {
    nicks.iter().map(|nick| {
        match authors.iter().find(|a| &a.nick == nick) {
            Some(author) => Ok(author.clone()),
            None => Err(PartnersError::AuthorNotFound(format!("unable to find author with nick: {}", nick)))
        }
    }).collect()
}

fn set<'a>(partners_config: &'a Config, matches: &ArgMatches) -> Result<()> {
    let nicks: Vec<&str> = matches.values_of("nicks").unwrap().collect();

    let authors = partners_config.authors()?;

    let filtered_authors = filter_authors(&authors, &nicks)?;
    
    let selection = AuthorSelection::new(partners_config, &filtered_authors)?;

    Config::Global.set_current_author(&selection)?;
            
    println!("{}", Config::Local.current_author()?);

    Ok(())
}

fn run() -> Result<()> {
    let yaml = load_yaml!("cli.yml");
    let app = App::from_yaml(yaml).version(crate_version!()).author(crate_authors!());
    let matches = app.get_matches();

    let config_path = std::env::home_dir().ok_or(PartnersError::HomeDirectoryNotFound)?.join(".partners.cfg");

    let partners_config = Config::File(&config_path);

    match matches.subcommand() {
        ("list", Some(_sub_matches)) => list(&partners_config),
        ("current", Some(_sub_matches)) => current(&partners_config),
        ("set", Some(sub_matches)) => set(&partners_config, sub_matches),
        ("add", Some(sub_matches)) => add(&partners_config, sub_matches),
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
