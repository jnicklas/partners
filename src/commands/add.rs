use git::Config;
use clap::ArgMatches;
use author::Author;
use helpers;
use Result;

fn get_required_value(matches: &ArgMatches, name: &str, prompt: &str) -> Result<String> {
    match matches.value_of(name) {
        Some(nick) => Ok(String::from(nick)),
        None => {
            loop {
                match helpers::query(prompt)? {
                    Some(value) => return Ok(value),
                    None => println!("ERROR: {} can't be blank", prompt),
                }
            }
        }
    }
}

fn get_optional_value(matches: &ArgMatches, name: &str, prompt: &str) -> Result<Option<String>> {
    match matches.value_of(name) {
        Some(nick) => Ok(Some(String::from(nick))),
        None => helpers::query(prompt),
    }
}


pub fn add<'a>(partners_config: &'a Config, matches: &ArgMatches) -> Result<()> {
    let nick = get_required_value(matches, "nick", "Nick")?;
    let name = get_required_value(matches, "name", "Name")?;
    let email = get_optional_value(matches, "email", "Email")?;

    let author = Author { nick: nick, name: name, email: email };
    partners_config.add_author(&author)?;

    Ok(())
}