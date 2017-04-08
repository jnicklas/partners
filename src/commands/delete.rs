use git::Config;
use clap::ArgMatches;
use Result;

pub fn delete<'a>(partners_config: &'a Config, matches: &ArgMatches) -> Result<()> {
    let nick = matches.value_of("nick").unwrap();

    partners_config.remove_author(nick)?;

    Ok(())
}