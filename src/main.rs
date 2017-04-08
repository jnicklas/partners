#[macro_use] extern crate clap;
#[macro_use] extern crate derive_error;

use std::process;
use std::io::Write;
use std::io;

#[macro_use]
mod git;
mod author;
mod concat;
mod author_selection;
mod error;
mod commands;

use git::Config;
use clap::{App};
use error::PartnersError;

pub type Result<T, E=PartnersError> = ::std::result::Result<T, E>;

fn run() -> Result<()> {
    let yaml = load_yaml!("cli.yml");
    let app = App::from_yaml(yaml).version(crate_version!()).author(crate_authors!());
    let matches = app.get_matches();

    let config_path = std::env::home_dir().ok_or(PartnersError::HomeDirectoryNotFound)?.join(".partners.cfg");

    let partners_config = Config::File(&config_path);

    match matches.subcommand() {
        ("list", Some(sub_matches)) => commands::list(&partners_config, sub_matches),
        ("current", Some(sub_matches)) => commands::current(&partners_config, sub_matches),
        ("set", Some(sub_matches)) => commands::set(&partners_config, sub_matches),
        ("add", Some(sub_matches)) => commands::add(&partners_config, sub_matches),
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
