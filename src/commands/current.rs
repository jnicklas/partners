use git::Config;
use clap::ArgMatches;
use Result;

pub fn current<'a>(_partners_config: &'a Config, _matches: &ArgMatches) -> Result<()> {
    println!("{}", Config::Local.current_author()?);

    Ok(())
}