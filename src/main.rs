#[macro_use] extern crate clap;
extern crate termion;
extern crate xdg;
#[macro_use] extern crate failure;

use std::process;

#[macro_use]
mod git;
mod author;
mod author_selection;
mod commands;
mod helpers;

use clap::App;

pub type Result<T, E=::failure::Error> = ::std::result::Result<T, E>;

#[derive(Debug, Fail)]
#[fail(display = "cannot procede")]
pub struct CannotProcede;

fn run() -> Result<()> {
    let yaml = load_yaml!("cli.yml");
    let app = App::from_yaml(yaml).version(crate_version!()).author(crate_authors!());
    let matches = app.get_matches();

    let partners_config = commands::initial()?;

    match matches.subcommand() {
        ("list", Some(sub_matches)) => commands::list(&partners_config, sub_matches),
        ("current", Some(sub_matches)) => commands::current(&partners_config, sub_matches),
        ("set", Some(sub_matches)) => commands::set(&partners_config, sub_matches),
        ("add", Some(sub_matches)) => commands::add(&partners_config, sub_matches),
        ("delete", Some(sub_matches)) => commands::delete(&partners_config, sub_matches),
        ("setup", Some(sub_matches)) => commands::setup(&partners_config, sub_matches),
        _ => {
            println!("{}", matches.usage());
            Ok(())
        }
    }
}

fn main() {
    match run() {
        Ok(_) => {},
        Err(error) => {
            match error.downcast::<CannotProcede>() {
                Ok(_) => process::exit(2),
                Err(error) => {
                    eprintln!("ERROR: {}", error);
                    process::exit(1);
                }
            }
        }
    }
}
