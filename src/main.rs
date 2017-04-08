#[macro_use] extern crate clap;
#[macro_use] extern crate derive_error;
extern crate termion;

use std::process;
use std::io::{self, Write};

#[macro_use]
mod git;
mod author;
mod author_selection;
mod error;
mod commands;
mod helpers;

use git::Config;
use clap::App;
use error::PartnersError;

pub type Result<T, E=PartnersError> = ::std::result::Result<T, E>;

fn run() -> Result<()> {
    let yaml = load_yaml!("cli.yml");
    let app = App::from_yaml(yaml).version(crate_version!()).author(crate_authors!());
    let matches = app.get_matches();

    let config_path = std::env::home_dir().ok_or(PartnersError::HomeDirectoryNotFound)?.join(".partners.cfg");

    if !config_path.exists() {
        println!("config file not found at {:?}", config_path);

        if helpers::confirm("do you want to create it?")? {
            helpers::create_config_file(&config_path)?;
        } else {
            Err(PartnersError::CannotProcede)?;
        }
    }

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
        Err(PartnersError::CannotProcede) => {
            process::exit(2);
        },
        Err(e) => {
            writeln!(&mut io::stderr(), "ERROR: {}", e).unwrap();
            process::exit(1);
        }
    }
}
