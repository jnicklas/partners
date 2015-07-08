extern crate docopt;
extern crate git2;

use docopt::Docopt;
use std::rc::Rc;

pub type Result<T, E=Box<::std::error::Error+Send+Sync>> = ::std::result::Result<T, E>;

#[macro_use]
mod author;
mod pair;
mod config;
mod concat;

use author::Author;
use pair::Pair;
use config::Config;
use std::borrow::Cow;
use std::fs;

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
        println!("{}", item);
    }
}

fn get_authors(partners_config: &git2::Config, config: Rc<Config>) -> Result<Vec<Author>> {
    let entries = try!(partners_config.entries(Some("author.*.name")));
    entries.map(|entry| {
        let nick = entry.name().expect("does not contain nick").split('.').nth(1).unwrap().to_string();
        let name = entry.value().expect("does not contain name").to_string();
        let email = partners_config.get_string(&format!("author.{}.email", nick)).ok();

        Ok(Author { config: config.clone(), nick: nick, name: name, email: email })
    }).collect()
}

fn write_author(partners_config: &mut git2::Config, author: &Author) -> Result<()> {
    try!(partners_config.set_str(&format!("author.{}.name", author.nick), &author.name));
    if let Some(ref email) = author.email {
        try!(partners_config.set_str(&format!("author.{}.email", author.nick), email));
    }
    Ok(())
}

fn print_current(git_config: &mut git2::Config) -> Result<()> {
    let snapshot = try!(git_config.snapshot());
    if let Ok(nick) = snapshot.get_str("partners.current") {
        println!("{}:", nick);
    }
    if let Ok(name) = snapshot.get_str("user.name") {
        println!("  Name:  {}", name);
    }
    if let Ok(email) = snapshot.get_str("user.email") {
        println!("  Email: {}", email);
    }
    Ok(())
}

fn filter_authors<'a>(authors: &'a [Author], nicks: &[&str]) -> Result<Vec<&'a Author>, String> {
    nicks.iter().map(|n| authors.iter().find(|a| &a.nick == n).ok_or_else(|| n.to_string())).collect()
}

fn set_current<T>(git_config: &mut git2::Config, current: T) -> Result<()> where T: AuthorInformation {
    try!(git_config.set_str("partners.current", &current.get_nick()));
    try!(git_config.set_str("user.name", &current.get_name()));
    try!(git_config.set_str("user.email", &current.get_email()));
    Ok(())
}

fn main() {
    let partners_config_path = std::env::home_dir().expect("can't determine home directory").join(".partners.cfg");

    let repository = git2::Repository::discover(::std::env::current_dir().unwrap()).unwrap();

    let docopt = Docopt::new(USAGE).unwrap().help(true).version(Some(env!("CARGO_PKG_VERSION").to_string()));
    let args = docopt.parse().unwrap_or_else(|e| e.exit());

    let mut partners_config = git2::Config::open(&partners_config_path).unwrap();

    let mut git_config = repository.config().unwrap();

    let mut global_config = git2::Config::open_default().unwrap();

    let config = Rc::new(Config::from_git(&partners_config));

    if !fs::metadata(&partners_config_path).map(|x| x.is_file()).unwrap_or(false) {
        println!("Config file {:?} does not exist, please run `partners setup`", partners_config_path)
    } else if args.get_bool("list") {
        let authors = get_authors(&partners_config, config.clone()).unwrap();
        print_author_list(&authors);
    } else if args.get_bool("add") {
        let email = args.get_str("--email");
        let email = if email.is_empty() { None } else { Some(email) };
        let author = Author::new(&config, args.get_str("--nick"), args.get_str("--name"), email);
        write_author(&mut partners_config, &author).unwrap();
    } else if args.get_bool("current") {
        print_current(&mut git_config).ok().expect("cannot print current author");
    } else if args.get_bool("set") {
        let nicks = args.get_vec("<nick>");
        let authors = get_authors(&partners_config, config.clone()).unwrap();
        match filter_authors(&authors, &nicks) {
            Ok(filtered_authors) => {
                match filtered_authors.len() {
                    0 => println!("no author specified"),
                    1 => set_current(&mut global_config, filtered_authors[0]).unwrap(),
                    _ => {
                        let pair = Pair { config: config.clone(), authors: &filtered_authors };
                        set_current(&mut global_config, &pair).unwrap()
                    }
                }
                print_current(&mut git_config).ok().expect("cannot print current author");;
            },
            Err(nick) => {
                println!("couldn't find author '{}'", nick);
            }
        }
    }
}
