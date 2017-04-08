use git::Config;
use clap::ArgMatches;
use author::Author;
use Result;

pub fn add<'a>(partners_config: &'a Config, matches: &ArgMatches) -> Result<()> {
    let nick = matches.value_of("nick").unwrap();
    let name = matches.value_of("name").unwrap();
    let email = matches.value_of("email").unwrap();

    let email = if email.is_empty() { None } else { Some(email.to_string()) };

    let author = Author { nick: nick.to_string(), name: name.to_string(), email: email };
    partners_config.add_author(&author)?;

    Ok(())
}